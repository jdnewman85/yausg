use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::MaterialMesh2dBundle, ui::RelativeCursorPosition, window::PrimaryWindow, render::view::screenshot::ScreenshotManager,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

mod camera;

use num_derive::FromPrimitive;

#[allow(dead_code)]
#[derive(Copy, Clone, Default, Component, Debug)]
#[derive(FromPrimitive)]
enum LadderTile {
    #[default]
    Empty,
    NoContact,
    NcContact,
    NoCoil,
    NcCoil,
    Horz,
    Vert,
    BR,
    BL,
    UR,
    UL,
    Cross,
    T000,
    T090,
    T180,
    T270,
    _Length,
}

#[allow(dead_code)]
#[derive(Component)]
struct LadderTileMap {
    //TODO Rect, Vec2 or use tiles length?
    width: usize,
    height: usize,
    tile_images: Vec<UiImage>,
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
            tile_images: default(),
            tiles: default(),
        }
    }
}

fn init_ladder_map_system(
    mut commands: Commands,
    mut tilemap_query: Query<(&mut LadderTileMap, Entity), Added<LadderTileMap>>,
) {
    let tile = LadderTile::default();
    let index = tile.clone() as usize;

    for (mut tilemap, tilemap_entity) in tilemap_query.iter_mut() {
        if tilemap.tile_images.len() <= index {
            todo!("Error, default tile index out of bounds for tilemap_images");
        };
        commands
            .entity(tilemap_entity)
            .insert((
                NodeBundle {
                    style: Style {
                        width: Val::Px(320.0),
                        height: Val::Px(320.0),
                        position_type: PositionType::Absolute,
                        display: Display::Grid,
                        grid_template_rows: RepeatedGridTrack::flex(10, 1.0),
                        grid_template_columns: RepeatedGridTrack::flex(10, 1.0),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 1.0).into(),
                    ..default()
                },
                //UiImage::new(asset_server.load("./textures/simple.png")),
            ))
            .with_children(|parent_tilemap| {
                tilemap.tiles =
                    (0..tilemap.width).map(|x| {
                        (0..tilemap.height).map(|y| {
                            parent_tilemap.spawn((
                                Name::new(format!("Tile ({x},{y})")),
                                tile.clone(),
                                NodeBundle {
                                    background_color: Color::rgba(1.0, 1.0, 1.0, 1.0).into(),
                                    ..default()
                                },
                                tilemap.tile_images[index].clone(),
                                RelativeCursorPosition::default(),
                                Interaction::default(),
                            )).id()
                        }).collect()
                    }).collect()
                ;
            });
    }
}


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new()
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            camera::orbital_camera_system,
            init_ladder_map_system,
            ladder_click_system,
            screenshot_on_spacebar,
            //camera::god_mode_camera_system,
            ladder_print_system,
            tile_mouse_over_highlight_system,
        ))
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
                ..default() },
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

    let mut tilemap = LadderTileMap::new(10, 10);
    tilemap.tile_images = vec![
        "./textures/Empty.png",
        "./textures/NO-Contact.png",
        "./textures/NC-Contact.png",
        "./textures/NO-Coil.png",
        "./textures/NC-Coil.png",
        "./textures/Horz.png",
        "./textures/Vert.png",
        "./textures/BR.png",
        "./textures/BL.png",
        "./textures/UR.png",
        "./textures/UL.png",
        "./textures/Cross.png",
        "./textures/T-000.png",
        "./textures/T-090.png",
        "./textures/T-180.png",
        "./textures/T-270.png",
    ].iter().map(|filename| {
        asset_server.load(*filename).into()
    }).collect();
    commands.spawn((
        Name::new("Tilemap A"),
        tilemap,
        SpatialBundle {
            transform: Transform::from_xyz(50.0, 50.0, 0.0),
            ..default()
        },
    ));
}

fn ladder_click_system(
    mouse_buttons: Res<Input<MouseButton>>,
    tilemap_query: Query<&LadderTileMap>,
    mut tile_query: Query<(&mut LadderTile, &mut UiImage, &RelativeCursorPosition, &Parent)>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) { return; };

    for (mut tile, mut ui_image, rel_cursor_pos, parent) in tile_query.iter_mut() {
        if rel_cursor_pos.mouse_over() {
            let new_tile_index = (*tile as usize + 1) % LadderTile::_Length as usize;
            *tile = num::FromPrimitive::from_usize(new_tile_index).unwrap();
            let parent_tilemap = tilemap_query.get(parent.get()).unwrap();
            let new_image = parent_tilemap.tile_images[new_tile_index].clone();
            *ui_image = new_image;
            return;
        }
    }
}

fn ladder_print_system(
    input: Res<Input<KeyCode>>,
    tilemap_query: Query<(&LadderTileMap, &Name)>,
    tile_query: Query<&LadderTile>,
) {
    if !input.just_pressed(KeyCode::L) { return; }
    for (tilemap, name) in tilemap_query.iter() {
        println!("Tilemap: {name}");
        for (x, col) in tilemap.tiles.iter().enumerate() {
            for (y, tile_entity) in col.iter().enumerate() {
                let tile = tile_query.get(*tile_entity).unwrap();
                println!("\tTile @ ({x}, {y}) == {tile:?}")
            }
        }
    }
}

fn tile_mouse_over_highlight_system(
    mut tile_query: Query<(&mut BackgroundColor, &Interaction), (With<LadderTile>, Changed<Interaction>)>,
) {
    for (mut background_color, interaction) in tile_query.iter_mut() {
        *background_color = if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            Color::rgb(0.0, 0.5, 0.0).into()
        } else {
            Color::rgb(1.0, 1.0, 1.0).into()
        };
    }
}

//from bevy examples
fn screenshot_on_spacebar(
    input: Res<Input<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if !input.just_pressed(KeyCode::P) { return; }
    let path = format!("./screenshot-{}.png", *counter);
    *counter += 1;
    screenshot_manager.save_screenshot_to_disk(main_window.single(), path).unwrap();
}
