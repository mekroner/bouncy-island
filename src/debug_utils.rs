use bevy::prelude::*;

use crate::{Grounded, Player, PlayerActionState};


pub fn debug_material_color(
    query: Query<(&Handle<StandardMaterial>, &Grounded, &PlayerActionState), With<Player>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (handle, grounded, state) in query.iter() {
        let Some(material) = &mut materials.get_mut(handle) else {
            continue;
        };

        let color = match (grounded, state) {
            (Grounded::Grounded, PlayerActionState::Idle) => Color::WHITE,
            (Grounded::Airborne, PlayerActionState::Idle) => Color::BLACK,
            (Grounded::Grounded, PlayerActionState::PrepareAttack(_)) => Color::GOLD,
            (Grounded::Grounded, PlayerActionState::Attack(_, _)) => Color::RED,
            (Grounded::Grounded, PlayerActionState::Shield(_)) => Color::BLUE,
            (_, _) => Color::PINK,
        };
        material.base_color = color;
    }
}
