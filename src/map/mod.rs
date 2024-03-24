use bevy::{
    math::vec3,
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};
use bevy_rapier3d::{
    dynamics::CoefficientCombineRule,
    geometry::{Collider, Friction},
};

use rand::seq::IteratorRandom;

#[derive(Resource, Default, DerefMut, Deref)]
pub struct WorldMap(HashMap<MapTileHandle, MapTile>);

pub struct MapTile {
    pub position: Vec3,
    pub constitution: f32,
    pub sinking: TileSinking,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct LeafTiles(HashSet<MapTileHandle>);

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MapTileHandle {
    x: i32,
    z: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileSinking {
    NotSinking,
    Sinking,
    Sunk,
}

#[derive(Resource)]
pub struct MapDescriptor {
    tile_size: f32,
    tile_height: f32,
    radius: u32,
    constitution_reduction_rate: f32,
}

impl Default for MapDescriptor {
    fn default() -> Self {
        Self {
            tile_size: 2.0,
            tile_height: 2.0,
            radius: 8,
            constitution_reduction_rate: 3.0,
        }
    }
}

pub struct MapPlugin;

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum MapState {
    #[default]
    CreateMap,
    SpawnMap,
    GamePlay,
    DespawnMap,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>()
            .init_resource::<LeafTiles>()
            .init_state::<MapState>()
            .insert_resource(MapDescriptor::default())
            .add_systems(
                OnEnter(MapState::CreateMap),
                (create_map, calc_leaf_tiles.after(create_map)),
            )
            .add_systems(OnEnter(MapState::SpawnMap), spawn_map)
            .add_systems(OnEnter(MapState::DespawnMap), despawn_map)
            .add_systems(
                Update,
                (
                    reduce_leaf_constitution,
                    update_leaf_tiles,
                    check_leaf_constitution,
                    sink_tile,
                )
                    .run_if(in_state(MapState::GamePlay)),
            );
    }
}

fn create_map(
    mut map: ResMut<WorldMap>,
    mut state: ResMut<NextState<MapState>>,
    des: Res<MapDescriptor>,
) {
    let radius = des.radius as i32;
    for x in (-radius)..radius {
        for z in (-radius)..radius {
            if x * x + z * z >= radius * radius {
                continue;
            }
            let xpos = x as f32 * des.tile_size;
            let zpos = z as f32 * des.tile_size;
            let tile = MapTile {
                position: vec3(xpos, 0.0, zpos),
                constitution: 1.0,
                sinking: TileSinking::NotSinking,
            };
            let handle = MapTileHandle { x, z };
            map.0.insert(handle, tile);
            state.set(MapState::SpawnMap);
        }
    }
}

fn calc_leaf_tiles(map: Res<WorldMap>, mut leafs: ResMut<LeafTiles>) {
    for &MapTileHandle { x, z } in map.keys() {
        let nx = MapTileHandle { x: x - 1, z };
        let px = MapTileHandle { x: x + 1, z };
        let nz = MapTileHandle { x, z: z - 1 };
        let pz = MapTileHandle { x, z: z + 1 };
        let contains_all = map.contains_key(&nx)
            && map.contains_key(&px)
            && map.contains_key(&nz)
            && map.contains_key(&pz);
        if contains_all {
            continue;
        }
        leafs.insert(MapTileHandle { x, z });
    }
}

fn spawn_map(
    mut cmd: Commands,
    map: Res<WorldMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    des: Res<MapDescriptor>,
    mut state: ResMut<NextState<MapState>>,
) {
    let mesh = meshes.add(Cuboid::default().mesh().scaled_by(vec3(
        des.tile_size,
        des.tile_height,
        des.tile_size,
    )));
    for (handle, tile) in map.iter() {
        let material = materials.add(Color::GREEN);
        cmd.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_translation(tile.position),
            ..default()
        })
        .insert(*handle)
        .insert(Collider::cuboid(
            des.tile_size / 2.0,
            des.tile_height / 2.0,
            des.tile_size / 2.0,
        ))
        .insert(Friction {
            coefficient: 0.9,
            combine_rule: CoefficientCombineRule::Average,
        });
    }
    state.set(MapState::GamePlay);
}

fn despawn_map() {}

// NOTE: select random leaf and reduce its constitution
fn reduce_leaf_constitution(
    mut map: ResMut<WorldMap>,
    leafs: Res<LeafTiles>,
    time: Res<Time>,
    des: Res<MapDescriptor>,
) {
    let Some(random_handle) = leafs.iter().choose(&mut rand::thread_rng()) else {
        error!("Unable to find Random Leaf in leafs: {:?}!", leafs.clone());
        return;
    };
    let Some(random_leaf) = map.get_mut(random_handle) else {
        error!("Unable to find a Leaf in Map!");
        return;
    };
    random_leaf.constitution -= des.constitution_reduction_rate * time.delta_seconds();
}

// NOTE: checks if leaf node has critical constitution, then randomly checks if it
fn check_leaf_constitution(mut query: Query<&MapTileHandle>, mut map: ResMut<WorldMap>) {
    for handle in query.iter_mut() {
        let Some(tile) = map.get_mut(handle) else {
            continue;
        };
        if tile.sinking != TileSinking::NotSinking {
            continue;
        }
        if tile.constitution >= 0.0 {
            continue;
        }
        tile.sinking = TileSinking::Sinking;
    }
}

// NOTE: Handle the logic for sinking a tile
fn sink_tile(
    mut query: Query<(&MapTileHandle, &mut Transform)>,
    mut map: ResMut<WorldMap>,
    time: Res<Time>,
) {
    for (handle, mut trans) in query.iter_mut() {
        let Some(tile) = map.get_mut(handle) else {
            continue;
        };
        if TileSinking::Sinking != tile.sinking {
            continue;
        }
        if trans.translation.y < -3.0 {
            tile.sinking = TileSinking::Sunk;
        }
        trans.translation.y -= time.delta_seconds();
    }
}

fn update_leaf_tiles(mut leafs: ResMut<LeafTiles>, map: Res<WorldMap>) {
    for &MapTileHandle { x, z } in map.keys() {
        let Some(tile) = map.get(&MapTileHandle { x, z }) else {
            continue;
        };

        if tile.sinking != TileSinking::Sunk {
            continue;
        }

        let neighs = [
            MapTileHandle { x: x - 1, z },
            MapTileHandle { x: x + 1, z },
            MapTileHandle { x, z: z - 1 },
            MapTileHandle { x, z: z + 1 },
        ];

        for n in neighs.iter() {
            let Some(tile) = map.get(n) else {
                continue;
            };
            if tile.sinking == TileSinking::NotSinking {
                leafs.insert(*n);
            }
        }
    }
}
