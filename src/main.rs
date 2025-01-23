use bevy::prelude::*;

mod world_setup;
mod camera_setup;

fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((world_setup::plugin, camera_setup::plugin))
        .run();

}
