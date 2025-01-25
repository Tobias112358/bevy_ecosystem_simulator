use bevy::prelude::*;

mod world_setup;
mod camera_setup;
mod rabbit;
mod frame_manager;
mod foliage;

fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((world_setup::plugin, camera_setup::plugin, rabbit::plugin, frame_manager::plugin, foliage::plugin))
        .run();

}
