use std::f32::consts::PI;
use std::time::Duration;

mod camera;
mod coin;
mod coin_collector;
mod debug_utils;
mod key_bindings;
mod map;
mod player;
mod ui;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

use camera::CameraTarget;
use coin_collector::{collect_coins, debug_log_coin_collection, CoinCollection};
use debug_utils::*;
use key_bindings::KeyBindings;
use player::player::*;

fn main() {
    App::new()
        .init_resource::<KeyBindings>()
        .init_resource::<PlayerActionValues>()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((
            camera::CameraPlugin::default(),
            map::MapPlugin,
            coin::CoinSpawnerPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec3::new(0.0, -19.62, 0.0),
            ..default()
        })
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_move,
                check_player_grounded,
                player_action_system,
                player_collision_system,
            ),
        )
        .add_systems(Update, collect_coins)
        .add_systems(Update, check_game_over)
        .add_systems(
            Update,
            (
                debug_material_color,
                debug_map_material_color,
                debug_log_coin_collection,
            ),
        )
        .run();
}

fn setup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 4.0)),
        ..default()
    });

    // Water
    cmd.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::ALICE_BLUE,
            specular_transmission: 0.9,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        PlayerPhysicsBundle::default(),
        PlayerBundle::default(),
        CameraTarget,
    ))
    .insert(CoinCollection { num: 0 });

    // spawn practice target
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(3.0, 2.0, 0.0),
            ..default()
        },
        PlayerPhysicsBundle::default(),
        PlayerBundle::default(),
    ))
    .insert(PracticeBox::DoNothing);
}

#[derive(Component)]
enum PracticeBox {
    DoNothing,
}

fn player_move(
    mut query: Query<
        (
            &mut ExternalForce,
            &mut ExternalImpulse,
            &Grounded,
            &PlayerActionState,
        ),
        (With<Player>, Without<PracticeBox>),
    >,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    values: Res<PlayerActionValues>,
) {
    let mut force = Vec3::ZERO;
    let forward = -Vec3::Z;
    let right = Vec3::X;
    for &key in keys.get_pressed() {
        if key == key_bindings.move_forward {
            force += forward;
        }
        if key == key_bindings.move_backward {
            force -= forward;
        }
        if key == key_bindings.move_right {
            force += right;
        }
        if key == key_bindings.move_left {
            force -= right;
        }
    }

    force = force.normalize_or_zero();
    let mut jump_impulse = Vec3::ZERO;

    if keys.just_pressed(key_bindings.move_jump) {
        jump_impulse = Vec3::new(0., 8.0, 0.0);
    }

    for (mut ext_force, mut ext_impulse, grounded, state) in query.iter_mut() {
        match grounded {
            Grounded::Grounded => {
                ext_force.force = force * values.move_multipier(state);
                ext_impulse.impulse += jump_impulse;
            }
            Grounded::Airborne => {
                ext_force.force = Vec3::ZERO;
            }
        }
    }
}

fn check_player_grounded(
    mut query: Query<(&mut Grounded, &Transform, Entity), With<Player>>,
    rapier: Res<RapierContext>,
) {
    let grounding_threshold = 0.55;
    for (mut grounded, trans, entity) in query.iter_mut() {
        *grounded = Grounded::Airborne;
        let ray_pos = trans.translation;
        let ray_dir = -Vec3::Y;
        let Some((_entity, toi)) = rapier.cast_ray(
            ray_pos,
            ray_dir,
            bevy_rapier3d::prelude::Real::MAX,
            false,
            QueryFilter::new().exclude_collider(entity),
        ) else {
            continue;
        };
        let dist = (ray_dir * toi).length();
        if dist < grounding_threshold {
            *grounded = Grounded::Grounded;
        }
    }
}

