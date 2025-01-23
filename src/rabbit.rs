use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::world_setup::{WorldMapDataSetEvent, WorldMap, Voxel};

pub(super) fn plugin(app: &mut App) {
    app
        .insert_resource(AnimalFrameControl {
            timer: Timer::new(Duration::from_millis(30), TimerMode::Repeating),
        })
        .add_systems(Update, (spawn_initial_rabbits, rabbit_movement));
}

#[derive(Component)]
pub struct Rabbit {
    pub id: u32,
    pub location: (i32, i32),
}

#[derive(Resource)]
pub struct AnimalFrameControl {
    pub timer: Timer,
}

const INITIAL_RABBIT_POPULATION: u32 = 3;

pub fn spawn_initial_rabbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<WorldMapDataSetEvent>,
    mut world_map_query: Query<&WorldMap>,
    mut voxel_query: Query<&Voxel>,
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
                        x = rng.gen_range(0..world_map.width);
                        z = rng.gen_range(0..world_map.height);

                        println!("{}, {}", x, z);

                        let Ok(current_voxel) = voxel_query.get(world_map.map[x as usize][z as usize]) else {
                            continue;
                        };

                        if current_voxel == &Voxel::GrassVoxel {
                            break;
                        }
                    }
            
            
            
                    commands.spawn((
                        Rabbit {
                            id: i,
                            location: (x, z),
                        },
                        Mesh3d(meshes.add(Cuboid {
                            half_size: Vec3::new(0.5, 0.5, 0.5),
                            ..default()
                        })),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::linear_rgb(0.625, 0.32, 0.1666),
                            ..default()
                        })),
                        Transform::from_translation(Vec3::new((x - 30) as f32, 0.25, (z - 30) as f32)),
                        
                    ));
                }
            }
        }
    }

    // commands.insert_resource(AnimalFrameControl {
    //     // create the repeating timer
    //     timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
    // })

    
}


fn rabbit_movement(
    time: Res<Time>,
    mut frame_control: ResMut<AnimalFrameControl>,
    mut rabbit_query: Query<(&mut Rabbit, &mut Transform), With<Rabbit>>,
    voxel_query: Query<&Voxel>,
    world_map_query: Query<&WorldMap>,
) {

    frame_control.timer.tick(time.delta());
    if frame_control.timer.finished() {
        

        let world_map = world_map_query.single() else {
            panic!("Cannot find the world map!");
        };
        for (mut rabbit, mut transform) in rabbit_query.iter_mut() {

            let x_range = match rabbit.location.0 {
                0 => [rabbit.location.0, (rabbit.location.0 + 1)],
                59 => [(rabbit.location.0 - 1),(rabbit.location.0)],
                _ => [(rabbit.location.0 - 1),(rabbit.location.0 + 1)]
            };

            let z_range = match rabbit.location.1 {
                0 => [(rabbit.location.1),(rabbit.location.1 + 1)],
                59 => [(rabbit.location.1 - 1),(rabbit.location.1)],
                _ => [(rabbit.location.1 - 1),(rabbit.location.1 + 1)]
            };

            let mut possible_moves: Vec<(i32, i32)> = Vec::new();

            for x in x_range {
                for z in z_range.clone() {

                    if let Ok(current_voxel) = voxel_query.get(world_map.map[x as usize][z as usize]) {
                        if current_voxel == &Voxel::GrassVoxel || current_voxel == &Voxel::SandVoxel {
                            possible_moves.push((x, z));
                        }
                    }
                }
            }


            if possible_moves.len() > 0 {
                let mut rng = rand::thread_rng();
                let random_index = rng.gen_range(0..possible_moves.len());
                rabbit.location = possible_moves[random_index];
                transform.translation = Vec3::new((rabbit.location.0 - 30) as f32, 0.25, (rabbit.location.1 - 30) as f32);
            }
        }
    }
}