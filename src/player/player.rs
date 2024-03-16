use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

#[derive(Bundle)]
pub struct PlayerPhysicsBundle {
    rigid_body: RigidBody,
    collider: Collider,
    mass_properties: ColliderMassProperties,
    friction: Friction,
    damping: Damping,
    active_events: ActiveEvents,
    external_force: ExternalForce,
    external_impulse: ExternalImpulse,
    locked_axes: LockedAxes,
}

impl Default for PlayerPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(0.5),
            mass_properties: ColliderMassProperties::Density(2.0),
            friction: Friction {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Average,
            },
            damping: Damping {
                linear_damping: 0.2,
                ..default()
            },
            active_events: ActiveEvents::COLLISION_EVENTS,
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    player_state: PlayerActionState,
    grounded: Grounded,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            player_state: PlayerActionState::Idle,
            grounded: Grounded::Airborne,
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Debug)]
pub enum PlayerActionState {
    Idle,
    PrepareAttack(Duration),
    Attack(Duration, f32), //secound value is attack strength
    Shield(Duration),
}

#[derive(Resource)]
pub struct PlayerActionValues {
    pub min_shield_duration: Duration,
    pub max_shield_duration: Duration,
    pub parry_duration: Duration,
    pub min_prepare_duration: Duration,
    pub max_prepare_duration: Duration,
}

#[derive(Component, Clone, Debug)]
pub enum Grounded {
    Grounded,
    Airborne,
}

impl Default for PlayerActionValues {
    fn default() -> Self {
        Self {
            min_shield_duration: Duration::from_millis(400),
            max_shield_duration: Duration::from_millis(2000),
            parry_duration: Duration::from_millis(400),
            min_prepare_duration: Duration::from_millis(400),
            max_prepare_duration: Duration::from_millis(3000),
        }
    }
}

impl PlayerActionValues {
    pub fn attack_strength(&self, dur: Duration) -> f32 {
        if dur < self.min_prepare_duration {
            return 0.0;
        }
        if dur > self.max_prepare_duration {
            return 1.0;
        }
        let d = (dur - self.min_prepare_duration).as_secs_f32();
        let range = (self.max_prepare_duration - self.min_shield_duration).as_secs_f32();
        d / range
    }

    pub fn move_multipier(&self, state: &PlayerActionState) -> f32 {
        match state {
            PlayerActionState::Idle => 24.0,
            PlayerActionState::PrepareAttack(_) => 12.0,
            PlayerActionState::Attack(_, _) => 32.0,
            PlayerActionState::Shield(_) => 12.0,
        }
    }
}
