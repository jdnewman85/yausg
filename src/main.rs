use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

mod camera;

#[allow(dead_code)]
#[derive(Clone, Default, Component)]
enum LadderTile {
    #[default]
    Empty,
    NOContact,
    NCContact,
}

#[allow(dead_code)]
#[derive(Component)]
struct LadderTileMap {
    width: usize,
    height: usize,
    atlas: Handle<TextureAtlas>, //Should I just request this handle as needed?
    tiles: Vec<Vec<Entity>>,
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
            tiles: Vec::new(),
        }
    }
}

fn init_ladder_map_system(
    mut commands: Commands,
    mut tilemap_query: Query<(&mut LadderTileMap, Entity), Added<LadderTileMap>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    let tile = LadderTile::default();
    let index = tile.clone() as usize;
    for (mut tilemap, tm_entity) in tilemap_query.iter_mut() {
        let atlas = texture_atlases.get(&tilemap.atlas).unwrap();
        let texture = atlas.textures[index];
        let tile_size = texture.size();
        let tiles =
        (0..tilemap.height).map(|y| {
            (0..tilemap.width).map(|x| {
                let tile_entity =
                commands
                    .spawn((
                        Name::new(format!("Tile ({x},{y})")),
                        tile.clone(),
                        SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                index,
                                ..default()
                            },
                            texture_atlas: tilemap.atlas.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                (x as f32)*tile_size.x,
                                (y as f32)*tile_size.y,
                                0.0,
                            )),
                            ..default()
                        },
                    ))
                    .id();
                commands
                    .entity(tm_entity)
                    .push_children(&[tile_entity]);
                tile_entity
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
    ))
    ;
}
