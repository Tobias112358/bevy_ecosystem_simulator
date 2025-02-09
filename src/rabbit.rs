use bevy::prelude::*;
use rand::Rng;

use crate::{
    foliage::{Foliage, FoliageConsumedEvent}, frame_manager::FrameControl, world_setup::{Voxel, VoxelType, WorldMap, WorldMapDataSetEvent}
};

pub(super) fn plugin(app: &mut App) {
    app
        .add_event::<RabbitBreedingEvent>()
        .add_event::<UpdateNearbyResourcesEvent>()
        .insert_resource(RabbitResource { rabbits: Vec::new() })
        .add_systems(Update, (spawn_initial_rabbits, rabbit_movement, update_rabbit_nearby_resources, spawn_new_rabbit,update_rabbit_partner_list, rabbit_age_tick, update_details_on_breeding));
}

#[derive(Component, Clone)]
pub struct Rabbit {
    pub id: u32,
    pub hunger: u32,
    pub thirst: u32,
    pub location: (i32, i32),
    pub plants_in_range: Vec<Entity>,
    pub water_in_range: Vec<Entity>,
    pub partner_in_range: Vec<Entity>,
    pub sight_distance: u32,
    pub satisfaction_threshold: u32,
    pub full_threshold: u32,
    pub age: u32,
    pub mating_cooldown: u32,
}

pub enum RabbitPriorityMovement {
    Partner,
    Water,
    Food,
    Random,
    None,
}

#[derive(Event)]
pub struct RabbitBreedingEvent(pub Entity, pub Entity);

#[derive(Event)]
pub struct UpdateNearbyResourcesEvent(pub Entity);

#[derive(Resource)]
pub struct RabbitResource {
    pub rabbits: Vec<Entity>,
}

#[derive(Default)]
pub struct RabbitAgeLocalCounter {
    pub counter: u32,
}


const INITIAL_RABBIT_POPULATION: u32 = 12;

pub fn spawn_initial_rabbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<WorldMapDataSetEvent>,
    world_map_query: Query<&WorldMap>,
    voxel_query: Query<&Voxel>,
    mut rabbit_resource: ResMut<RabbitResource>,
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
            
                    let rabbit = Rabbit {
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
                        age: 0,
                        mating_cooldown: 0,
                    };

                    let rand_r = rng.gen_range(0.5..1.0);
                    let rand_g = rng.gen_range(0.3..0.6);
                    let rand_b = rng.gen_range(0.15..0.3);
            
                    let rabbit_entity = commands.spawn((
                        rabbit,
                        Mesh3d(meshes.add(Cuboid {
                            half_size: Vec3::new(rng.gen_range(0.2..0.3), rng.gen_range(0.2..0.3), rng.gen_range(0.2..0.3)),
                            ..default()
                        })),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::linear_rgb(rand_r, rand_g, rand_b),
                            ..default()
                        })),
                        Transform::from_translation(Vec3::new((x - 30) as f32, -0.175, (z - 30) as f32)),
                        
                    )).id();

                    rabbit_resource.rabbits.push(rabbit_entity);
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
    mut rabbit_resource: ResMut<RabbitResource>,
) {

    for active_event in events.read() {
        match active_event {
            RabbitBreedingEvent(entity1, entity2) => {
                let rabbit1 = rabbit_query.get(*entity1).unwrap();
                let rabbit2 = rabbit_query.get(*entity2).unwrap();

                let mut rng = rand::thread_rng();

                let baby_count = rng.gen_range(1..3);

                if rabbit_resource.rabbits.len() as usize > 200 {
                    println!("Rabbit population is at maximum capacity!");
                    continue;
                }

                println!("New rabbit!");

                for _ in 0..baby_count {

                    

                    let rand_r = rng.gen_range(0.5..1.0);
                    let rand_g = rng.gen_range(0.3..0.6);
                    let rand_b = rng.gen_range(0.15..0.3);

                    let rabbit_entity = commands.spawn((
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
                            age: 0,
                            mating_cooldown: 0,
                        },
                        Mesh3d(meshes.add(Cuboid {
                            half_size: Vec3::new(rng.gen_range(0.2..0.3), rng.gen_range(0.2..0.3), rng.gen_range(0.2..0.3)),
                            ..default()
                        })),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::linear_rgb(rand_r, rand_g, rand_b),
                            ..default()
                        })),
                        Transform::from_translation(Vec3::new((rabbit1.location.0 - 30) as f32, -0.175, (rabbit2.location.1 - 30) as f32)),
                        
                    )).id();

                    rabbit_resource.rabbits.push(rabbit_entity);
                }
                
            }
        }
    }
}

fn update_details_on_breeding(
    mut events: EventReader<RabbitBreedingEvent>,
    mut rabbit_query: Query<&mut Rabbit>,
) {

    for active_event in events.read() {
        match active_event {
            RabbitBreedingEvent(entity1, entity2) => {
                let mut rabbits = rabbit_query.get_many_mut([*entity1, *entity2]).unwrap();

                rabbits[0].mating_cooldown = 20;
                rabbits[1].mating_cooldown = 20;
            }
        }
    }
}



