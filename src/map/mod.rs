use bevy::{math::vec3, prelude::*, utils::hashbrown::HashMap};
use bevy_rapier3d::{
    dynamics::CoefficientCombineRule,
    geometry::{Collider, Friction},
};

#[derive(Resource, Default, DerefMut, Deref)]
pub struct WorldMap(HashMap<MapTileHandle, MapTile>);

pub struct MapTile {
    position: Vec3,
}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MapTileHandle {
    x: i32,
    z: i32,
}

pub struct MapPlugin;

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum MapState {
    #[default]
    CreateMap,
    SpawnMap,
    Idle,
    DespawnMap,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>()
            .init_state::<MapState>()
            .add_systems(OnEnter(MapState::CreateMap), create_map)
            .add_systems(OnEnter(MapState::SpawnMap), spawn_map)
            .add_systems(OnEnter(MapState::DespawnMap), despawn_map);
    }
}

fn create_map(mut map: ResMut<WorldMap>, mut state: ResMut<NextState<MapState>>) {
    let radius = 7;
    for x in (-radius)..radius {
        for z in (-radius)..radius {
            if x*x + z*z >= radius*radius {
                continue;
            }
            let tile = MapTile {
                position: vec3(x as f32* 2.0, 0.0, z as f32*2.0),
            };
            let handle = MapTileHandle { x, z };
            map.0.insert(handle, tile);
            state.set(MapState::SpawnMap);
        }
    }
}

fn spawn_map(
    mut cmd: Commands,
    map: Res<WorldMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tile_size = 2.0;
    let tile_height = 1.0;
    let mesh = meshes.add(Cuboid::default().mesh().scaled_by(vec3(
        tile_size,
        tile_height,
        tile_size,
    )));
    let material = materials.add(Color::GREEN);
    for (handle, tile) in map.iter() {
        cmd.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_translation(tile.position),
            ..default()
        })
        .insert(*handle)
        .insert(Collider::cuboid(
            tile_size / 2.0,
            tile_height / 2.0,
            tile_size / 2.0,
        ))
        .insert(Friction {
            coefficient: 0.9,
            combine_rule: CoefficientCombineRule::Average,
        });
    }
}
fn despawn_map() {}
