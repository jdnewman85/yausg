use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    input::mouse::MouseMotion,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    prelude::*,
    rapier::prelude::{JointAxesMask, JointAxis},
};
use std::{f32::consts::PI, ops::Not};

#[derive(Component)]
struct OrbitCamera {
    distance: f32,
    y_angle: f32,
}

#[derive(Component)]
struct GodModeCamera {}

#[derive(Component)]
struct OrbitalTarget;

#[derive(Component)]
struct Vehicle;

#[derive(Component)]
struct WheelMotor;

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
        .add_system(orbital_camera_system)
        .add_system(raycast_system)
        .add_system(kb_motor)
        //.add_system(god_mode_camera_system)
        .run();
}

const STATIC_GROUP: Group = Group::GROUP_1;
const DYNAMIC_GROUP: Group = Group::GROUP_2;
const CAR_GROUP: Group = Group::GROUP_10;
const WHEEL_GROUP: Group = Group::GROUP_11;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials2d: ResMut<Assets<ColorMaterial>>,
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
        });
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
        });

    commands
        .spawn(Name::new("Ground Plane"))
        .insert(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(100.0))),
            material,
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(500.0, 0.001, 500.0))
        .insert(Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(CollisionGroups::new(STATIC_GROUP, Group::all()));

    commands
        .spawn(Name::new("The Cube"))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(CollisionGroups::new(DYNAMIC_GROUP, Group::all()));

    let vehicle_spawn_position = Vec3::new(30.0, 7.0, 1.0);
    let vehicle = spawn_vehicle(
        &mut commands,
        &mut meshes,
        materials,
        vehicle_spawn_position,
    ); // TODO Take spawn position
    commands
        .entity(vehicle)
        .insert(OrbitalTarget)
        //TODO Move into spawn_vehicle
        .insert(CollisionGroups::new(
            CAR_GROUP,
            CAR_GROUP.union(WHEEL_GROUP).not(),
        ));

    commands
        .spawn(Name::new("3D Camera"))
        .insert(Camera3dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            transform: Transform::from_xyz(1.0, 1.0, 1.0)
                .looking_at(vehicle_spawn_position, Vec3::Y),
            ..Default::default()
        })
        //.insert(GodModeCamera {});
        .insert(OrbitCamera {
            distance: 25.0,
            y_angle: 0.0,
        });

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
        });

    commands
        .spawn(Name::new("UI Circle"))
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials2d.add(Color::PURPLE.into()),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            ..default()
        });

    let gltf = assets.load("models/not-cube/not-cube.gltf#Scene0");
    commands.spawn(Name::new("Not-Cube")).insert(SceneBundle {
        scene: gltf,
        transform: Transform::from_xyz(-2.0, 0.0, -2.0).with_scale(Vec3::splat(0.25)),
        ..default()
    });
}

fn spawn_vehicle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
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

    let vehicle = commands
        .spawn(Name::new("Vehicle"))
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
    let axle_color = Color::rgb(1.0, 1.0, 1.0);

    let wheel_alignments = [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)];

    let _wheels: Vec<_> = wheel_alignments
        .into_iter()
        .map(|(x_align, z_align)| {
            let axle_offset = Vec3::new((hw + wheel_thickness) * 1.5 * x_align, 0.0, hl * z_align);
            let wheel_offset = Vec3::new((hw + wheel_thickness) * 3.0 * x_align, 0.0, hl * z_align);

            let axle_position = spawn_position + axle_offset;
            let wheel_position = spawn_position + wheel_offset;

            //Axel Joint
            let axle_joint_builder = GenericJointBuilder::new(JointAxesMask::LOCKED_FIXED_AXES)
                //.local_axis1(Vec3::X)
                //.local_axis2(-Vec3::Y)
                .local_anchor1(axle_offset)
                //.local_anchor2(Vec3::new(0.0, 0.0, 0.0))
            ;

            let axle = commands
                .spawn(Name::new(format!("Axel - {x_align} {z_align}")))
                .insert(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(axle_color.into()),
                    ..default()
                })
                .insert(SpatialBundle::from_transform(Transform::from_translation(
                    axle_position,
                )))
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(0.5, 0.5, 0.5))
                .insert(CollisionGroups::new(Group::NONE, Group::NONE))
                .insert(ImpulseJoint::new(vehicle, axle_joint_builder))
                .insert(Sleeping::disabled())
                .insert(Ccd::enabled())
                .id();

            //Wheel Joint
            let motor_max_force = 500.0;
            let wheel_joint_builder = GenericJointBuilder::new(JointAxesMask::LOCKED_REVOLUTE_AXES)
                .local_axis1(Vec3::X)
                .local_axis2(-Vec3::Y)
                .local_anchor1(wheel_offset - axle_offset)
                .local_anchor2(Vec3::new(0.0, 0.0, 0.0))
                .motor_max_force(JointAxis::AngX, motor_max_force);

            let wheel = commands
                .spawn(Name::new(format!("Wheel - {x_align} {z_align}")))
                .insert(RigidBody::Dynamic)
                .insert(Collider::cylinder(hwt, wheel_radius))
                //.insert(Restitution::coefficient(0.7))
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
                .insert(SpatialBundle::from_transform(
                    Transform::from_translation(wheel_position)
                        .with_rotation(Quat::from_rotation_z(90f32.to_radians())),
                ))
                .insert(ImpulseJoint::new(axle, wheel_joint_builder))
                .insert(WheelMotor)
                .insert(Friction {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(CollisionGroups::new(
                    WHEEL_GROUP,
                    CAR_GROUP.union(WHEEL_GROUP).not(),
                ))
                .id();

            wheel
        })
        .collect();

    vehicle
}

fn kb_motor(
    keys: Res<Input<KeyCode>>,
    mut joint_query: Query<&mut ImpulseJoint, With<WheelMotor>>,
) {
    let motor_speed = 250.0;
    let motor_factor = 1.0;

    let throttle_forward = keys.pressed(KeyCode::Space);
    let throttle_backward = keys.pressed(KeyCode::Z);

    let throttle = match (throttle_forward, throttle_backward) {
        (true, false) => motor_speed,
        (false, true) => -motor_speed,
        _ => 0.0,
    };

    for mut joint in joint_query.iter_mut() {
        joint
            .data
            .set_motor_velocity(JointAxis::AngX, throttle, motor_factor);
    }
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

//TODO Should factor out the raycast into a failable function
fn raycast(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mouse_buttons: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,

    window_query: Query<&Window, With<PrimaryWindow>>,
    mut orbital_target_query: Query<(Entity, &Handle<StandardMaterial>), With<OrbitalTarget>>,
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

    for (target_entity, cube_material_handle) in orbital_target_query.iter_mut() {
        if entity != target_entity {
            continue;
        };

        let Some(cube_material) = materials.get_mut(cube_material_handle) else { continue };
        cube_material.base_color = Color::rgb(rand::random(), rand::random(), rand::random());
    }

    return Some(());
}

fn raycast_system(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mouse_buttons: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,

    window_query: Query<&Window, With<PrimaryWindow>>,
    orbital_target_query: Query<(Entity, &Handle<StandardMaterial>), With<OrbitalTarget>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    raycast(
        camera_query,
        mouse_buttons,
        rapier_context,
        window_query,
        orbital_target_query,
        materials,
    );
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
