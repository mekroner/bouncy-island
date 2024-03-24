use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, NotShadowCaster},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::AsBindGroup,
    },
};

#[derive(Resource, Clone)]
pub struct WaterPluginDescriptor {
    size: f32,
    sub_divisions: u32,
}

impl Default for WaterPluginDescriptor {
    fn default() -> Self {
        Self {
            size: 50.0,
            sub_divisions: 10,
        }
    }
}

#[derive(Default)]
pub struct WaterPlugin {
    descriptor: WaterPluginDescriptor,
}

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.descriptor.clone())
            .add_plugins(MaterialPlugin::<WaterMaterial>::default())
            .add_systems(Startup, spawn_water);
    }
}

fn spawn_water(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    des: Res<WaterPluginDescriptor>,
) {
    let mesh = meshes.add(create_plane(des.size, des.sub_divisions));
    let material = materials.add(WaterMaterial {
        base: StandardMaterial {
            base_color: Color::ALICE_BLUE,
            ..default()
        },
        extension: WaterExtension { ..default() },
    });
    cmd.spawn(MaterialMeshBundle {
        mesh,
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(NotShadowCaster);
}

fn create_plane(size: f32, sub_div: u32) -> Mesh {
    let width = 2 + sub_div;
    let depth = 2 + sub_div;

    let mut vertex_positions: Vec<[f32; 3]> = vec![];
    let mut normals: Vec<[f32; 3]> = vec![];
    let mut indices: Vec<u32> = vec![];
    for x in 0..width {
        for z in 0..depth {
            let position = [
                size / width as f32 * x as f32 - size / 2.0,
                0.0,
                size / depth as f32 * z as f32 - size / 2.0,
            ];
            vertex_positions.push(position);
        }
    }
    for _ in 0..width {
        for _ in 0..depth {
            let normal = [0.0, 1.0, 0.0];
            normals.push(normal);
        }
    }

    for x in 0..(width - 1) {
        for z in 0..(depth - 1) {
            let index = z * depth + x;
            //first triangle
            indices.push(index);
            indices.push(index + 1);
            indices.push(index + depth);

            //secound triangle
            indices.push(index + 1);
            indices.push(index + depth + 1);
            indices.push(index + depth);
        }
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}

type WaterMaterial = ExtendedMaterial<StandardMaterial, WaterExtension>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
struct WaterExtension {}

#[cfg(not(feature = "embed_shaders"))]
fn water_vertex_shader() -> bevy::render::render_resource::ShaderRef {
    "shaders/water.wgsl".into()
}

impl MaterialExtension for WaterExtension {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        water_vertex_shader()
    }

    // fn deferred_vertex_shader() -> bevy::render::render_resource::ShaderRef {
    //     "shaders/water.wgsl".into()
    // }

    // fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
    //     "shaders/water.wgsl".into()
    // }
}
