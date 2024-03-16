use bevy::{math::vec3, prelude::*};

#[derive(Default)]
pub struct CameraPlugin {
    descriptor: CameraDescriptor,
}

#[derive(Resource, Clone)]
pub struct CameraDescriptor {
    spawn_position: Vec3,
    opt_postition: Vec3,
    camera_speed: f32,
    window: f32,
}

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.descriptor.clone())
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, control_camera);
    }
}

fn spawn_camera(mut cmd: Commands, descriptor: Res<CameraDescriptor>) {
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(descriptor.spawn_position)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));
}

fn control_camera(
    target_query: Query<&Transform, With<CameraTarget>>,
    mut cam_query: Query<&mut Transform, (With<MainCamera>, Without<CameraTarget>)>,
    time: Res<Time>,
    descriptor: Res<CameraDescriptor>,
) {
    let mut cam = cam_query.single_mut();
    let target = target_query.single();

    let opt_cam_pos = target.translation + descriptor.opt_postition;
    let diff = opt_cam_pos - cam.translation;
    let dir = vec3(diff.x, 0.0, diff.z).normalize_or_zero();
    let amount = (vec3(diff.x, 0.0, diff.z).length() / descriptor.window).clamp(0.0, 1.0);
    cam.translation += dir * time.delta_seconds() * descriptor.camera_speed * amount;
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        Self {
            spawn_position: vec3(0.0, 12.0, 12.0),
            opt_postition: vec3(0.0, 12.0, 12.0),
            camera_speed: 7.0,
            window: 5.0,
        }
    }
}
