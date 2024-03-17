use std::time::Duration;

use bevy::{math::vec3, prelude::*};
use bevy_rapier3d::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Coin;

#[derive(Bundle)]
struct CoinPhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
}

impl Default for CoinPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(0.5),
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}

#[derive(Default)]
pub struct CoinSpawnerPlugin {
    descriptor: CoinSpawnerDescriptor,
}

#[derive(Resource, Clone)]
struct CoinSpawnerDescriptor {
    spawn_radius: u32,
    spawn_height: f32,
}

impl Default for CoinSpawnerDescriptor {
    fn default() -> Self {
        Self {
            spawn_radius: 16,
            spawn_height: 16.0,
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct CoinTimer(Timer);

#[derive(Component)]
struct Rotated(f32);

impl Default for CoinTimer {
    fn default() -> Self {
        Self(Timer::new(
            Duration::from_millis(1000),
            TimerMode::Repeating,
        ))
    }
}

impl Plugin for CoinSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.descriptor.clone())
            .insert_resource(CoinTimer::default())
            .add_systems(
                Update,
                (spawn_coins, update_rotated, tick_timer, despawn_coins),
            );
    }
}

fn spawn_coins(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    timer: Res<CoinTimer>,
    des: Res<CoinSpawnerDescriptor>,
) {
    if !timer.just_finished() {
        return;
    }
    let radius = des.spawn_radius as i32;
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-radius..=radius) as f32;
    let z = rng.gen_range(-radius..=radius) as f32;

    let mesh = meshes.add(Sphere::default());
    let material = materials.add(Color::GOLD);
    cmd.spawn(Coin)
        .insert(CoinPhysicsBundle::default())
        .insert(Rotated(1.0))
        .insert(Transform::from_xyz(x, des.spawn_height, z))
        .insert(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(x, des.spawn_height, z).with_scale(vec3(0.3, 0.5, 0.5)),
            ..default()
        });
}

fn update_rotated(time: Res<Time>, mut query: Query<(&mut Transform, &Rotated)>) {
    for (mut trans, rot_speed) in query.iter_mut() {
        trans.rotate_y(rot_speed.0 * time.delta_seconds());
    }
}

fn tick_timer(time: Res<Time>, mut timer: ResMut<CoinTimer>) {
    timer.tick(time.delta());
}

fn despawn_coins(mut cmd: Commands, query: Query<(Entity, &Transform), With<Coin>>) {
    for (e, trans) in query.iter() {
        if trans.translation.y >= -1.0 {
            continue;
        }
        cmd.entity(e).despawn();
    }
}
