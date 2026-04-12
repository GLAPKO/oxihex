use bevy::prelude::*;

fn main () -> AppExit {
    App::new().add_plugins(MinimalPlugins).add_systems(Startup, startup).run()
}

fn startup () {
    println!("zapinam se");
}