fn rabbit_movement(
    mut commands: Commands,
    frame_control: Res<FrameControl>,
    mut rabbit_query: Query<(Entity, &mut Rabbit, &mut Transform), With<Rabbit>>,
    //partner_query: Query<&Rabbit>,
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

        let mut rabbit_priority_vector: Vec<(Entity, RabbitPriorityMovement, i32, i32, Vec<(i32, i32)>)> = Vec::new();

        for (rabbit_entity, rabbit, transform) in rabbit_query.iter() {

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

            let mut rabbit_priority_movement = RabbitPriorityMovement::None;
            let mut x_direction = (std::i32::MAX-1) / 2;
            let mut z_direction = (std::i32::MAX-1) / 2;

            if possible_moves.len() > 0 {
                if rabbit.hunger >= rabbit.satisfaction_threshold && rabbit.thirst >= rabbit.satisfaction_threshold && rabbit.partner_in_range.len() > 0 && rabbit.age > 20 && rabbit.mating_cooldown == 0 {
                    //Look for partner
                    println!("Looking for partner");

                    

                    let mut closest_partner: Option<Entity> = None;
                    for partner_entity in rabbit.partner_in_range.clone() {

                        let Ok((_, partner, _)) = rabbit_query.get(partner_entity) else {
                            continue;
                        };

                        //I need to get the partner's location
                        let x_direction_temp = partner.location.0 - rabbit.location.0;
                        let z_direction_temp = partner.location.1 - rabbit.location.1;

                        if x_direction_temp.abs() + z_direction_temp.abs() < x_direction.abs() + z_direction.abs() { 
                            x_direction = x_direction_temp;
                            z_direction = z_direction_temp;
                            closest_partner = Some(partner_entity);
                        }
                    }

                    if (x_direction >= -1 && x_direction <= 1) && (z_direction >= -1 && z_direction <= 1) {
                        //spawn new rabbit
                        rabbit_breeding_event_writer.send(RabbitBreedingEvent(rabbit_entity, closest_partner.unwrap()));
                        
                    } else {
                        rabbit_priority_movement = RabbitPriorityMovement::Partner;
                    }

                    
                } else if (rabbit.hunger >= rabbit.full_threshold && rabbit.thirst >= rabbit.full_threshold) || (rabbit.plants_in_range.len() == 0 && rabbit.water_in_range.len() == 0) {

                    rabbit_priority_movement = RabbitPriorityMovement::Random;

                } else if rabbit.hunger < rabbit.full_threshold || rabbit.thirst < rabbit.full_threshold {
                    if rabbit.hunger <= rabbit.thirst && rabbit.plants_in_range.len() > 0 {
                        //Look for food
                        //println!("Looking for food");

                        rabbit_priority_movement = RabbitPriorityMovement::Food;

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
                            event_writer.send(FoliageConsumedEvent(closest_plant.unwrap()));
                        }
                        
                        
                    } else if rabbit.hunger > rabbit.thirst && rabbit.water_in_range.len() > 0 {
                        //Look for water
                        //println!("Looking for water");

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

                        rabbit_priority_movement = RabbitPriorityMovement::Water;

                        
                    } else {
                        //Walk randomly.
                        rabbit_priority_movement = RabbitPriorityMovement::Random;
                    }
                }

                
            }

            rabbit_priority_vector.push((rabbit_entity, rabbit_priority_movement, x_direction, z_direction, possible_moves));


            //Update the rabbit's nearby resources
            // binary search vector for the closest resources.

        }

        for (rabbit_entity, rabbit_priority_movement, x_direction, z_direction, possible_moves) in rabbit_priority_vector {
            match rabbit_priority_movement {
                RabbitPriorityMovement::Partner => {

                    let (rabbit_entity, mut rabbit, mut transform) = rabbit_query.get_mut(rabbit_entity).unwrap();

                    
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
                RabbitPriorityMovement::Food => {

                    let (rabbit_entity, mut rabbit, mut transform) = rabbit_query.get_mut(rabbit_entity).unwrap();

                    if x_direction == 0 && z_direction == 0 {
                        //despawn the plant and increase hunger
                        
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
                }
                RabbitPriorityMovement::Water => {
                    let (rabbit_entity, mut rabbit, mut transform) = rabbit_query.get_mut(rabbit_entity).unwrap();
                    
                    if (x_direction.abs() == 1 && z_direction == 0) || (z_direction.abs() == 1 && x_direction == 0) {
                        //despawn the plant and increase hunger
                        
                        //println!("Drinking Water!");
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
                }
                RabbitPriorityMovement::Random => {
                    //Walk randomly
                    let (rabbit_entity, mut rabbit, mut transform) = rabbit_query.get_mut(rabbit_entity).unwrap();
                    
                    walk_randomly(&mut rabbit, &mut transform, &possible_moves);
                    update_rabbit_hunger_and_thirst(&mut rabbit, rabbit_entity, &mut commands);
                }
                RabbitPriorityMovement::None => {
                    //No priority movement
                }
            }
        }
    }

}

fn update_rabbit_hunger_and_thirst(
    rabbit: &mut Rabbit,
    rabbit_entity: Entity,
    commands: &mut Commands,
) {
    
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
    voxel_query: Query<(Entity, &Voxel)>,
    mut event_writer: EventWriter<UpdateNearbyResourcesEvent>,
) {
    if frame_control.timer.finished() {
        
        for (rabbit_entity, mut rabbit) in rabbit_query.iter_mut() {
            let rabbit_x = rabbit.location.0;
            let rabbit_z = rabbit.location.1;

            rabbit.plants_in_range = Vec::new();
            rabbit.water_in_range = Vec::new();

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
                    //println!("fol{}: {:?}", i, foliage.location);
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
                        //println!("vox{}: {:?}", j, voxel.location);
                        rabbit.water_in_range.push(v_entity);
                    }
                }
            }

            // I need to figure out a way to query all other rabbits in a scene for this scenario.
            if rabbit.age > 20 {
                //event_writer.send(UpdateNearbyResourcesEvent(rabbit_entity));
            }


            // for (i, (r_entity, partner)) in available_rabbits.clone().into_iter().enumerate() {
            //     if (
            //         rabbit_x - (rabbit.sight_distance as i32) <= partner.location.0 
            //         && rabbit_x + (rabbit.sight_distance as i32) >= partner.location.0
            //     ) 
            //     && (
            //         rabbit_z - (rabbit.sight_distance as i32) <= partner.location.1 
            //         && rabbit_z + (rabbit.sight_distance as i32) >= partner.location.1
            //     ) {
            //         //println!("rabbit{}: {:?}", i, partner.location);
            //         rabbit.partner_in_range.push((r_entity, partner.clone()));
            //     }
            // }
        }
    }
}

