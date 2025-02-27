use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameplayInput>();
        app.init_resource::<GameplayInput>();
        app.add_systems(Update, update_gameplay_input);
    }
}

#[derive(Resource, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct GameplayInput {
    pub movement: Vec2,
    pub sprint: bool,
    pub zoom: f32,
    pub toggle_inventory: bool,
}

fn update_gameplay_input(
    mut input: ResMut<GameplayInput>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
) {
    input.movement = {
        let mut movement = Vec2::ZERO;
        if keyboard.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        };
        if keyboard.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        };
        if keyboard.pressed(KeyCode::KeyW) {
            movement.y += 1.0;
        };
        if keyboard.pressed(KeyCode::KeyS) {
            movement.y -= 1.0;
        };

        movement.normalize_or_zero()
    };

    input.sprint = keyboard.pressed(KeyCode::ShiftLeft);
    input.zoom = mouse_scroll.delta.y;
    input.toggle_inventory = keyboard.just_pressed(KeyCode::Tab);
}
