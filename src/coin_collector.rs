use crate::Player;

use super::coin::Coin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Default)]
pub struct CoinCollection {
    pub num: u32,
}

pub fn collect_coins(
    mut cmd: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<&mut CoinCollection, With<Player>>,
    coin_query: Query<Entity, With<Coin>>,
) {
    for collision_event in collision_events.read() {
        let CollisionEvent::Started(e0, e1, _) = collision_event else {
            continue;
        };
        if let Ok(_) = coin_query.get(*e0) {
            if let Ok(mut collection) = query.get_mut(*e1) {
                collection.num += 1;
                cmd.entity(*e0).despawn();
            }
        };
        if let Ok(_) = coin_query.get(*e1) {
            if let Ok(mut collection) = query.get_mut(*e0) {
                collection.num += 1;
                cmd.entity(*e1).despawn();
            }
        };
    }
}

pub fn debug_log_coin_collection(query: Query<&CoinCollection, With<Player>>) {
    for collection in query.iter() {
        info!("Player has collected {} Coins", collection.num);
    }
}
