use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

mod camera;

use num_derive::FromPrimitive;

#[allow(dead_code)]
#[derive(Clone, Default, Component)]
#[derive(FromPrimitive)]
enum LadderTile {
    #[default]
    Empty,
    NOContact,
    NCContact,
    _Length,
}

#[allow(dead_code)]
#[derive(Component)]
struct LadderTileMap {
    //TODO Rect, Vec2 or use tiles length?
    width: usize,
    height: usize,
    atlas: Handle<TextureAtlas>, //TODO Should I just request this handle as needed?
    tiles: Vec<Vec<Entity>>,
    selection: Option<(usize, usize)>,
}

#[allow(dead_code)]
impl LadderTileMap {
    fn new(
        width: usize,
        height: usize,
        atlas: Handle<TextureAtlas>,
    ) -> Self {
        LadderTileMap {
            width,
            height,
            atlas,
            tiles: default(),
            selection: None,
        }
    }
}

fn ladder_click_system(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse_buttons: Res<Input<MouseButton>>,
    tilemap_query: Query<(&LadderTileMap, &Transform)>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut tile_query: Query<(&mut LadderTile, &mut TextureAtlasSprite)>
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) { return; };
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_viewport_position) = window.cursor_position() else { return; };
    let Some(cursor_world_position) = camera.viewport_to_world_2d(camera_transform, cursor_viewport_position) else { return; };

    for (tilemap, tilemap_transform) in tilemap_query.iter() {
        let texture_atlas = texture_atlases.get(&tilemap.atlas).unwrap();
        //TODO: By using LadderTile::Empty as a constant index,
        //we only ever check the first texture,
        //assuming all textures in the atlas have the same size
        let texture_rect = texture_atlas.textures[LadderTile::Empty as usize];

        let delta = cursor_world_position - tilemap_transform.translation.truncate();
        let tilemap_pixel_size = Vec2::new(tilemap.width as f32, tilemap.height as f32) * texture_rect.size();

        let tilemap_position = tilemap_transform.translation.truncate();
        let tilemap_rect = Rect::from_corners(
            tilemap_position,
            tilemap_position + tilemap_pixel_size
        );
        if !tilemap_rect.contains(cursor_world_position) { continue; };

        let cursor_tile_x = (delta.x / texture_rect.width()) as usize;
        let cursor_tile_y = (delta.y / texture_rect.height()) as usize;

        let tile_entity = tilemap.tiles[cursor_tile_x][cursor_tile_y];
        let (mut tile, mut sprite) = tile_query.get_mut(tile_entity).unwrap();
        let new_index = (sprite.index + 1) % LadderTile::_Length as usize;
        let new_tile: LadderTile = num::FromPrimitive::from_usize(new_index).unwrap();
        sprite.index = new_index;
        *tile = new_tile;
    }
}

fn init_ladder_map_system(
    mut commands: Commands,
    mut tilemap_query: Query<(&mut LadderTileMap, Entity), Added<LadderTileMap>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let tile = LadderTile::default();
    let index = tile.clone() as usize;
    for (mut tilemap, tilemap_entity) in tilemap_query.iter_mut() {
        let atlas = texture_atlases.get(&tilemap.atlas).unwrap();
        let texture = atlas.textures[index];
        let tile_size = texture.size();
        commands.entity(tilemap_entity).with_children(|parent_tilemap| {
            tilemap.tiles =
                (0..tilemap.width).map(|x| {
                    (0..tilemap.height).map(|y| {
                        parent_tilemap.spawn((
                            Name::new(format!("Tile ({x},{y})")),
                            tile.clone(),
                            SpriteSheetBundle {
                                sprite: TextureAtlasSprite {
                                    index,
                                    anchor: bevy::sprite::Anchor::BottomLeft, //TODO Different anchors
                                    ..default()
                                },
                                texture_atlas: tilemap.atlas.clone(),
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
//            camera::god_mode_camera_system,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials2d: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    let tilemap_texture = asset_server.load("./textures/simple_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(tilemap_texture, Vec2::new(32.0, 32.0), 3, 1, None, None);
    let atlas_handle = texture_atlases.add(texture_atlas);
    let tilemap = LadderTileMap::new(10, 10, atlas_handle);
    commands.spawn((
        tilemap,
        SpatialBundle {
            transform: Transform::from_xyz(50.0, 50.0, 0.0),
            ..default()
        },
    ));
}
