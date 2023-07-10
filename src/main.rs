use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

mod camera;

#[derive(Clone, Default, Component)]
enum LadderTile {
    #[default]
    Empty,
}

#[allow(dead_code)]
#[derive(Component, Default)]
struct LadderTileMap {
    width: usize,
    height: usize,

    tiles: Vec<Vec<Entity>>,
}

#[allow(dead_code)]
impl LadderTileMap {
    fn new(
        width: usize,
        height: usize,
    ) -> Self {

        LadderTileMap {
            width,
            height,
            ..default()
        }
    }
}

fn init_ladder_map_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials2d: ResMut<Assets<ColorMaterial>>,
    mut tilemap_query: Query<&mut LadderTileMap, Added<LadderTileMap>>,
    ) {
    for mut tilemap in tilemap_query.iter_mut() {
        //TODO Tile Size
        let tile_size = 16.0;
        let tiles =
        (0..tilemap.height).map(|y| {
            (0..tilemap.width).map(|x| {
                commands
                    .spawn(Name::new(format!("Tile ({x},{y})")))
                    .insert(MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                        material: materials2d.add(Color::PURPLE.into()),
                        transform: Transform::from_translation(Vec3::new(
                            (x as f32)*tile_size,
                            (y as f32)*tile_size,
                            0.0,
                        )),
                        ..default()
                    })
                    .id()
            }).collect()
        }).collect();

        tilemap.tiles = tiles;
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(camera::orbital_camera_system)
        .add_system(init_ladder_map_system)
        //.add_system(camera::god_mode_camera_system)
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
        .insert(Collider::cuboid(50.0, 0.001, 50.0))
        .insert(Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(camera::OrbitalTarget)
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
        .insert(camera::OrbitCamera {
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

    let tilemap = LadderTileMap::new(16, 16);
    commands
        .spawn(tilemap)
    ;
}
