use bevy::prelude::*;
use smart_default::SmartDefault;

use super::input::GameplayInput;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameCamera>();
        app.register_type::<GameCameraTarget>();
        app.add_systems(Update, update_camera);
    }
}

#[derive(Component, SmartDefault, Reflect, Debug)]
#[require(Transform, Camera3d)]
pub struct GameCamera {
    pub offset: Vec3,
    #[default(Dir3::from_xyz(0.0, 2.5, 1.0).unwrap())]
    pub direction: Dir3,
    #[default(20.0)]
    pub distance: f32,
    #[default(5.0)]
    pub min_distance: f32,
    #[default(50.0)]
    pub max_distance: f32,
    #[default(5.0)]
    pub smooth_rate: f32,
}

#[derive(Component, Default, Clone, Reflect, Debug)]
#[require(Transform)]
pub struct GameCameraTarget;

fn update_camera(
    mut camera: Single<(&mut Transform, &mut GameCamera), Without<GameCameraTarget>>,
    target: Single<&Transform, (With<GameCameraTarget>, Without<GameCamera>)>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    let (ref mut camera_transform, ref mut camera) = *camera;

    if input.zoom != 0.0 {
        camera.distance =
            (camera.distance - input.zoom).clamp(camera.min_distance, camera.max_distance);
    }

    camera_transform.translation.smooth_nudge(
        &(target.translation + camera.offset + camera.direction * camera.distance),
        camera.smooth_rate,
        time.delta_secs(),
    );

    let camera_target = camera_transform.translation - *camera.direction;
    camera_transform.look_at(camera_target, Vec3::Y);
}
