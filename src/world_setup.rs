use bevy::prelude::*;
use rand::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app
        .add_systems(Startup, spawn_world);
}


pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let world_mesh = Mesh3d(meshes.add(Plane3d {
        half_size: Vec2::new(30.0, 30.0),
        ..default()
    }));
    let world_mesh_mat = MeshMaterial3d(
        materials.add(StandardMaterial {
            base_color: Color::linear_rgb(0.1, 0.9, 0.3),
            ..default()
        })
    );

    //Spawn World base.
    commands.spawn((
                world_mesh,
                world_mesh_mat,
                Transform::from_translation(Vec3::new(0.0, -5.0, 0.0)),
        )
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

    
    let mut rng = rand::thread_rng();

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


    //Spawn a bunch of cubes.
    for i in -50..50 {
        for j in -50..50 {
            let r: f32 = rng.gen();

            if r < 0.3 {
                commands.spawn((
                    Mesh3d(voxel.clone()),
                    MeshMaterial3d(water_voxel_mat.clone()),
                    Transform::from_translation(Vec3::new(i as f32, -1.0, j as f32)),
                ));
            } else {
                commands.spawn((
                    Mesh3d(voxel.clone()),
                    MeshMaterial3d(grass_voxel_mat.clone()),
                    Transform::from_translation(Vec3::new(i as f32, -1.0, j as f32)),
                ));
            }
            
        }
    }
}