use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::prelude::tess::geom;

use num_derive::FromPrimitive;

#[derive(Copy, Clone, Default, Component, Debug, FromPrimitive)]
pub enum LadderTile {
    #[default]
    Empty,
    NoContact,
    NcContact,
    NoCoil,
    NcCoil,
    Horz,
    Vert,
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
    T000,
    T090,
    T180,
    T270,
    Cross,
    _Length,
}

impl From<usize> for LadderTile {
    fn from(item: usize) -> Self {
        num::FromPrimitive::from_usize(item).unwrap()
    }
}

impl LadderTile {
    fn path_string(&self) -> String {
        match self {
            LadderTile::Empty => "",
            LadderTile::NoContact => "M 0.625,0.5 H 1.0 M 0.625,0.1875 V 0.8125 M 0.375,0.1875 V 0.8125 M 0,0.5 H 0.375",
            LadderTile::NcContact => "M 0.6875,0.25L 0.3125,0.75 M 0.375,0.5 H 0 M 0.625,0.5 H 1.0 M 0.625,0.1875 V 0.8125 M 0.375,0.1875 V 0.8125",
            LadderTile::NoCoil => "M 0.75,0.5 H 1.0 M 0.25,0.5 H 0 M 0.5625,0.75 A 0.26046875,0.26046875 0 0 1 0.5625,0.25M 0.4375,0.25A 0.26046875,0.26046875 0 0 1 0.4375,0.75",
            LadderTile::NcCoil => "M 0.6875,0.25L 0.3125,0.75 M 0.5625,0.75 A 0.26046875,0.26046875 0 0 1 0.5625,0.25M 0.4375,0.25A 0.26046875,0.26046875 0 0 1 0.4375,0.75 M 1.0,0.5 H 0.75 M 0,0.5 H 0.25",
            LadderTile::Horz => "M 0,0.5 H 1.0",
            LadderTile::Vert => "M 0.5,0 V 1.0",
            LadderTile::LeftDown => "M 0,0.5 H 0.5 V 1.0",
            LadderTile::LeftUp => "M 0,0.5 H 0.5 V 0",
            LadderTile::RightDown => "M 1.0,0.5 H 0.5 V 1.0",
            LadderTile::RightUp => "M 1.0,0.5 H 0.5 V 0",
            LadderTile::T000 => "M 0,0.5 H 1.0 M 0.5,0.5 V 1.0",
            LadderTile::T090 => "M 0.5,0 V 1.0 M 0.5,0.5 H 1.0",
            LadderTile::T180 => "M 0,0.5 H 1.0 M 0.5,0.5 V 0",
            LadderTile::T270 => "M 0.5,0 V 1.0 M 0.5,0.5 H 0",
            LadderTile::Cross => "M 0,0.5 H 1.0 M 0.5,0 V 1.0",
            LadderTile::_Length => unreachable!(),
        }.into()
    }
}

#[derive(Component)]
pub struct LadderTileMap {
    //TODO Rect, Vec2 or use tiles length?
    width: usize,
    height: usize,
    tiles: Vec<Vec<Entity>>,
}

pub type TileMapPositionalFunc = fn(
    tile: &mut LadderTile,
    position: (usize, usize),
    size: (usize, usize)
);

impl LadderTileMap {
    pub fn new(
        width: usize,
        height: usize,
    ) -> Self {
        LadderTileMap {
            width,
            height,
            tiles: default(),
        }
    }

    #[allow(dead_code)]
    pub fn apply_pos_fn(
        &self,
        func: TileMapPositionalFunc,
        tile_query: &mut Query<&mut LadderTile>,
    ) {
        self.tiles.iter().enumerate().for_each(|(x, tile_col)| {
            tile_col.iter().enumerate().for_each(|(y, entity)| {
                let mut tile = tile_query.get_mut(entity.clone()).unwrap();
                func(&mut tile, (x, y), (self.width, self.height));
            });
        });
    }
}

pub fn ladder_path_update_system(
    mut tile_query: Query<(&LadderTile, &mut Path), Changed<LadderTile>>,
) {
    for (tile, mut path) in tile_query.iter_mut() {
        *path = GeometryBuilder::build_as(&shapes::SvgPathShape {
            svg_path_string: tile.clone().path_string(),
            //svg_doc_size_in_px: Vec2::ZERO, //Vec2::new(64.0, 64.0),
            //svg_doc_size_in_px: Vec2::Y * 128.0, //TODO TEMP Attempt y offset
            //svg_doc_size_in_px: Vec2::new(64.0, 64.0),
            svg_doc_size_in_px: Vec2::ZERO,
        });
        *path = bevy_prototype_lyon::entity::Path(path.0.clone().transformed(&geom::Transform::<f32>::scale(64.0, -64.0)));
    }
}

