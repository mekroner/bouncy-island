#import bevy_pbr::{
    mesh_functions,
    forward_io::{Vertex, VertexOutput},
    view_transformations::position_world_to_clip,
}

#import bevy_pbr::mesh_view_bindings::globals

@vertex
fn vertex(vert_in: Vertex) -> VertexOutput {
    var vertex = vert_in;
    var out: VertexOutput;

    let w = sin(globals.time);
    vertex.position.y += w * 0.3;


    // normal
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);

    // vertex position
    var model = mesh_functions::get_model_matrix(vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);

    //wave
    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    //let time = globals.time;
    return vec4(0.0, 1.0, 1.0, 1.0);
}
