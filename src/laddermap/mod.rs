use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy_prototype_lyon::prelude::*;

use num_derive::FromPrimitive;

#[derive(Clone, Copy, Default, Debug)]
#[derive(FromPrimitive)]
pub enum Wire {
    #[default]
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

impl Wire {
    fn path_string(&self) -> String {
        match self {
            Self::Horz => "M 0,0.5 H 1.0",
            Self::Vert => "M 0.5,0 V 1.0",
            Self::LeftDown => "M 0,0.5 H 0.5 V 1.0",
            Self::LeftUp => "M 0,0.5 H 0.5 V 0",
            Self::RightDown => "M 1.0,0.5 H 0.5 V 1.0",
            Self::RightUp => "M 1.0,0.5 H 0.5 V 0",
            Self::T000 => "M 0,0.5 H 1.0 M 0.5,0.5 V 1.0",
            Self::T090 => "M 0.5,0 V 1.0 M 0.5,0.5 H 1.0",
            Self::T180 => "M 0,0.5 H 1.0 M 0.5,0.5 V 0",
            Self::T270 => "M 0.5,0 V 1.0 M 0.5,0.5 H 0",
            Self::Cross => "M 0,0.5 H 1.0 M 0.5,0 V 1.0",
            Self::_Length => unreachable!(),
        }.into()
    }
    fn scroll(&mut self, x: f32) {
        let len = Self::_Length as i32;
        let change = x.round() as i32;
        let delta_index = *self as i32 + change;
        let index = (delta_index + len) % len;
        *self = num::FromPrimitive::from_i32(index).unwrap()
    }
}

#[derive(Clone, Copy, Default, Debug)]
enum Polarity {
    #[default]
    NO,
    NC,
}

impl Polarity {
    fn invert(&mut self) {
        *self = match *self {
            Polarity::NO => Polarity::NC,
            Polarity::NC => Polarity::NO,
        };
    }
}

#[derive(Clone, Copy, Default, Debug)]
enum ContactOrCoil {
    #[default]
    Contact,
    Coil,
}
#[derive(Clone, Default, Debug)]
pub struct BoolElement {
    contact_or_coil: ContactOrCoil,
    address: String,
    polarity: Polarity,
}

impl BoolElement {
    fn path_string(&self) -> String {
        match (self.contact_or_coil, self.polarity) {
            (ContactOrCoil::Contact, Polarity::NO) =>"M 0.625,0.5 H 1.0 M 0.625,0.1875 V 0.8125 M 0.375,0.1875 V 0.8125 M 0,0.5 H 0.375",
            (ContactOrCoil::Contact, Polarity::NC) => "M 0.6875,0.25L 0.3125,0.75 M 0.375,0.5 H 0 M 0.625,0.5 H 1.0 M 0.625,0.1875 V 0.8125 M 0.375,0.1875 V 0.8125",
            (ContactOrCoil::Coil, Polarity::NO) => "M 0.75,0.5 H 1.0 M 0.25,0.5 H 0 M 0.5625,0.75 A 0.26046875,0.26046875 0 0 1 0.5625,0.25M 0.4375,0.25A 0.26046875,0.26046875 0 0 1 0.4375,0.75",
            (ContactOrCoil::Coil, Polarity::NC) => "M 0.6875,0.25L 0.3125,0.75 M 0.5625,0.75 A 0.26046875,0.26046875 0 0 1 0.5625,0.25M 0.4375,0.25A 0.26046875,0.26046875 0 0 1 0.4375,0.75 M 1.0,0.5 H 0.75 M 0,0.5 H 0.25",
        }.into()
    }
}


#[derive(Clone, Default, Debug)]
#[derive(Component)]
pub enum Tile {
    #[default]
    None,
    Contact(BoolElement),
    Coil(BoolElement),
    Wire(Wire),
}

impl Tile {
    fn path_string(&self) -> String {
        match self {
            Self::None => "".into(),
            Self::Contact(contact) => contact.path_string(),
            Self::Coil(coil) => coil.path_string(),
            Self::Wire(wire) => wire.path_string(),
        }
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
    tile: &mut Tile,
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

    pub fn apply_pos_fn(
        &self,
        func: TileMapPositionalFunc,
        tile_query: &mut Query<&mut Tile>,
    ) {
        self.tiles.iter().enumerate().for_each(|(x, tile_col)| {
            tile_col.iter().enumerate().for_each(|(y, entity)| {
                let mut tile = tile_query.get_mut(entity.clone()).unwrap();
                func(&mut tile, (x, y), (self.width, self.height));
            });
        });
    }
}

pub fn ladder_tile_update_system(
    mut commands: Commands,
    mut tile_query: Query<(Entity, &Tile, &mut Path, Option<&Children>), Changed<Tile>>,
    mut label_query: Query<(Entity, &mut Text), With<Parent>>,
) {
    for (tile_entity, tile, mut path, maybe_children) in tile_query.iter_mut() {
        *path = GeometryBuilder::build_as(&shapes::SvgPathShape {
            svg_path_string: tile.clone().path_string(),
            //svg_doc_size_in_px: Vec2::new(-1.0, 1.0),
            svg_doc_size_in_px: Vec2::ZERO,
        });
        *path = bevy_prototype_lyon::entity::Path(
            path.0.clone().transformed(
                &tess::geom::Transform::<f32>::scale(64.0, -64.0)
            )
        );

        let label_text = match tile {
            Tile::None |
            Tile::Wire(_) => "".to_string(),
            Tile::Contact(bool_element) |
            Tile::Coil(bool_element) => bool_element.address.clone(),
        };
        let should_have_label = !label_text.is_empty();

        let style = TextStyle {
            font_size: 24.0,
            color: Color::BLACK,
            ..default()
        };
        let new_label_text = Text::from_section(label_text, style.clone())
            .with_alignment(TextAlignment::Center);

        let mut has_label = false;
        if let Some(children) = maybe_children {
            for (label_entity, mut text) in label_query.iter_mut() {
                let children_has_label = children.contains(&label_entity);
                match (should_have_label, children_has_label) {
                    (false, false) => (),
                    (false, true) => commands.entity(label_entity).despawn(),
                    (true, false) => (),
                    (true, true) => {
                        *text = new_label_text.clone();
                        has_label = true;
                    }
                }
            }
        }

        if should_have_label && !has_label {
            commands.entity(tile_entity)
                .with_children(|parent_laddertile| {
                    parent_laddertile.spawn((
                        Text2dBundle {
                            text: new_label_text.clone(),
                            text_anchor: bevy::sprite::Anchor::Center,
                            transform: Transform::from_xyz(32.0, 64.0, 1.0),
                            ..default()
                        },
                    ));
                });
        }
    }
}

pub fn ladder_print_system(
    input: Res<Input<KeyCode>>,
    tilemap_query: Query<(&LadderTileMap, &Name)>,
    tile_query: Query<&Tile>,
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
    mut scroll_events: EventReader<MouseWheel>,
    tilemap_query: Query<(&LadderTileMap, &Transform)>,
    mut tile_query: Query<&mut Tile>,
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

        //TODO impl further mouse interface
        if mouse_buttons.just_pressed(MouseButton::Left) {
            let is_coil_column = cursor_tile_x == tilemap.width-1;
            let contact_or_coil = match is_coil_column {
                false => ContactOrCoil::Contact,
                true => ContactOrCoil::Coil,
            };

            *tile = Tile::Contact(BoolElement {
                contact_or_coil,
                address: "X0".into(),
                polarity: Polarity::NO,
            });
        }

        if mouse_buttons.just_pressed(MouseButton::Right) {
            let (is_none, is_wire) = match *tile {
                Tile::None => (true, false),
                Tile::Wire(_) => (false, true),
                _ => (false, false),
            };
            let is_coil_column = cursor_tile_x == tilemap.width-1;

            *tile = match (is_none, is_wire, is_coil_column) {
                (false, _    , _     ) => Tile::None,
                (true , _    , true  ) => tile.clone(), //TODO Opt, cloning self
                (true , false, false ) => Tile::Wire(Wire::default()),
                (true , true , false ) => Tile::None,
            };
        }

        for event in scroll_events.iter() {
            //TODO handle each event.unit differently
            //TODO handle scroll values

            match *tile {
                Tile::None => { },
                Tile::Contact(ref mut bool_element) |
                Tile::Coil(ref mut bool_element) => bool_element.polarity.invert(),
                Tile::Wire(ref mut wire) => wire.scroll(event.y),
            }
        }

    }
}

pub fn test_clear_tilemap_system(
    input: Res<Input<KeyCode>>,
    tilemap_query: Query<&mut LadderTileMap>,
    mut tile_query: Query<&mut Tile>,
) {
    if !input.just_pressed(KeyCode::Key0) { return; }

    for tilemap in tilemap_query.iter() {
        tilemap.apply_pos_fn(|tile, position, size| {
            *tile = match (&tile, position, size) {
                (_, pos, size) if pos.0 == 0 || pos.0 == size.0-1 => Tile::Wire(Wire::Vert),
                (_, _, _) => Tile::None,
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
                            Tile::default(),
                            ShapeBundle {
                                transform: Transform::from_translation(Vec3::new(
                                    (x as f32)*tile_size.x,
                                    (y as f32)*tile_size.y,
                                    1.0,
                                )).with_scale(Vec3::splat(1.0)),
                                path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                                    svg_path_string: Tile::default().path_string(),
                                    svg_doc_size_in_px: Vec2::ZERO, //Vec2::new(64.0, 64.0),
                                }),
                                ..default()
                            },
                            Stroke::new(Color::BLACK, 2.0),
                        ))
                        .id()
                    }).collect()
                }).collect()
            ;
        });
    }
}

