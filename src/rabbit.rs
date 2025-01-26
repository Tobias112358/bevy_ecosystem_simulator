use bevy::prelude::*;
use rand::Rng;

use crate::{
    foliage::{Foliage, FoliageConsumedEvent}, frame_manager::FrameControl, world_setup::{Voxel, VoxelType, WorldMap, WorldMapDataSetEvent}
};

pub(super) fn plugin(app: &mut App) {
    app
        .add_event::<RabbitBreedingEvent>()
        .add_systems(Update, (spawn_initial_rabbits, rabbit_movement, update_rabbit_nearby_resources, debug_rabbit_info, spawn_new_rabbit));
}

#[derive(Component, Clone)]
pub struct Rabbit {
    pub id: u32,
    pub hunger: u32,
    pub thirst: u32,
    pub location: (i32, i32),
    pub plants_in_range: Vec<Entity>,
    pub water_in_range: Vec<Entity>,
    pub partner_in_range: Vec<(Entity, Rabbit)>,
    pub sight_distance: u32,
    pub satisfaction_threshold: u32,
    pub full_threshold: u32,
}

#[derive(Event)]
pub struct RabbitBreedingEvent(pub Entity, pub Entity);


const INITIAL_RABBIT_POPULATION: u32 = 8;

pub fn spawn_initial_rabbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<WorldMapDataSetEvent>,
    world_map_query: Query<&WorldMap>,
    voxel_query: Query<&Voxel>,
) {
    for active_event in events.read() {
        match active_event {
            WorldMapDataSetEvent => {
                let world_map = world_map_query.single();
                for i in 0..INITIAL_RABBIT_POPULATION {

                    let mut rng = rand::thread_rng();
            
                    let mut x: i32;
                    let mut z: i32;

                    loop {
                        x = rng.gen_range(0..world_map.width);
                        z = rng.gen_range(0..world_map.height);

                        println!("{}, {}", x, z);

                        let Ok(current_voxel) = voxel_query.get(world_map.map[x as usize][z as usize]) else {
                            continue;
                        };

                        if current_voxel.voxel_type == VoxelType::GrassVoxel {
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
                            partner_in_range: Vec::new(),
                            sight_distance: 3,
                            satisfaction_threshold: 50,
                            full_threshold: 70,
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
    
}

pub fn spawn_new_rabbit(
    mut events: EventReader<RabbitBreedingEvent>,
    mut commands: Commands,
    rabbit_query: Query<&Rabbit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    for active_event in events.read() {
        match active_event {
            RabbitBreedingEvent(entity1, entity2) => {
                let rabbit1 = rabbit_query.get(*entity1).unwrap();
                let rabbit2 = rabbit_query.get(*entity2).unwrap();

                let mut rng = rand::thread_rng();

                let baby_count = rng.gen_range(1..3);

                for _ in 0..baby_count {
                    commands.spawn((
                        Rabbit {
                            id: rabbit1.id + rabbit2.id,
                            hunger: 50,
                            thirst: 50,
                            location: (rabbit1.location.0, rabbit2.location.1),
                            plants_in_range: Vec::new(),
                            water_in_range: Vec::new(),
                            partner_in_range: Vec::new(),
                            sight_distance: 3,
                            satisfaction_threshold: 50,
                            full_threshold: 70,
                        },
                        Mesh3d(meshes.add(Cuboid {
                            half_size: Vec3::new(0.5, 0.5, 0.5),
                            ..default()
                        })),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::linear_rgb(0.625, 0.32, 0.1666),
                            ..default()
                        })),
                        Transform::from_translation(Vec3::new((rabbit1.location.0 - 30) as f32, 0.25, (rabbit2.location.1 - 30) as f32)),
                        
                    ));
                }
                
            }
        }
    }
}


fn rabbit_movement(
    mut commands: Commands,
    frame_control: Res<FrameControl>,
    mut rabbit_query: Query<(Entity, &mut Rabbit, &mut Transform), With<Rabbit>>,
    voxel_query: Query<&Voxel>,
    foliage_query: Query<&Foliage>,
    world_map_query: Query<&WorldMap>,
    mut event_writer: EventWriter<FoliageConsumedEvent>,
    mut rabbit_breeding_event_writer: EventWriter<RabbitBreedingEvent>,
) {

    if frame_control.timer.finished() {
        

        let Ok(world_map) = world_map_query.get_single() else {
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
                        if current_voxel.voxel_type == VoxelType::GrassVoxel || current_voxel.voxel_type == VoxelType::SandVoxel {
                            possible_moves.push((x, z));
                        }
                    }
                }
            }


            if possible_moves.len() > 0 {
                if rabbit.hunger >= rabbit.satisfaction_threshold && rabbit.thirst >= rabbit.satisfaction_threshold && rabbit.partner_in_range.len() > 0 {
                    //Look for partner
                    println!("Looking for partner");
                    let mut x_direction = (std::i32::MAX-1) / 2;
                    let mut z_direction = (std::i32::MAX-1) / 2;

                    let mut closest_partner: Option<Entity> = None;
                    for (partner_entity, partner) in rabbit.partner_in_range.clone() {

                        //I need to get the partner's location
                        let x_direction_temp = partner.location.0 - rabbit.location.0;
                        let z_direction_temp = partner.location.1 - rabbit.location.1;

                        if x_direction_temp.abs() + z_direction_temp.abs() < x_direction.abs() + z_direction.abs() { 
                            x_direction = x_direction_temp;
                            z_direction = z_direction_temp;
                            closest_partner = Some(partner_entity);
                        }
                    }

                    if x_direction == 0 && z_direction == 0 {
                        //spawn new rabbit
                        rabbit_breeding_event_writer.send(RabbitBreedingEvent(rabbit_entity, closest_partner.unwrap()));
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
                } else if (rabbit.hunger >= rabbit.full_threshold && rabbit.thirst >= rabbit.full_threshold) || (rabbit.plants_in_range.len() == 0 && rabbit.water_in_range.len() == 0) {
                    walk_randomly(&mut rabbit, &mut transform, &possible_moves);
                } else if rabbit.hunger < rabbit.satisfaction_threshold || rabbit.thirst < rabbit.satisfaction_threshold {
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
                            
                            event_writer.send(FoliageConsumedEvent(closest_plant.unwrap()));
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
                        println!("Looking for water");
                        let mut x_direction = (std::i32::MAX-1) / 2;
                        let mut z_direction = (std::i32::MAX-1) / 2;

                        for voxel_entity in rabbit.water_in_range.clone() {
                            let Ok(voxel) = voxel_query.get(voxel_entity) else {
                                continue;
                            };
                            let x_direction_temp = voxel.location.0 - rabbit.location.0;
                            let z_direction_temp = voxel.location.1 - rabbit.location.1;

                            if x_direction_temp.abs() + z_direction_temp.abs() < x_direction.abs() + z_direction.abs() {
                                x_direction = x_direction_temp;
                                z_direction = z_direction_temp;
                            }
                        }

                        if (x_direction.abs() == 1 && z_direction == 0) || (z_direction.abs() == 1 && x_direction == 0) {
                            //despawn the plant and increase hunger
                            
                            println!("Drinking Water!");
                            rabbit.thirst += 10;
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

            if rabbit.thirst == 0 {
                commands.entity(rabbit_entity).despawn();
            } else {
                rabbit.thirst -= 1;
            }

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
    frame_control: Res<FrameControl>,
    mut rabbit_query: Query<(Entity, &mut Rabbit)>,
    foliage_query: Query<(Entity, &Foliage)>,
    voxel_query: Query<(Entity, &Voxel)>
) {
    if frame_control.timer.finished() {

        let mut available_rabbits: Vec<(Entity, Rabbit)> = Vec::new();
        
        for (rabbit_entity, mut rabbit) in rabbit_query.iter_mut() {
            let rabbit_x = rabbit.location.0;
            let rabbit_z = rabbit.location.1;

            rabbit.plants_in_range = Vec::new();
            rabbit.water_in_range = Vec::new();
            rabbit.partner_in_range = Vec::new();

            for (i, (f_entity, foliage)) in foliage_query.iter().enumerate() {
                if foliage.consumed {
                    continue;
                }
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
            
            for (j, (v_entity, voxel)) in voxel_query.iter().enumerate() {
                if voxel.voxel_type == VoxelType::WaterVoxel {
                    if (
                        rabbit_x - (rabbit.sight_distance as i32) <= voxel.location.0 
                        && rabbit_x + (rabbit.sight_distance as i32) >= voxel.location.0
                    ) 
                    && (
                        rabbit_z - (rabbit.sight_distance as i32) <= voxel.location.1 
                        && rabbit_z + (rabbit.sight_distance as i32) >= voxel.location.1
                    ) {
                        println!("vox{}: {:?}", j, voxel.location);
                        rabbit.water_in_range.push(v_entity);
                    }
                }
            }

            // I need to figure out a way to query all other rabbits in a scene for this scenario.
            for (i, (r_entity, partner)) in available_rabbits.clone().into_iter().enumerate() {
                if (
                    rabbit_x - (rabbit.sight_distance as i32) <= partner.location.0 
                    && rabbit_x + (rabbit.sight_distance as i32) >= partner.location.0
                ) 
                && (
                    rabbit_z - (rabbit.sight_distance as i32) <= partner.location.1 
                    && rabbit_z + (rabbit.sight_distance as i32) >= partner.location.1
                ) {
                    println!("rabbit{}: {:?}", i, partner.location);
                    rabbit.partner_in_range.push((r_entity, partner.clone()));
                }
            }
        }
    }
}


pub fn debug_rabbit_info(
    frame_control: Res<FrameControl>,
    rabbit_query: Query<&Rabbit>,
    foliage_query: Query<&Foliage>,
    voxel_query: Query<&Voxel>,
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
            for voxel_entity in rabbit.water_in_range.iter() {
                let Ok(voxel) = voxel_query.get(*voxel_entity) else {
                    println!("    Warning: Water not found for entity: {:?}", voxel_entity);
                    continue;
                };
                println!("    Water Voxel in range: {:?}", voxel.location);
            }
        }
    }
}