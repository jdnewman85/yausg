use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

mod camera;
mod laddermap;
mod utils;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                //.set(ImagePlugin::default_nearest())
            ,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new()
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            camera::orbital_camera_system,
            //camera::god_mode_camera_system,
            utils::screenshot_on_spacebar,
            laddermap::ladder_init_system,
            laddermap::ladder_mouse_system,
            laddermap::ladder_print_system,
            laddermap::ladder_image_update_system,
        ))
        //.insert_resource(Msaa::Off)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials2d: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //Clear color
    commands.insert_resource(ClearColor(Color::CYAN));

    //Light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.10,
    });
    commands.spawn((
        Name::new("Spotlight"),
        SpotLightBundle {
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
        }
    ));
    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLightBundle {
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
        }
    ));
    commands.spawn((
        Name::new("Ground Plane"),
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(100.0))),
            material: materials.add(Color::rgb(0.7, 0.9, 0.7).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        Collider::cuboid(50.0, 0.001, 50.0),
        Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        camera::OrbitalTarget,
    ));
    commands.spawn((
        Name::new("The Cube"),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Restitution::coefficient(0.7),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.4, 0.4, 1.0).into()),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        Name::new("3D Camera"),
        Camera3dBundle {
            camera: Camera {
                order: 0,
                ..default()
            },
            transform: Transform::from_xyz(1.0, 1.0, 1.0),
            ..Default::default()
        },
        //GodModeCamera {},
        camera::OrbitCamera {
            distance: 25.0,
            y_angle: 0.0,
        },
    ));
    commands.spawn((
        Name::new("UI Camera"),
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..default()
        },
    ));
    commands.spawn((
        Name::new("UI Circle"),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials2d.add(Color::PURPLE.into()),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            ..default()
        },
    ));

    let mut tilemap = laddermap::LadderTileMap::new(10, 10);
    tilemap.load_tile_images(&asset_server); //TODO Put in init of tilemap
    commands.spawn((
        tilemap,
        SpatialBundle {
            transform: Transform::from_xyz(50.0, 50.0, 0.0),
            ..default()
        },
    ));
}
