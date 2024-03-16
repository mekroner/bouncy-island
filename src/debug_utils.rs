use bevy::prelude::*;

use crate::{
    map::{MapTileHandle, WorldMap},
    Grounded, Player, PlayerActionState,
};

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

pub fn debug_map_material_color(
    query: Query<(&Handle<StandardMaterial>, &MapTileHandle)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<WorldMap>,
) {
    for (mat_handle, map_handle) in query.iter() {
        let Some(material) = &mut materials.get_mut(mat_handle) else {
            continue;
        };
        let Some(tile) = map.get(map_handle) else {
            continue;
        };
        let color = Color::rgb(0.0, tile.constitution, 0.0);
        material.base_color = color;
    }
}