fn update_rabbit_partner_list(
    frame_control: Res<FrameControl>,
    mut rabbit_resource: ResMut<RabbitResource>,
    mut rabbit_query: Query<(Entity, &mut Rabbit)>,
    mut events: EventReader<UpdateNearbyResourcesEvent>,
) {

    if frame_control.timer.finished() {
        if rabbit_resource.rabbits.len() as usize > 200 {
            return;
        }

        for rabbit_entity in rabbit_resource.rabbits.iter_mut() {

            let Ok((_, rabbit)) = rabbit_query.get(*rabbit_entity) else {
                continue;
            };
    
            if rabbit.age < 20 {
                continue;
            }
    
            //println!("Updating partner list for rabbit: {:?}", rabbit_entity);
    
            let rabbit_x = rabbit.location.0;
            let rabbit_z = rabbit.location.1;
    
            let mut available_rabbits: Vec<Entity> = Vec::new();
    
            for (partner_entity, partner) in rabbit_query.iter() {
                if partner_entity == *rabbit_entity {
                    continue;
                }
                if partner.age < 20 || partner.mating_cooldown > 0 {
                    continue;
                }
    
                if (
                    rabbit_x - (rabbit.sight_distance as i32) <= partner.location.0 
                    && rabbit_x + (rabbit.sight_distance as i32) >= partner.location.0
                ) 
                && (
                    rabbit_z - (rabbit.sight_distance as i32) <= partner.location.1 
                    && rabbit_z + (rabbit.sight_distance as i32) >= partner.location.1
                ) {
                    //println!("rabbit {:?} for partner rabbit: {:?}", rabbit.location, partner.location);
                    available_rabbits.push(partner_entity);
                }
            }
    
            let Ok((_, mut rabbit)) = rabbit_query.get_mut(*rabbit_entity) else {
                continue;
            };
    
            rabbit.partner_in_range = available_rabbits;
        }
    }
}


fn rabbit_age_tick(
    mut commands: Commands,
    frame_control: Res<FrameControl>,
    mut rabbit_resource: ResMut<RabbitResource>,
    mut rabbit_query: Query<(Entity, &mut Rabbit)>,
    mut local_counter: Local<RabbitAgeLocalCounter>,
) {
    if frame_control.timer.finished() {
        local_counter.counter += 1;
        if local_counter.counter >= 5 {
            for (entity, mut rabbit) in rabbit_query.iter_mut() {
                rabbit.age += 1;
                if rabbit.mating_cooldown > 0 {
                    rabbit.mating_cooldown -= 1;
                }
                if rabbit.age >= 50 {
                    let mut rng = rand::thread_rng();
                    if rabbit.age < 100 {
                        let random_number = rng.gen_range(0..(100 - rabbit.age));
                        if random_number == 0 {
                            rabbit_resource.rabbits.retain(|&x| x != entity);
                            commands.entity(entity).despawn();
                        }
                    } else {
                        rabbit_resource.rabbits.retain(|&x| x != entity);
                        commands.entity(entity).despawn();
                    }
                }
            }
            local_counter.counter = 0;
        }
    }
}