use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::world_setup::{Foliage, Voxel, WorldMap, WorldMapDataSetEvent};

pub(super) fn plugin(app: &mut App) {
    app
        .insert_resource(AnimalFrameControl {
            timer: Timer::new(Duration::from_millis(90), TimerMode::Repeating),
        })
        .add_systems(Update, (spawn_initial_rabbits, rabbit_movement, update_rabbit_nearby_resources, debug_rabbit_info));
}

#[derive(Component, Clone)]
pub struct Rabbit {
    pub id: u32,
    pub hunger: u32,
    pub thirst: u32,
    pub location: (i32, i32),
    pub plants_in_range: Vec<Entity>,
    pub water_in_range: Vec<Voxel>,
    pub sight_distance: u32,
}

#[derive(Resource)]
pub struct AnimalFrameControl {
    pub timer: Timer,
}

const INITIAL_RABBIT_POPULATION: u32 = 10;

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
                            hunger: 50,
                            thirst: 50,
                            location: (x, z),
                            plants_in_range: Vec::new(),
                            water_in_range: Vec::new(),
                            sight_distance: 3,
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
    mut commands: Commands,
    time: Res<Time>,
    mut frame_control: ResMut<AnimalFrameControl>,
    mut rabbit_query: Query<(Entity, &mut Rabbit, &mut Transform), With<Rabbit>>,
    voxel_query: Query<&Voxel>,
    foliage_query: Query<&Foliage>,
    world_map_query: Query<&WorldMap>,
) {

    frame_control.timer.tick(time.delta());
    if frame_control.timer.finished() {
        

        let world_map = world_map_query.single() else {
            panic!("Cannot find the world map!");
        };
        for (rabbit_entity, mut rabbit, mut transform) in rabbit_query.iter_mut() {

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
                if (rabbit.hunger >= 50 && rabbit.thirst >= 50) || (rabbit.plants_in_range.len() == 0 && rabbit.water_in_range.len() == 0) {
                    walk_randomly(&mut rabbit, &mut transform, &possible_moves);
                } else if rabbit.hunger < 50 || rabbit.thirst < 50 {
                    if rabbit.hunger <= rabbit.thirst && rabbit.plants_in_range.len() > 0 {
                        //Look for food
                        println!("Looking for food");
                        let mut x_direction = (std::i32::MAX-1) / 2;
                        let mut z_direction = (std::i32::MAX-1) / 2;

                        let mut closest_plant: Option<Entity> = None;
                        for plant_entity in rabbit.plants_in_range.clone() {
                            let Ok(plant) = foliage_query.get(plant_entity) else {
                                continue;
                            };
                            let x_direction_temp = plant.location.0 - rabbit.location.0;
                            let z_direction_temp = plant.location.1 - rabbit.location.1;

                            if x_direction_temp.abs() + z_direction_temp.abs() < x_direction.abs() + z_direction.abs() { 
                                x_direction = x_direction_temp;
                                z_direction = z_direction_temp;
                                closest_plant = Some(plant_entity);
                            }
                        }

                        if x_direction == 0 && z_direction == 0 {
                            //despawn the plant and increase hunger
                            
                            commands.entity(closest_plant.unwrap()).despawn();
                            rabbit.hunger += 10;
                        } else {
                            if x_direction.abs() > z_direction.abs() {
                                if x_direction > 0 {
                                    rabbit.location.0 += 1;
                                } else if x_direction < 0 {
                                    rabbit.location.0 -= 1;
                                }
                                transform.translation = Vec3::new((rabbit.location.0 - 30) as f32, 0.25, (rabbit.location.1 - 30) as f32);
                            } else {
                                if z_direction > 0 {
                                    rabbit.location.1 += 1;
                                } else if z_direction < 0 {
                                    rabbit.location.1 -= 1;
                                }
                                transform.translation = Vec3::new((rabbit.location.0 - 30) as f32, 0.25, (rabbit.location.1 - 30) as f32);
                            }
                        }

                        
                    } else if rabbit.hunger > rabbit.thirst && rabbit.water_in_range.len() > 0 {
                        //Look for water
                        walk_randomly(&mut rabbit, &mut transform, &possible_moves);
                    } else {
                        //Walk randomly.
                        walk_randomly(&mut rabbit, &mut transform, &possible_moves);
                    }
                }
                
            }

            //Update the rabbit's hunger and thirst
            if rabbit.hunger == 0 {
                commands.entity(rabbit_entity).despawn();
            } else {
                rabbit.hunger -= 1;
            }
            
            //rabbit.thirst -= 1;

            //Update the rabbit's nearby resources
            // binary search vector for the closest resources.

        }
    }

}

    
fn walk_randomly(
    rabbit: &mut Rabbit,
    transform: &mut Transform,
    possible_moves: &Vec<(i32, i32)>,
) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let random_index = rng.gen_range(0..possible_moves.len());
    rabbit.location = possible_moves[random_index];
    transform.translation = Vec3::new((rabbit.location.0 - 30) as f32, 0.25, (rabbit.location.1 - 30) as f32);
}

pub fn update_rabbit_nearby_resources(
    frame_control: ResMut<AnimalFrameControl>,
    mut rabbit_query: Query<&mut Rabbit>,
    foliage_query: Query<(Entity, &Foliage)>,
) {
    if frame_control.timer.finished() {
        
        for mut rabbit in rabbit_query.iter_mut() {
            let rabbit_x = rabbit.location.0;
            let rabbit_z = rabbit.location.1;

            rabbit.plants_in_range = Vec::new();

            for (i, (f_entity, foliage)) in foliage_query.iter().enumerate() {
                if (
                    rabbit_x - (rabbit.sight_distance as i32) <= foliage.location.0 
                    && rabbit_x + (rabbit.sight_distance as i32) >= foliage.location.0
                ) 
                && (
                    rabbit_z - (rabbit.sight_distance as i32) <= foliage.location.1 
                    && rabbit_z + (rabbit.sight_distance as i32) >= foliage.location.1
                ) {
                    println!("fol{}: {:?}", i, foliage.location);
                    rabbit.plants_in_range.push(f_entity);
                }
            }
            
        }
    }
    
}


pub fn debug_rabbit_info(
    frame_control: ResMut<AnimalFrameControl>,
    rabbit_query: Query<&Rabbit>,
    foliage_query: Query<&Foliage>
) {
    if frame_control.timer.finished() {
        for rabbit in rabbit_query.iter() {
            println!("Rabbit: {}, Hunger: {}, Thirst: {}, Location: {:?}", rabbit.id, rabbit.hunger, rabbit.thirst, rabbit.location);
            for plant_entity in rabbit.plants_in_range.iter() {
                let Ok(plant) = foliage_query.get(*plant_entity) else {
                    println!("    Warning: Plant not found for entity: {:?}", plant_entity);
                    continue;
                };
                println!("    Plant in range: {:?}", plant.location);
            }
        }
    }
}