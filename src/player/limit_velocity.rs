use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{Player, PlayerActionState};

fn horizontal_velocity(linvel: Vec3) -> f32 {
    let mut h = linvel;
    h.y = 0.0;
    h.length()
}

fn limit_horizontal_velocity(linvel: Vec3, maxvel: f32) -> Vec3 {
    if horizontal_velocity(linvel) <= maxvel {
        return linvel;
    }
    let result = linvel.normalize_or_zero() * maxvel;
    Vec3::new(result.x, linvel.y, result.z)
}

//FIXME: This sucks! We shoud do this by introducing more damping after a max velocity threshold is reached
fn limit_max_velocity(mut query: Query<(&PlayerActionState, &mut Velocity,), With<Player>>) {
    for (state, mut velocity) in query.iter_mut() {
        info!("Velocity: {:?}", horizontal_velocity(velocity.linvel));
        match state {
            PlayerActionState::Idle => {
                velocity.linvel = limit_horizontal_velocity(velocity.linvel, 5.0)
            }
            PlayerActionState::PrepareAttack(_) => {
                velocity.linvel = limit_horizontal_velocity(velocity.linvel, 2.0)
            }
            PlayerActionState::Attack(_, _) => {
                velocity.linvel = limit_horizontal_velocity(velocity.linvel, 20.0)
            }
            PlayerActionState::Shield(_) => {
                velocity.linvel = limit_horizontal_velocity(velocity.linvel, 2.0)
            }
        }
    }
}
