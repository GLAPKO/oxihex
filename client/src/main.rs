use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My Bevy Game".into(),
                // Provide the canvas selector for WASM
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#game-canvas".into()),
                // Tells Bevy to resize the output to match the browser element
                #[cfg(target_arch = "wasm32")]
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
