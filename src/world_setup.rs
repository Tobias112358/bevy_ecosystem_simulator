use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

#[derive(Component, PartialEq, Clone)]
pub enum VoxelType {
    WaterVoxel,
    SandVoxel,
    GrassVoxel
}

#[derive(Component)]
pub struct Voxel {
    pub voxel_type: VoxelType,
    pub location: (i32, i32),
}

#[derive(Event)]
pub struct VoxelsSpawnedEvent(pub Vec<Vec<Entity>>);


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
        .add_systems(Update, set_world_map);
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

                if p_value < -0.15 {
                    world_voxels[(i + 30) as usize].push(parent.spawn((
                        Voxel {voxel_type: VoxelType::WaterVoxel, location: (i + 30, j + 30)},
                        Mesh3d(voxel.clone()),
                        MeshMaterial3d(water_voxel_mat.clone()),
                        Visibility::Visible,
                        Transform::from_translation(Vec3::new(i as f32, -1., j as f32)),
                    )).id());
                } else if p_value >= -0.15 && p_value < 0.05 {
                    world_voxels[(i + 30) as usize].push(parent.spawn((
                        Voxel {voxel_type: VoxelType::SandVoxel, location: (i + 30, j + 30)},
                        Mesh3d(voxel.clone()),
                        MeshMaterial3d(sand_voxel_mat.clone()),
                        Visibility::Visible,
                        Transform::from_translation(Vec3::new(i as f32, -0.85, j as f32)),
                    )).id());
                } else {
                    world_voxels[(i + 30) as usize].push(parent.spawn((
                        Voxel {voxel_type: VoxelType::GrassVoxel, location: (i + 30, j + 30)},
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