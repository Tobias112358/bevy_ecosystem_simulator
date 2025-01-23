use bevy::prelude::*;
use rand::Rng;

use crate::world_setup::{WorldMapDataSetEvent, WorldMap, GrassVoxel};

pub(super) fn plugin(app: &mut App) {
    app
        .add_systems(Update, spawn_initial_rabbits);
}

#[derive(Component)]
pub struct Rabbit(pub u32);

const INITIAL_RABBIT_POPULATION: u32 = 10;

pub fn spawn_initial_rabbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<WorldMapDataSetEvent>,
    mut world_map_query: Query<&WorldMap>,
    mut grass_voxel_query: Query<&GrassVoxel>,
) {
    for active_event in events.read() {
        match active_event {
            WorldMapDataSetEvent => {
                let world_map = world_map_query.single();
                for i in 0..INITIAL_RABBIT_POPULATION {

                    let mut rng = rand::thread_rng();
            
                    let mut x: i32 = rng.gen_range(-30..30);
                    let mut z: i32 = rng.gen_range(-30..30);

                    while true {
                        x = rng.gen_range(-30..30);
                        z = rng.gen_range(-30..30);

                        let Ok(current_voxel) = grass_voxel_query.get(world_map.map[(x+30) as usize][(z+30) as usize]) else {
                            continue;
                        };
                        break;
                    }
            
            
            
                    commands.spawn((
                        Rabbit(i),
                        Mesh3d(meshes.add(Cuboid {
                            half_size: Vec3::new(0.5, 0.5, 0.5),
                            ..default()
                        })),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::linear_rgb(0.625, 0.32, 0.1666),
                            ..default()
                        })),
                        Transform::from_translation(Vec3::new(x as f32, 0.25, z as f32)),
                        
                    ));
                }
            }
        }
    }
    
}