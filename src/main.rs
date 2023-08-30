#![allow(dead_code)]
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

mod camera;
mod laddermap;
mod utils;
mod vladder;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                //.set(ImagePlugin::default_nearest())
            ,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
            ShapePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            camera::orbital_camera_system,
            //camera::god_mode_camera_system,
            utils::screenshot_on_spacebar,
            laddermap::ladder_init_system,
            laddermap::ladder_print_system,
            laddermap::ladder_tile_path_update_system,
            laddermap::test_clear_tilemap_system,
            laddermap::tile_label_reference_system,
            laddermap::ladder_tile_label_update_system,

            laddermap::ladder_tile_mouse_system,
            laddermap::tilemap_mouse_position_system,
            laddermap::tilemap_cursor_system,
            laddermap::tilemap_cursor_removal_system,
            laddermap::tile_highlight_system,
            laddermap::ladder_tile_highlight_system,
            laddermap::ladder_tile_unhighlight_system,
            laddermap::ladder_tile_focus_highlight_system,
            laddermap::ladder_tile_focus_unhighlight_system,
        ))
        //.insert_resource(Msaa::Off)
        .register_type::<vladder::InputModule>()
        .register_type::<vladder::OutputModule>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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


    //TODO Move to tilemap init function
    let tilemap = laddermap::LadderTileMap::new(UVec2::new(8, 8));
    let tilemap_pixel_size = tilemap.pixel_size();
    let tile_map_path = format!("M 0,0 H {} V {} H 0 Z", tilemap_pixel_size.x, tilemap_pixel_size.y);
    commands.spawn((
        tilemap,
        ShapeBundle {
            transform: Transform::from_xyz(-200.0, 0.0, 0.0),
            path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                svg_path_string: tile_map_path,
                svg_doc_size_in_px: Vec2::Y * (tilemap_pixel_size.y * 2.0), //TODO HACK Invert Y
            }),
            ..default()
        },
        Stroke::new(Color::BLACK, 1.0),
        Fill::color(Color::WHITE),

        vladder::DebugCpuModule::new(8, 8),
    ));
}

