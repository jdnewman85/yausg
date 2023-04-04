use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    window::PrimaryWindow, input::mouse::MouseMotion,
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
struct FpsCamera { }


#[derive(Component)]
struct TheCube;

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "e6c67ca5-2f13-4fc1-8a29-f6c99dfaf16e"]
pub struct RedMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for RedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/red_material.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MaterialPlugin::<RedMaterial>::default())
        .add_startup_system(setup)
        .add_system(apply_kb_thrust)
        .add_system(aim_camera_cube)
        .add_system(raycast_system)
        .add_system(fps_camera_controls)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<RedMaterial>>,
    assets: Res<AssetServer>,
) {
    //TEMP Custom material test
    let material = custom_materials.add(RedMaterial { color: Color::BLUE });

    //Clear color
    commands.insert_resource(ClearColor(Color::ALICE_BLUE));

    //Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.00,
    });

    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(-1.0, 2.0, 0.0)
            .looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: 1600.0,
            color: Color::WHITE,
            shadows_enabled: true,
            inner_angle: 0.6,
            outer_angle: 0.8,
            ..default()
        },
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
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
    });

    //Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FpsCamera { });
        /*
        .insert(OrbitCamera {
            distance: 5.0,
            y_angle: 0.0,
        });
        */

    //Plane
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(10.0))),
            material,
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(5.0, 0.1, 5.0));

    //Cube
    commands
        .spawn(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..default()
        })
        .insert(TheCube);

    //gltf
    let gltf = assets.load("models/not-cube/not-cube.gltf#Scene0");
    commands.spawn(SceneBundle {
        scene: gltf,
        transform: Transform::from_xyz(-2.0, 0.0, -2.0).with_scale(Vec3::new(0.25, 0.25, 0.25)),
        ..default()
    });
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
    keys: Res<Input<KeyCode>>,
    cube_query: Query<&Transform, (With<TheCube>, Without<OrbitCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let Ok(cube_transform) = cube_query.get_single() else { return };
    let Ok((mut transform, mut camera)) = camera_query.get_single_mut() else { return };

    if keys.pressed(KeyCode::A) {
        camera.y_angle -= 0.05;
    }
    if keys.pressed(KeyCode::D) {
        camera.y_angle += 0.05;
    }
    if keys.pressed(KeyCode::S) {
        camera.distance -= 0.5;
    }
    if keys.pressed(KeyCode::W) {
        camera.distance += 0.5;
    }

    let cube_position = cube_transform.translation;
    let start_position = cube_position + Vec3::new(camera.distance, 0.0, 0.0);
    let mut camera_transform = Transform::from_translation(start_position);
    camera_transform.translate_around(
        cube_position,
        Quat::from_euler(EulerRot::YXZ, camera.y_angle, 0.0, PI / 4.0),
    );
    *transform = camera_transform.looking_at(cube_transform.translation, Vec3::Y);
}

//TODO Should factor out the raycast into a failable function
fn raycast(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,

    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cube_query: Query<(Entity, &Handle<StandardMaterial>), With<TheCube>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Option<()> {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return None;
    };

    let primary_window = window_query.get_single().unwrap();
    let cursor_position = primary_window.cursor_position()?;

    let Ok((camera, camera_transform)) = camera_query.get_single() else { return None };
    let cursor_ray = camera.viewport_to_world(camera_transform, cursor_position)?;
    let (entity, _toi) = rapier_context.cast_ray(
        cursor_ray.origin,           //position
        cursor_ray.direction,        //rotation
        f32::MAX,                    //max_toi
        true,                        //solid
        QueryFilter::only_dynamic(), //filter
    )?;

    //let ray_hit_position = cursor_ray.origin + cursor_ray.direction * toi;
    //println!("Entity {:?} @ {}", entity, ray_hit_position);

    for (cube_entity, cube_material_handle) in cube_query.iter_mut() {
        if entity != cube_entity {
            continue;
        };

        let Some(cube_material) = materials.get_mut(cube_material_handle) else { continue };
        cube_material.base_color = Color::rgb(rand::random(), rand::random(), rand::random());
    }

    return Some(());
}

fn raycast_system(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,

    window_query: Query<&Window, With<PrimaryWindow>>,
    cube_query: Query<(Entity, &Handle<StandardMaterial>), With<TheCube>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    raycast(
        camera_query,
        mouse_buttons,
        rapier_context,
        window_query,
        cube_query,
        materials,
    );
}

use bevy::window::CursorGrabMode;
fn fps_camera_controls(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&FpsCamera, &mut Transform)>,
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

    let cursor_locked = window.cursor.grab_mode == CursorGrabMode::Confined || window.cursor.grab_mode == CursorGrabMode::Locked;
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

        let move_speed = 0.05;
        if keys.pressed(KeyCode::A) {
            let move_direction = camera_transform.local_x();
            camera_transform.translation += move_direction * -move_speed;
        }
        if keys.pressed(KeyCode::D) {
            let move_direction = camera_transform.local_x();
            camera_transform.translation += move_direction * move_speed;
        }
        if keys.pressed(KeyCode::S) {
            let move_direction = camera_transform.local_z();
            camera_transform.translation += move_direction * move_speed;
        }
        if keys.pressed(KeyCode::W) {
            let move_direction = camera_transform.local_z();
            camera_transform.translation += move_direction * -move_speed;
        }

    }

    ev_motion.clear();
}
