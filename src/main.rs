use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::MaterialMesh2dBundle, render::view::screenshot::ScreenshotManager, window::PrimaryWindow,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

mod camera;

use num_derive::FromPrimitive;

#[derive(Copy, Clone, Default, Component, Debug, FromPrimitive)]
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
    T000,
    T090,
    T180,
    T270,
    Cross,
    _Length,
}

impl LadderTile {
    fn texture_filename(&self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::NoContact => "NO-Contact",
            Self::NcContact => "NC-Contact",
            Self::NoCoil => "NO-Coil",
            Self::NcCoil => "NC-Coil",
            Self::Horz => "Horz",
            Self::Vert => "Vert",
            Self::BR => "BR",
            Self::BL => "BL",
            Self::UR => "UR",
            Self::UL => "UL",
            Self::T000 => "T-000",
            Self::T090 => "T-090",
            Self::T180 => "T-180",
            Self::T270 => "T-270",
            Self::Cross => "Cross",
            Self::_Length => unreachable!(),
        }
    }
}

#[derive(Component)]
struct LadderTileMap {
    //TODO Rect, Vec2 or use tiles length?
    width: usize,
    height: usize,
    tile_images: Vec<Handle<Image>>,
    tiles: Vec<Vec<Entity>>,
}

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

    fn load_tile_images(&mut self, asset_server: &Res<AssetServer>) {
        self.tile_images = (0..LadderTile::_Length as usize)
            .map(|tile_variant| num::FromPrimitive::from_usize(tile_variant).unwrap())
            .map(|tile: LadderTile| tile.texture_filename())
            .map(|tile_filename| format!("./textures/{tile_filename}.png"))
            .map(|full_path| asset_server.load(full_path).into())
            .collect();
    }
}

fn ladder_image_update_system(
    tilemap_query: Query<&LadderTileMap>, //TODO Opt, maybe store the entire handle vec in each tile? :(
    mut tile_query: Query<(&LadderTile, &mut Handle<Image>, &Parent), Changed<LadderTile>>,
) {
    for (tile, mut image_handle, parent) in tile_query.iter_mut() {
        let tilemap = tilemap_query.get(parent.get()).unwrap();
        *image_handle = tilemap.tile_images[tile.clone() as usize].clone();
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

fn ladder_mouse_system(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse_buttons: Res<Input<MouseButton>>,
    tilemap_query: Query<(&LadderTileMap, &Transform)>,
    textures: Res<Assets<Image>>,
    mut tile_query: Query<&mut LadderTile>
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_viewport_position) = window.cursor_position() else { return; };
    let Some(cursor_world_position) = camera.viewport_to_world_2d(camera_transform, cursor_viewport_position) else { return; };

    for (tilemap, tilemap_transform) in tilemap_query.iter() {
        //TODO TEMP?
        //Highlight under cursor

        let empty_texture = textures.get(&tilemap.tile_images[LadderTile::Empty as usize]).unwrap();

        let delta = cursor_world_position - tilemap_transform.translation.truncate();
        let tilemap_pixel_size = Vec2::new(tilemap.width as f32, tilemap.height as f32) * empty_texture.size();

        let tilemap_position = tilemap_transform.translation.truncate();
        let tilemap_rect = Rect::from_corners(
            tilemap_position,
            tilemap_position + tilemap_pixel_size
        );
        if !tilemap_rect.contains(cursor_world_position) { continue; };

        let cursor_tile_x = (delta.x / empty_texture.size().x) as usize;
        let cursor_tile_y = (delta.y / empty_texture.size().y) as usize;

        let tile_entity = tilemap.tiles[cursor_tile_x][cursor_tile_y];
        let mut tile = tile_query.get_mut(tile_entity).unwrap();

        if mouse_buttons.just_pressed(MouseButton::Left) {
            let new_index = (*tile as usize + 1) % LadderTile::_Length as usize; //TODO Unuglify
            let new_tile: LadderTile = num::FromPrimitive::from_usize(new_index).unwrap();
            *tile = new_tile;
        }
    }
}

fn ladder_init_system(
    mut commands: Commands,
    mut tilemap_query: Query<(&mut LadderTileMap, Entity), Added<LadderTileMap>>,
    textures: Res<Assets<Image>>,
) {
    let empty_tile = LadderTile::default();
    let empty_texture_index = empty_tile.clone() as usize;
    for (mut tilemap, tilemap_entity) in tilemap_query.iter_mut() {
        let empty_texture_handle = tilemap.tile_images[empty_texture_index].clone();
        let texture = textures.get(&empty_texture_handle).unwrap();
        let tile_size = texture.size();
        commands.entity(tilemap_entity).with_children(|parent_tilemap| {
            tilemap.tiles =
                (0..tilemap.width).map(|x| {
                    (0..tilemap.height).map(|y| {
                        parent_tilemap.spawn((
                            Name::new(format!("Tile ({x},{y})")),
                            empty_tile.clone(),
                            SpriteBundle {
                                texture: empty_texture_handle.clone(),
                                sprite: Sprite {
                                    anchor: bevy::sprite::Anchor::BottomLeft, //TODO Different anchors
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(
                                    (x as f32)*tile_size.x,
                                    (y as f32)*tile_size.y, //TODO Reverse Y
                                    0.0,
                                )),
                                ..default()
                            },
                        )).id()
                    }).collect()
                }).collect()
            ;
        });
    }
}

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
            ladder_init_system,
            ladder_mouse_system,
            //camera::god_mode_camera_system,
            screenshot_on_spacebar,
            ladder_print_system,
            ladder_image_update_system,
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

    let mut tilemap = LadderTileMap::new(10, 10);
    tilemap.load_tile_images(&asset_server); //TODO Put in init of tilemap
    commands.spawn((
        tilemap,
        SpatialBundle {
            transform: Transform::from_xyz(50.0, 50.0, 0.0),
            ..default()
        },
    ));
}
