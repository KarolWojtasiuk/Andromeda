use super::input::GameplayInput;
use crate::engine::prelude::*;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera);
    }
}

#[derive(Component, Default, Debug)]
#[require(Transform, Camera2d)]
pub struct GameCamera;

#[derive(Component, Default, Debug)]
#[require(Transform)]
pub struct GameCameraTarget;

fn update_camera(
    mut camera: Single<
        (&mut Transform, &mut OrthographicProjection),
        (With<GameCamera>, Without<GameCameraTarget>),
    >,
    target: Single<&Transform, (With<GameCameraTarget>, Without<GameCamera>)>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    camera.0.translation = camera
        .0
        .translation
        .interpolate_stable(&target.translation, time.delta_secs() * 5.0);

    if input.zoom != 0.0 {
        camera.1.scale = (camera.1.scale.ln() - input.zoom * 0.2)
            .exp()
            .clamp(0.2, 2.0);
    }
}
