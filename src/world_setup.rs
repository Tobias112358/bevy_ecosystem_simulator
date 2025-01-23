use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

#[derive(Component, PartialEq)]
pub enum Voxel {
    WaterVoxel,
    SandVoxel,
    GrassVoxel
}
#[derive(Component)]
pub struct Foliage;


#[derive(Event)]
pub struct VoxelsSpawnedEvent(Vec<Vec<Entity>>);


#[derive(Event)]
pub struct WorldMapDataSetEvent;

#[derive(Component)]
pub struct WorldMap{
    pub map: Vec<Vec<Entity>>,
    pub width: i32,
    pub height: i32,
}

pub(super) fn plugin(app: &mut App) {
    app
        .add_event::<VoxelsSpawnedEvent>()
        .add_event::<WorldMapDataSetEvent>()
        .add_systems(Startup, spawn_world)
        .add_systems(Update, (generate_foliage, set_world_map));
}


pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_writer: EventWriter<VoxelsSpawnedEvent>,
) {
    let world_mesh = Mesh3d(meshes.add(Plane3d {
        half_size: Vec2::new(30.0, 30.0),
        ..default()
    }));
    let world_mesh_mat = MeshMaterial3d(
        materials.add(StandardMaterial {
            base_color: Color::linear_rgba(0.1, 0.9, 0.3, 0.0),
            ..default()
        })
    );



    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 1_000_000.0,
            radius: 1_000_000.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
    ));

    //Spawn World base.
    let mut world = commands.spawn((
            WorldMap {
                map: Vec::new(),
                width: 60,
                height: 60,
            },
            world_mesh,
            world_mesh_mat,
            Visibility::Hidden,
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        )
    );


    let voxel = meshes.add(Cuboid {
        half_size: Vec3::new(0.5,0.5,0.5),
        ..default()
    });

    let water_voxel_mat = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.0, 0.0,1.0),
        ..default()
    });

    let grass_voxel_mat = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.0, 1.0,0.0),
        ..default()
    });

    let sand_voxel_mat = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(1.0, 1.0,0.6),
        ..default()
    });

    let perlin = Perlin::new(5593487);

    let mut world_voxels: Vec<Vec<Entity>> = Vec::new();

    world.with_children(|parent| {

        //Spawn a bunch of cubes.
        for i in -30..30 {
            world_voxels.push(Vec::new());
            for j in -30..30 {
                
                let p_value = perlin.get([(i as f64 + 30.)/10. , (j as f64 + 30.)/10. ]);

                if p_value < 0.0 {
                    world_voxels[(i + 30) as usize].push(parent.spawn((
                        Voxel::WaterVoxel,
                        Mesh3d(voxel.clone()),
                        MeshMaterial3d(water_voxel_mat.clone()),
                        Visibility::Visible,
                        Transform::from_translation(Vec3::new(i as f32, -1., j as f32)),
                    )).id());
                } else if p_value >= 0.0 && p_value < 0.15 {
                    world_voxels[(i + 30) as usize].push(parent.spawn((
                        Voxel::SandVoxel,
                        Mesh3d(voxel.clone()),
                        MeshMaterial3d(sand_voxel_mat.clone()),
                        Visibility::Visible,
                        Transform::from_translation(Vec3::new(i as f32, -0.85, j as f32)),
                    )).id());
                } else {
                    world_voxels[(i + 30) as usize].push(parent.spawn((
                        Voxel::GrassVoxel,
                        Mesh3d(voxel.clone()),
                        MeshMaterial3d(grass_voxel_mat.clone()),
                        Visibility::Visible,
                        Transform::from_translation(Vec3::new(i as f32, -0.8, j as f32)),
                    )).id());
                }
            }
        }
    });

    event_writer.send(VoxelsSpawnedEvent(world_voxels));
}

fn set_world_map(
    mut event_reader: EventReader<VoxelsSpawnedEvent>,
    mut event_writer: EventWriter<WorldMapDataSetEvent>,
    mut world_query: Query<(Entity, &mut WorldMap)>,
) {

    for active_event in event_reader.read() {
        match active_event {
            VoxelsSpawnedEvent(world_voxels) => {
                
                println!("Event received 2!");
                world_query.single_mut().1.map = world_voxels.clone();
                println!("Event sending!");
                event_writer.send(WorldMapDataSetEvent);
                println!("Event se t!");
            }
        }
    }
}

fn generate_foliage(
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

                for (entity, _transform, voxel) in voxel_query.iter_mut() {
                    if voxel != &Voxel::GrassVoxel {
                        continue;
                    }
                    if rand::random::<f32>() < 0.1 {
                        commands.entity(entity).with_children(|parent| {
                            parent.spawn((
                                Foliage,
                                foliage.clone(),
                                foliage_mat.clone(),
                                Transform::from_translation(Vec3::new(0.0, 0.75, 0.0)),
                            ));
                        });
                    }
                }
            }
        }
    }
}