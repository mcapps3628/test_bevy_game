use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movePlayer)
        .run();
}

fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn(Camera2d);
}

//Add Objects

fn movePlayer() {
    
}
