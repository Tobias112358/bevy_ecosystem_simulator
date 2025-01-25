use bevy::prelude::*;

use crate::{
    world_setup::{Voxel, VoxelType,  VoxelsSpawnedEvent},
    frame_manager::FrameControl
};


#[derive(Component, Clone)]
pub struct Foliage {
    pub location: (i32, i32),
    pub consumed: bool,
    pub regen_counter: u32,
}

#[derive(Event)]
pub struct FoliageConsumedEvent(pub Entity);

pub(super) fn plugin(app: &mut App) {
    app
        .add_event::<FoliageConsumedEvent>()
        .add_systems(Update, (initial_foliage_spawn, consume_foliage, regenerate_foliage));
}

fn initial_foliage_spawn(
    mut commands: Commands,
    mut events: EventReader<VoxelsSpawnedEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut voxel_query: Query<(Entity, &Transform, &Voxel), With<Voxel>>,
) {

    for active_event in events.read() {
        match active_event {
            VoxelsSpawnedEvent(_world_voxels) => {
                
                println!("Event received!");

                let foliage = Mesh3d(meshes.add(Cuboid {
                    half_size: Vec3::new(0.1, 0.3, 0.1),
                    ..default()
                }));

                let foliage_mat = MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::linear_rgb(0.1, 0.9,0.4),
                    ..default()
                }));

                for (entity, transform, voxel) in voxel_query.iter_mut() {
                    if voxel.voxel_type != VoxelType::GrassVoxel {
                        continue;
                    }
                    if rand::random::<f32>() < 0.1 {
                        spawn_single_foliage(entity, &transform.translation, &mut commands, foliage.clone(), foliage_mat.clone());
                    }
                }
            }
        }
    }
}

fn spawn_single_foliage(
    entity: Entity,
    position: &Vec3,
    commands: &mut Commands,
    foliage: Mesh3d,
    foliage_mat: MeshMaterial3d<StandardMaterial>,
) {
    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            Foliage {
                location: ((position.x as i32) + 30, (position.z as i32) + 30),
                consumed: false,
                regen_counter: 0,
            },
            foliage,
            foliage_mat,
            Visibility::Visible,
            Transform::from_translation(Vec3::new(0.0, 0.75, 0.0)),
        ));
    });
}


fn consume_foliage(
    mut foliage_query: Query<(&mut Foliage, &mut Visibility)>,
    mut consumed_foliage_event: EventReader<FoliageConsumedEvent>,
) {
    for active_event in consumed_foliage_event.read() {
        match active_event {
            FoliageConsumedEvent(entity) => {
                let Ok((mut foliage, mut visibility)) = foliage_query.get_mut(*entity) else {
                    continue;
                };
                foliage.consumed = true;
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn regenerate_foliage(
    frame_control: Res<FrameControl>,
    mut foliage_query: Query<(&mut Foliage, &mut Visibility)>,
) {
    if frame_control.timer.finished() {
        for (mut foliage, mut visibility) in foliage_query.iter_mut() {
            if foliage.consumed && foliage.regen_counter > 20 {
                foliage.consumed = false;
                *visibility = Visibility::Visible;
                foliage.regen_counter = 0;
            } else if foliage.consumed {
                foliage.regen_counter += 1;
            }
        }
    }
}