use bevy::prelude::*;


fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut camera  = Camera2dBundle::new_with_far(5.0);
    commands.spawn(camera);
}   

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