pub fn ladder_print_system(
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

pub fn ladder_mouse_system(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse_buttons: Res<Input<MouseButton>>,
    tilemap_query: Query<(&LadderTileMap, &Transform)>,
    mut tile_query: Query<&mut LadderTile>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_viewport_position) = window.cursor_position() else { return; };
    let Some(cursor_world_position) = camera.viewport_to_world_2d(camera_transform, cursor_viewport_position) else { return; };

    for (tilemap, tilemap_transform) in tilemap_query.iter() {
        if tilemap.tiles.is_empty() { return; };
        let tile_size = Vec2::splat(64.0);

        let delta = cursor_world_position - tilemap_transform.translation.truncate();
        let tilemap_pixel_size = Vec2::new(tilemap.width as f32, tilemap.height as f32) * tile_size;

        let tilemap_position = tilemap_transform.translation.truncate();
        let tilemap_rect = Rect::from_corners(tilemap_position, tilemap_position + tilemap_pixel_size);
        if !tilemap_rect.contains(cursor_world_position) { continue; };

        let cursor_tile_x = (delta.x / tile_size.x) as usize;
        let cursor_tile_y = (delta.y / tile_size.y) as usize;

        let tile_entity = tilemap.tiles[cursor_tile_x][cursor_tile_y];
        let Ok(mut tile) = tile_query.get_mut(tile_entity) else {
            //TODO Fix
            dbg!("FIX ME:", tile_entity, cursor_tile_x, cursor_tile_y);
            return;
        };
        if mouse_buttons.just_pressed(MouseButton::Left) {
            let new_index = (*tile as usize + 1) % LadderTile::_Length as usize; //TODO Unuglify
            *tile = new_index.into();
        }

    }
}

pub fn test_clear_tilemap_system(
    input: Res<Input<KeyCode>>,
    tilemap_query: Query<&mut LadderTileMap>,
    mut tile_query: Query<&mut LadderTile>,
) {
    if !input.just_pressed(KeyCode::Key0) { return; }

    for tilemap in tilemap_query.iter() {
        tilemap.apply_pos_fn(|tile, position, size| {
            *tile = match (&tile, position, size) {
                (_, pos, size) if pos.0 == 0 || pos.0 == size.0-1 => LadderTile::Vert,
                (_, _, _) => LadderTile::Empty,
            }
        }, &mut tile_query);
    }
}

pub fn ladder_init_system(
    mut commands: Commands,
    mut tilemap_query: Query<(&mut LadderTileMap, Entity), Added<LadderTileMap>>,
) {
    for (mut tilemap, tilemap_entity) in tilemap_query.iter_mut() {
        //let tile_size = Vec2::splat(64.0);
        let tile_size = Vec2::new(64.0, 64.0);
        commands.entity(tilemap_entity)
            .with_children(|parent_tilemap| {
            tilemap.tiles =
                (0..tilemap.width).map(|x| {
                    (0..tilemap.height).map(|y| {
                        parent_tilemap.spawn((
                            Name::new(format!("Tile ({x},{y})")),
                            LadderTile::default(),
                            ShapeBundle {
                                transform: Transform::from_translation(Vec3::new(
                                    (x as f32)*tile_size.x,
                                    (y as f32)*tile_size.y, //TODO Reverse Y
                                    1.0,
                                )).with_scale(Vec3::splat(1.0)),
                                path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                                    svg_path_string: LadderTile::default().path_string(),
                                    svg_doc_size_in_px: Vec2::ZERO, //Vec2::new(64.0, 64.0),
                                    //svg_doc_size_in_px: Vec2::Y * 128.0, //TODO TEMP Attempt y offset
                                }),
                                ..default()
                            },
                            Stroke::new(Color::BLACK, 2.0),
                        ))
                            /*
                        .with_children(|parent_laddertile| {
                            parent_laddertile.spawn((
                                Text2dBundle {
                                    text: Text::from_section("C0", TextStyle {
                                        font_size: 24.0,
                                        color: Color::WHITE,
                                        ..default()
                                    }).with_alignment(TextAlignment::Center),
                                    text_anchor: bevy::sprite::Anchor::Center,
                                    transform: Transform::from_xyz(0.0, 48.0, 1.0),
                                    ..default()
                                },
                            ));
                        })
                            */
                        .id()
                    }).collect()
                }).collect()
            ;
        });
    }
}

