use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use crate::rabbit::Rabbit;


mod world_setup;
mod camera_setup;
mod rabbit;
mod frame_manager;
mod foliage;

fn main() {
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins((world_setup::plugin, camera_setup::plugin, rabbit::plugin, frame_manager::plugin, foliage::plugin))
        .add_systems(Update, ui_example_system)
        .run();

}

fn ui_example_system(
    mut contexts: EguiContexts,
    entity_query: Query<(Entity, &Rabbit), With<Rabbit>>,
) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        entity_query.iter().for_each(|(entity, rabbit)| {
            ui.label(format!("{:?}, {:?}", entity, rabbit.location));
            for (partner_entity, partner) in rabbit.partner_in_range.iter() {
                ui.label(format!("    {:?}", partner_entity));
            }
        });
    });
}
