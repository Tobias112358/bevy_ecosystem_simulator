use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

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

    //Spawn a bunch of cubes.
    for i in -30..30 {
        for j in -30..30 {
            
            let p_value = perlin.get([(i as f64 + 30.)/10. , (j as f64 + 30.)/10. ]);

            if p_value < 0.0 {
                commands.spawn((
                    Mesh3d(voxel.clone()),
                    MeshMaterial3d(water_voxel_mat.clone()),
                    Transform::from_translation(Vec3::new(i as f32, -1., j as f32)),
                ));
            } else if p_value >= 0.0 && p_value < 0.15 {
                commands.spawn((
                    Mesh3d(voxel.clone()),
                    MeshMaterial3d(sand_voxel_mat.clone()),
                    Transform::from_translation(Vec3::new(i as f32, -0.85, j as f32)),
                ));
            } else {
                commands.spawn((
                    Mesh3d(voxel.clone()),
                    MeshMaterial3d(grass_voxel_mat.clone()),
                    Transform::from_translation(Vec3::new(i as f32, -0.8, j as f32)),
                ));
            }
        }
    }
}