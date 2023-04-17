use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    prelude::*,
    rapier::prelude::{JointAxesMask, JointAxis},
};
use std::f32::consts::PI;

#[derive(Component)]
struct OrbitCamera {
    distance: f32,
    y_angle: f32,
}

#[derive(Component)]
struct FpsCamera {}

#[derive(Component)]
struct TheCube;

#[derive(Component)]
struct Vehicle;

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "e6c67ca5-2f13-4fc1-8a29-f6c99dfaf16e"]
pub struct PerlinNoiseMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for PerlinNoiseMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/perlin_noise_material.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MaterialPlugin::<PerlinNoiseMaterial>::default())
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
    mut custom_materials: ResMut<Assets<PerlinNoiseMaterial>>,
    assets: Res<AssetServer>,
) {
    //TEMP Custom material test
    let material = custom_materials.add(PerlinNoiseMaterial { color: Color::BLUE });

    //Clear color
    commands.insert_resource(ClearColor(Color::CYAN));

    //Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.00,
    });

    commands.spawn(SpotLightBundle {
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

    //Plane
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(100.0))),
            material,
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(500.0, 0.001, 500.0));

    //Cube
    commands
        .spawn_empty()
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(TheCube);

    let vehicle_spawn_position = Vec3::new(30.0, 7.0, 1.0);
    let _vehicle = spawn_vehicle(&mut commands, meshes, materials, vehicle_spawn_position); // TODO Take spawn position

    //Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0)
                .looking_at(vehicle_spawn_position, Vec3::Y),
            ..Default::default()
        })
        .insert(FpsCamera {});

    //gltf
    let gltf = assets.load("models/not-cube/not-cube.gltf#Scene0");
    commands.spawn(SceneBundle {
        scene: gltf,
        transform: Transform::from_xyz(-2.0, 0.0, -2.0).with_scale(Vec3::splat(0.25)),
        ..default()
    });
}

fn spawn_vehicle(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawn_position: Vec3,
) -> Entity {
    let width = 4.0; // x
    let hw = width / 2.0;
    let height = 1.0; // y
    let hh = height / 2.0;
    let length = 10.0; // z
    let hl = length / 2.0;

    let color = Color::rgb(1.0, 0.2, 0.2);

    //Vehicle
    let vehicle = commands
        .spawn_empty()
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(hw, hh, hl))
        .insert(Restitution::coefficient(0.7))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -hw,
                max_x: hw,
                min_y: -hh,
                max_y: hh,
                min_z: -hl,
                max_z: hl,
            })),
            material: materials.add(color.into()),
            ..default()
        })
        .insert(SpatialBundle::from_transform(Transform::from_translation(
            spawn_position,
        )))
        .insert(Vehicle)
        .id();

    //Wheels
    let wheel_radius = 3.0;
    let wheel_thickness = 0.5;
    let hwt = wheel_thickness / 2.0;
    let wheel_color = Color::rgb(0.2, 0.2, 0.2);

    let wheel_alignments = [
        ( 1.0,  1.0),
        ( 1.0, -1.0),
        (-1.0,  1.0),
        (-1.0, -1.0),
    ];

    let _wheels: Vec<_> = wheel_alignments
        .into_iter()
        .map(|(x_align, z_align)| {
            let wheel_x = hw + wheel_thickness;
            let wheel_y = 0.0;
            let wheel_z = hl;
            let wheel_local_position = Vec3::new(wheel_x * x_align, wheel_y, wheel_z * z_align);
            let wheel_position = spawn_position + wheel_local_position;

            //Joint
            let joint = GenericJointBuilder::new(JointAxesMask::LOCKED_REVOLUTE_AXES)
                .local_axis1(Vec3::X)
                .local_axis2(-Vec3::Y)
                .local_anchor1(wheel_local_position)
                .local_anchor2(Vec3::new(0.0, 0.0, 0.0))
                .motor_velocity(JointAxis::AngX, 10.0, 0.5);
//                .set_motor(JointAxis::AngX, 0.0, 10.0, 100.0, 0.0);

            let wheel = commands
                .spawn_empty()
                .insert(RigidBody::Dynamic)
                .insert(Collider::cylinder(hwt, wheel_radius))
                .insert(Restitution::coefficient(0.7))
                .insert(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cylinder {
                        radius: wheel_radius,
                        height: wheel_thickness,
                        resolution: 20,
                        segments: 5,
                    })),
                    material: materials.add(wheel_color.into()),
                    ..default()
                })
                .insert(ImpulseJoint::new(vehicle, joint))
                .insert(SpatialBundle::from_transform(
                    Transform::from_translation(wheel_position)
                        .with_rotation(Quat::from_rotation_z(90f32.to_radians())),
                ))
                .id();

            wheel
        })
        .collect();

    //commands.entity(vehicle).push_children(&wheels); //BUG This seems to not work with rapier

    vehicle
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
