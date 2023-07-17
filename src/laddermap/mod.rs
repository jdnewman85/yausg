use bevy::prelude::*;

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

impl From<usize> for LadderTile {
    fn from(item: usize) -> Self {
        num::FromPrimitive::from_usize(item).unwrap()
    }
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
pub struct LadderTileMap {
    //TODO Rect, Vec2 or use tiles length?
    width: usize,
    height: usize,
    tile_images: Vec<Handle<Image>>,
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
            tile_images: default(),
            tiles: default(),
        }
    }

    pub fn load_tile_images(&mut self, asset_server: &Res<AssetServer>) {
        self.tile_images = (0..LadderTile::_Length as usize)
            .map(|tile_variant| LadderTile::from(tile_variant))
            .map(|tile| tile.texture_filename())
            .map(|tile_filename| format!("./textures/{tile_filename}.png"))
            .map(|full_path| asset_server.load(full_path).into())
            .collect();
    }

    /*
    fn load_tile_images_2(&mut self, asset_server: &Res<AssetServer>) {
        let tile_length = LadderTile::_Length as usize;
        let tile_range = 0..tile_length;
        for tile_variant in tile_range {
            let tile: LadderTile = tile_variant.into();
            let full_path = format!("./textures/{}.png", tile.texture_filename());
            self.tile_images.push(asset_server.load(full_path).into());
        }
    }
    */
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

pub fn ladder_image_update_system(
    tilemap_query: Query<&LadderTileMap>, //TODO Opt, maybe store the entire handle vec in each tile? :(
    mut tile_query: Query<(&LadderTile, &mut Handle<Image>, &Parent), Changed<LadderTile>>,
) {
    for (tile, mut image_handle, parent) in tile_query.iter_mut() {
        let tilemap = tilemap_query.get(parent.get()).unwrap();
        *image_handle = tilemap.tile_images[tile.clone() as usize].clone();
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
    textures: Res<Assets<Image>>,
    mut tile_query: Query<&mut LadderTile>,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_viewport_position) = window.cursor_position() else { return; };
    let Some(cursor_world_position) = camera.viewport_to_world_2d(camera_transform, cursor_viewport_position) else { return; };

    for (tilemap, tilemap_transform) in tilemap_query.iter() {
        let empty_texture = textures.get(&tilemap.tile_images[LadderTile::Empty as usize]).unwrap();

        let delta = cursor_world_position - tilemap_transform.translation.truncate();
        let tilemap_pixel_size = Vec2::new(tilemap.width as f32, tilemap.height as f32) * empty_texture.size();

        let tilemap_position = tilemap_transform.translation.truncate();
        let tilemap_rect = Rect::from_corners(tilemap_position, tilemap_position + tilemap_pixel_size);
        if !tilemap_rect.contains(cursor_world_position) { continue; };

        let cursor_tile_x = (delta.x / empty_texture.size().x) as usize;
        let cursor_tile_y = (delta.y / empty_texture.size().y) as usize;

        let tile_entity = tilemap.tiles[cursor_tile_x][cursor_tile_y];
        let mut tile = tile_query.get_mut(tile_entity).unwrap();

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

