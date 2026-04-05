use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
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
    }));

    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}