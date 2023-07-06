use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    input::mouse::MouseMotion,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
struct OrbitCamera {
    distance: f32,
    y_angle: f32,
}

#[derive(Component)]
struct GodModeCamera {}

#[derive(Component)]
struct OrbitalTarget;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(orbital_camera_system)
        //.add_system(god_mode_camera_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials2d: ResMut<Assets<ColorMaterial>>,
) {
    //Clear color
    commands.insert_resource(ClearColor(Color::CYAN));

    //Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.10,
    });

    commands
        .spawn(Name::new("Spotlight"))
        .insert(SpotLightBundle {
            transform: Transform::from_xyz(-1.0, 2.0, 0.0).looking_at(Vec3::NEG_X, Vec3::Z),
            spot_light: SpotLight {
                intensity: 1600.0,
                color: Color::WHITE,
                shadows_enabled: true,
                inner_angle: 0.6,
                outer_angle: 0.8,
                ..default()
            },
            ..default()
        })
    ;
    commands
        .spawn(Name::new("Directional Light"))
        .insert(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.0),
                ..default()
            },
            ..default()
        })
    ;

    commands
        .spawn(Name::new("Ground Plane"))
        .insert(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(100.0))),
            material: materials.add(Color::rgb(0.7, 0.9, 0.7).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(500.0, 0.001, 500.0))
        .insert(Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(OrbitalTarget)
    ;

    commands
        .spawn(Name::new("The Cube"))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.4, 0.4, 1.0).into()),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
    ;

    commands
        .spawn(Name::new("3D Camera"))
        .insert(Camera3dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        })
        //.insert(GodModeCamera {});
        .insert(OrbitCamera {
            distance: 25.0,
            y_angle: 0.0,
        })
    ;

    commands
        .spawn(Name::new("UI Camera"))
        .insert(Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..default()
        })
    ;

    commands
        .spawn(Name::new("UI Circle"))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials2d.add(Color::PURPLE.into()),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            ..default()
        })
    ;
}

fn orbital_camera_system(
    keys: Res<Input<KeyCode>>,
    orbital_target_query: Query<&Transform, (With<OrbitalTarget>, Without<OrbitCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let Ok(orbital_target) = orbital_target_query.get_single() else { return };
    let Ok((mut transform, mut camera)) = camera_query.get_single_mut() else { return };

    if keys.pressed(KeyCode::S) {
        camera.y_angle -= 0.05;
    }
    if keys.pressed(KeyCode::F) {
        camera.y_angle += 0.05;
    }
    if keys.pressed(KeyCode::E) {
        camera.distance -= 0.5;
    }
    if keys.pressed(KeyCode::D) {
        camera.distance += 0.5;
    }

    let target_position = orbital_target.translation;
    let start_position = target_position + Vec3::new(camera.distance, 0.0, 0.0);
    let mut camera_transform = Transform::from_translation(start_position);
    camera_transform.translate_around(
        target_position,
        Quat::from_euler(EulerRot::YXZ, camera.y_angle, 0.0, PI / 4.0),
    );
    *transform = camera_transform.looking_at(orbital_target.translation, Vec3::Y);
}

use bevy::window::CursorGrabMode;
fn god_mode_camera_system(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&GodModeCamera, &mut Transform)>,
    mut ev_motion: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
) {
    let Ok(mut window) = window_query.get_single_mut() else { return };
    let Ok((_camera, mut camera_transform)) = camera_query.get_single_mut() else { return };

    if mouse_buttons.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    if keys.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }

    let cursor_locked = window.cursor.grab_mode == CursorGrabMode::Confined
        || window.cursor.grab_mode == CursorGrabMode::Locked;
    if cursor_locked {
        let mut mouse_move = Vec2::ZERO;
        for motion_event in ev_motion.into_iter() {
            mouse_move += motion_event.delta;
        }

        if mouse_move.length_squared() > 0.0 {
            let delta_x = {
                let delta = mouse_move.x / window.resolution.width() * std::f32::consts::PI * 2.0;
                //if pan_orbit.upside_down { -delta } else { delta }
                delta
            };
            let delta_y = mouse_move.y / window.resolution.height() * std::f32::consts::PI;

            let sensitivity = 0.1;
            let delta_x = delta_x * sensitivity;
            let delta_y = delta_y * sensitivity;

            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            camera_transform.rotation = yaw * camera_transform.rotation; // rotate around global y axis
            camera_transform.rotation = camera_transform.rotation * pitch; // rotate around local x axis
        }

        //let move_speed = 0.05;
        let move_speed = 0.2;
        let mut move_direction = Vec3::ZERO;
        if keys.pressed(KeyCode::S) {
            move_direction -= camera_transform.local_x();
        }
        if keys.pressed(KeyCode::F) {
            move_direction += camera_transform.local_x();
        }
        if keys.pressed(KeyCode::D) {
            move_direction += camera_transform.local_z();
        }
        if keys.pressed(KeyCode::E) {
            move_direction -= camera_transform.local_z();
        }
        if keys.pressed(KeyCode::Space) {
            move_direction += camera_transform.local_y();
        }
        if keys.pressed(KeyCode::Z) {
            move_direction -= camera_transform.local_y();
        }

        camera_transform.translation += move_direction.normalize_or_zero() * move_speed;
    }

    ev_motion.clear();
}