fn player_action_system(
    mut query: Query<
        (
            &mut PlayerActionState,
            &Grounded,
            &ExternalForce,
            &mut ExternalImpulse,
        ),
        (With<Player>, Without<PracticeBox>),
    >,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    time: Res<Time>,
    values: Res<PlayerActionValues>,
) {
    for (mut state, grounded, force, mut impulse) in query.iter_mut() {
        let temp_state = state.clone();
        match (grounded, temp_state) {
            (Grounded::Grounded, PlayerActionState::Idle) => {
                if keys.just_pressed(key_bindings.attack) {
                    *state = PlayerActionState::PrepareAttack(Duration::from_secs(0))
                }
                if keys.just_pressed(key_bindings.shield) {
                    *state = PlayerActionState::Shield(Duration::from_secs(0))
                }
            }
            (Grounded::Grounded, PlayerActionState::PrepareAttack(dur)) => {
                let new_dur = time.delta() + dur;
                if keys.pressed(key_bindings.attack) && new_dur < values.max_prepare_duration {
                    *state = PlayerActionState::PrepareAttack(new_dur);
                    continue;
                }
                let strength = values.attack_strength(new_dur);
                *state = PlayerActionState::Attack(Duration::from_secs(0), strength);
                let direction = force.force.normalize_or_zero();
                impulse.impulse += direction * 30.0 * strength;
                info!(
                    "Attack with strength {} => impulse {}.",
                    strength, impulse.impulse
                );
            }
            (Grounded::Grounded, PlayerActionState::Attack(dur, strength)) => {
                let new_dur = time.delta() + dur;
                if new_dur > Duration::from_millis(400) {
                    *state = PlayerActionState::Idle;
                    continue;
                }
                *state = PlayerActionState::Attack(new_dur, strength);
            }
            (Grounded::Grounded, PlayerActionState::Shield(dur)) => {
                let new_dur = time.delta() + dur;
                if keys.pressed(key_bindings.shield) || new_dur < values.min_shield_duration {
                    *state = PlayerActionState::Shield(new_dur);
                    continue;
                }
                *state = PlayerActionState::Idle;
            }
            (Grounded::Airborne, _) => {
                // info!("ActionState changes for airborne not jet implemented!")
            }
        }
    }
}

// FIXME: Apply Airborne penalty
fn player_collision_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<
        (
            &mut ExternalImpulse,
            &PlayerActionState,
            &Grounded,
            &Transform,
        ),
        With<Player>,
    >,
) {
    use Grounded as G;
    use PlayerActionState as PAS;
    for collision_event in collision_events.read() {
        println!("Received collision event: {:?}", collision_event);
        let CollisionEvent::Started(e0, e1, _) = collision_event else {
            continue;
        };
        let (imp0, imp1) = {
            let Ok((_, pas0, _, trans0)) = query.get(*e0) else {
                continue;
            };
            let Ok((_, pas1, _, trans1)) = query.get(*e1) else {
                continue;
            };
            let dir0 = trans0.translation - trans1.translation;
            let dir1 = trans1.translation - trans0.translation;

            match (pas0, pas1) {
                (PAS::Idle, PAS::Idle) => {
                    info!("Entity {:?} has touched Entity {:?} ", e1, e0);
                    (12.0 * dir0, 12.0 * dir1)
                }
                (PAS::Idle, PAS::Attack(_, _)) => {
                    info!("Entity {:?} has attacked Entity {:?} ", e1, e0);
                    (24.0 * dir0, 3.0 * dir1)
                }
                (PAS::Attack(_, _), PAS::Idle) => {
                    info!("Entity {:?} has attacked Entity {:?} ", e1, e0);
                    (3.0 * dir0, 24.0 * dir1)
                }
                (PAS::Attack(_, _), PAS::Attack(_, _)) => {
                    info!("Entity {:?} has attacked Entity {:?} ", e1, e0);
                    (2.0 * dir0, 24.0 * dir1)
                }
                (PAS::Shield(_), PAS::Attack(_, _)) => {
                    info!("Entity {:?} has attacked Entity {:?} ", e1, e0);
                    unimplemented!();
                }
                (PAS::Attack(_, _), PAS::Shield(_)) => {
                    info!("Entity {:?} has attacked Entity {:?} ", e1, e0);
                    unimplemented!();
                }
                (_, _) => (Vec3::ZERO, Vec3::ZERO),
            }
        };
        query.get_mut(*e0).unwrap().0.impulse += imp0;
        query.get_mut(*e1).unwrap().0.impulse += imp1;
    }
}

// FIXME: This should reset the game and not just print a message
fn check_game_over(query: Query<&Transform, With<Player>>) {
    for trans in query.iter() {
        if trans.translation.y > 0.0 {
            continue;
        }
        info!("Game Over!",)
    }
}
