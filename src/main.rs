use bevy::{prelude::*, transform};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct MainCamera;
#[derive(Component)]
struct TheCube;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(apply_kb_thrust)
        .add_system(aim_camera_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(MainCamera);

    //Plane
    commands
        .spawn()
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        });

    //Cube
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()
        }).insert(TheCube);

    //commands.entity(camera).push_children(&[cube]);
}

fn apply_kb_thrust(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut entities: Query<Entity, With<RigidBody>>,
) {
    if keys.pressed(KeyCode::Space) {
        for entity in entities.iter_mut() {
            commands.entity(entity).insert(ExternalImpulse {
                impulse: Vec3::new(0.0, 0.2, 0.0),
                torque_impulse: Vec3::new(0.01, 0.0, 0.0),
            });
        }
    }
}

fn aim_camera_cube(
    mut cube_query: Query<&Transform, (With<TheCube>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    //TODO Assumes exactly a single TheCube
    let Some(cube_transform) = cube_query.iter_mut().next() else {
        return
    };
    for mut transform in camera_query.iter_mut() {
        *transform = Transform::from_xyz(3.0, 3.0, 10.0).looking_at(cube_transform.translation, Vec3::Y);
    }
}
