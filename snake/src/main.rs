use bevy::prelude::*;

/// The screen height.
const SCREEN_HEIGHT: f32 = 480.;
/// The screen width.
const SCREEN_WIDTH: f32 = 640.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(window_resize_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // The camera
    commands.spawn_bundle(Camera2dBundle::default());
}

/// Resizes the window at startup.
fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(SCREEN_WIDTH, SCREEN_HEIGHT);
}
