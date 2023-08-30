use std::ops::Deref;
use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy_prototype_lyon::prelude::*;

use num_derive::FromPrimitive;

use crate::vladder::DebugCpuModule;

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
    //TODO TEMP
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
#[derive(Clone,  Debug)]
pub struct BoolElement {
    contact_or_coil: ContactOrCoil,
    address: String,
    polarity: Polarity,
}

impl BoolElement {
    //TODO Trait?
    fn path_string(&self) -> String {
        match (self.contact_or_coil, self.polarity) {
            (ContactOrCoil::Contact, Polarity::NO) => "M 0.625,0.5 H 1.0 M 0.625,0.1875 V 0.8125 M 0.375,0.1875 V 0.8125 M 0,0.5 H 0.375",
            (ContactOrCoil::Contact, Polarity::NC) => "M 0.6875,0.25L 0.3125,0.75 M 0.375,0.5 H 0 M 0.625,0.5 H 1.0 M 0.625,0.1875 V 0.8125 M 0.375,0.1875 V 0.8125",
            (ContactOrCoil::Coil, Polarity::NO) => "M 0.75,0.5 H 1.0 M 0.25,0.5 H 0 M 0.5625,0.75 A 0.26046875,0.26046875 0 0 1 0.5625,0.25M 0.4375,0.25A 0.26046875,0.26046875 0 0 1 0.4375,0.75",
            (ContactOrCoil::Coil, Polarity::NC) => "M 0.6875,0.25L 0.3125,0.75 M 0.5625,0.75 A 0.26046875,0.26046875 0 0 1 0.5625,0.25M 0.4375,0.25A 0.26046875,0.26046875 0 0 1 0.4375,0.75 M 1.0,0.5 H 0.75 M 0,0.5 H 0.25",
        }.into()
    }
}

#[derive(Component)]
pub struct TileLabelRef(Entity);

#[derive(Component)]
pub struct TileLabel;

//TODO Are these that useful?
//TODO Can it be a derive marco?
impl Deref for TileLabelRef {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component)]
pub struct FocusedRef(Entity);

#[derive(Component)]
pub struct Focused;

impl Deref for FocusedRef {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Component)]
pub struct HoveredRef(Entity);

#[derive(Component)]
pub struct Hovered;

impl Deref for HoveredRef {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component)]
pub struct TileMapCursorRef(Entity);

#[derive(Component)]
pub struct TileMapCursor;

#[derive(Component)]
pub struct MouseTilePosition(UVec2);

impl Deref for TileMapCursorRef {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Component)]
pub struct TilePosition(UVec2);

impl Deref for TilePosition {
    type Target = UVec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//TODO
const Z_ORDER_SPRITE: f32 = 1.0;
const Z_ORDER_CURSOR: f32 = 0.1;

//TODO
const TILE_SIZE: Vec2 = Vec2::new(64.0, 64.0);

#[derive(Clone, Default, Debug)]
#[derive(Component)]
pub enum Tile {
    #[default]
    None,
    BoolElement(BoolElement),
    Wire(Wire),
}

impl Tile {
    //TODO trait?
    fn path_string(&self) -> String {
        match self {
            Self::None => "".into(), //TODO Return Option<String>?
            Self::BoolElement(bool_element) => bool_element.path_string(),
            Self::Wire(wire) => wire.path_string(),
        }
    }

    fn label_string(&self) -> String {
        match self {
            Self::None |
            Self::Wire(_) => "".to_string(),
            Self::BoolElement(bool_element) => bool_element.address.clone(),
        }
    }
}

#[derive(Component)]
pub struct LadderTileMap {
    size: UVec2,
    tiles: Vec<Vec<Entity>>, //TODO Opt? Single vec? For grid, but maybe not for later rung based
}

pub type TileMapPositionalFunc = fn(
    tile: &mut Tile,
    position: UVec2,
    size: UVec2,
);

impl LadderTileMap {
    pub fn new(
        size: UVec2,
    ) -> Self {
        LadderTileMap {
            size, //TODO
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
                func(&mut tile, UVec2::new(x.try_into().unwrap(), y.try_into().unwrap()), self.size);
            });
        });
    }

    pub fn width(&self) -> u32 {
        return self.size.x
    }

    pub fn height(&self) -> u32 {
        return self.size.y
    }

    pub fn is_empty(&self) -> bool {
        return self.tiles.is_empty()
    }

    pub fn pixel_size(&self) -> Vec2 {
        self.size.as_vec2() * TILE_SIZE
    }

    pub fn rect(&self, position: Vec2) -> Rect {
        Rect::from_corners(position, position + self.pixel_size())
    }

    //TODO BUG - negative coordinates
    pub fn pixel_to_tile_position(&self, transform: &Transform, pixel_coords: Vec2) -> Option<UVec2> {
        if !self.contains_pixel_position(transform, pixel_coords) { return None }
        let position = transform.translation.truncate();
        let delta = pixel_coords - position;
        let tile_position = delta/TILE_SIZE;
        Some(tile_position.as_uvec2())
    }

    pub fn contains_index(&self, index: UVec2) -> bool {
        index.cmpge(self.size).any()
    }

    pub fn contains_pixel_position(&self, transform: &Transform, target_position: Vec2) -> bool {
        let position = transform.translation.truncate();
        self.rect(position).contains(target_position)
    }

    pub fn get_tile(&self, index: UVec2) -> Option<Entity> {
        self.tiles.get(index.x as usize)?.get(index.y as usize).copied()
    }

    pub fn get_tile_from_pixel_position(&self, transform: &Transform, position: Vec2) -> Option<Entity> {
        self.get_tile(self.pixel_to_tile_position(&transform, position)?)
    }
}

pub fn ladder_tile_path_update_system(
    mut tile_query: Query<(&Tile, &mut Path), Changed<Tile>>,
) {
    for (tile, mut path) in tile_query.iter_mut() {
        //Build paths
        *path = GeometryBuilder::build_as(&shapes::SvgPathShape {
            svg_path_string: tile.clone().path_string(),
            //svg_doc_size_in_px: Vec2::new(-1.0, 1.0),
            svg_doc_size_in_px: Vec2::ZERO,
        });
        *path = bevy_prototype_lyon::entity::Path(
            path.0.clone().transformed(
                &tess::geom::Transform::<f32>::scale(TILE_SIZE.x, -TILE_SIZE.y) //TODO Fix invert y
            )
        );
    }
}

pub fn ladder_debug_cpu_debug_system(
    tilemap_query: Query<(&LadderTileMap, Option<&DebugCpuModule>)>,
) {
    for (_tilemap, maybe_debug_cpu) in tilemap_query.iter() {
        //TODO TEMP - Testing debug_cpu
        if let Some(debug_cpu) = maybe_debug_cpu {
            debug_cpu.digital("Xamo69".to_string()).unwrap();
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

pub fn tilemap_mouse_position_system(
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut tilemap_query: Query<(
        Entity, &LadderTileMap, &Transform,
        Option<&mut MouseTilePosition>,
    ), Without<TileMapCursor>
    >,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_viewport_position) = window.cursor_position() else { return; };
    let Some(cursor_world_position) = camera.viewport_to_world_2d(camera_transform, cursor_viewport_position) else { return; };

    for (tilemap_entity, tilemap, tilemap_transform, maybe_mouse_tile_position) in tilemap_query.iter_mut() {
        //Update MouseTilePosition
        let maybe_new_tile_position = tilemap.pixel_to_tile_position(tilemap_transform, cursor_world_position);
        match (maybe_mouse_tile_position, maybe_new_tile_position) {
            (None, None) => (),
            (None, Some(tile_position)) => {
                commands.entity(tilemap_entity)
                .insert(MouseTilePosition(tile_position));
            },
            (Some(_), None) => {
                commands.entity(tilemap_entity)
                .remove::<MouseTilePosition>();
            },
            (Some(mut current_tile_position), Some(new_tile_position)) => {
                *current_tile_position = MouseTilePosition(new_tile_position); //TODO Opt mutate interior
            },
        };
    }
}

pub fn tilemap_cursor_system(
    mut commands: Commands,
    mut tilemap_query: Query<
        (Entity, &MouseTilePosition, Option<&mut TileMapCursorRef>),
        Changed<MouseTilePosition>
    >,
    mut cursor_query: Query<(&mut Transform, &mut TilePosition), With<TileMapCursor>>,
) {
    for (tilemap_entity, mouse_tile_position, maybe_cursor_tile_ref) in tilemap_query.iter_mut() {
        match maybe_cursor_tile_ref {
            None => {
                let cursor_entity = Some(spawn_tile_cursor(&mut commands, mouse_tile_position.0)).unwrap();
                commands.entity(tilemap_entity)
                    .push_children(&vec![cursor_entity])
                    .insert(TileMapCursorRef(cursor_entity)
                );
            },
            Some(cursor_tile_ref) => {
                let (mut cursor_transform, mut cursor_tile_position) = cursor_query.get_mut(**cursor_tile_ref).unwrap();
                cursor_transform.translation = (mouse_tile_position.0.as_vec2()*TILE_SIZE).extend(Z_ORDER_CURSOR);
                cursor_tile_position.0 = mouse_tile_position.0;
            },
        };
    }
}
pub fn tilemap_cursor_removal_system(
    mut commands: Commands,
    mut removed_entities: RemovedComponents<MouseTilePosition>,
    cursor_ref_query: Query<&TileMapCursorRef, Without<MouseTilePosition>>
) {
    for tilemap_entity in &mut removed_entities {
        let Ok(tilemap_cursor_ref) = cursor_ref_query.get(tilemap_entity) else {
            dbg!("TODO FIX ME: MouseTilePosition removed, but no TileMapCursorRef in cursor query");
            return;
        };
        let cursor_entity = **tilemap_cursor_ref;
        commands.entity(tilemap_entity).remove::<TileMapCursorRef>();
        commands.entity(cursor_entity).despawn_recursive();
    }
}

pub fn tile_highlight_system(
    mut commands: Commands,
    mut tilemap_query: Query<(Entity, &LadderTileMap, &MouseTilePosition, Option<&mut HoveredRef>), Changed<MouseTilePosition>>,
) {
    for (tilemap_entity, tilemap, mouse_tile_position, maybe_hovered_ref) in tilemap_query.iter_mut() {
        let tile_entity = tilemap.get_tile(mouse_tile_position.0).unwrap();

        match maybe_hovered_ref {
            Some(mut hovered_tile_ref) if (*hovered_tile_ref).0 != tile_entity => {
                commands.entity((*hovered_tile_ref).0).remove::<Hovered>();
                (*hovered_tile_ref).0 = tile_entity;
                commands.entity(tile_entity).insert(Hovered);
            },
            Some(hovered_tile_ref) if (*hovered_tile_ref).0 == tile_entity => (), //Skip
            Some(_) => unreachable!(),
            None => {
                commands.entity(tilemap_entity).insert(HoveredRef(tile_entity));
                commands.entity(tile_entity).insert(Hovered);
            },
        }
    }
}

fn spawn_tile_cursor(
    commands: &mut Commands,
    tile_position: UVec2,
) -> Entity {
    let cursor_path = format!("M 0,0 H {} V {} H 0 Z", TILE_SIZE.x, TILE_SIZE.y);
    commands.spawn((
        TileMapCursor,
        TilePosition(tile_position),
        ShapeBundle {
            transform: Transform::from_translation(
                (tile_position.as_vec2()*TILE_SIZE).extend(Z_ORDER_CURSOR)
            ),
            path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                svg_path_string: cursor_path,
                svg_doc_size_in_px: Vec2::Y * (TILE_SIZE.y * 2.0), //TODO HACK Invert Y
            }),
            ..default()
        },
        Stroke::new(Color::BLACK, 2.0),
        Fill::color(Color::rgb(0.7, 0.7, 0.9)),
    )).id()
}

pub fn ladder_tile_highlight_system(
    mut tile_query: Query<&mut Stroke, Added<Hovered>>,
) {
    for mut stroke in tile_query.iter_mut() {
        *stroke = Stroke::new(Color::GREEN, 2.0);
    }
}
pub fn ladder_tile_unhighlight_system(
    mut removed_hovered_entities: RemovedComponents<Hovered>,
    mut tile_query: Query<&mut Stroke, Without<Hovered>>,
) {
    for unhovered_entity in &mut removed_hovered_entities {
        let mut stroke = tile_query.get_mut(unhovered_entity).unwrap();
        *stroke = Stroke::new(Color::BLACK, 1.0);
    }
}

pub fn ladder_tile_focus_highlight_system(
    mut tile_query: Query<&mut Stroke, Added<Focused>>,
) {
    for mut stroke in tile_query.iter_mut() {
        *stroke = Stroke::new(Color::GREEN, 6.0);
    }
}
pub fn ladder_tile_focus_unhighlight_system(
    mut removed_focused_entities: RemovedComponents<Focused>,
    mut tile_query: Query<&mut Stroke, Without<Focused>>,
) {
    for unfocused_entity in &mut removed_focused_entities {
        let Ok(mut stroke) = tile_query.get_mut(unfocused_entity) else {
            dbg!("TODO FIX ME: abcd");
            return;
        };
        *stroke = Stroke::new(Color::BLACK, 1.0);
    }
}

pub fn ladder_tile_mouse_system(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    tilemap_query: Query<Option<&FocusedRef>>,
    mut tile_query: Query<(Entity, &Parent), With<Hovered>>,
) {
    for (tile_entity, parent) in tile_query.iter_mut() {
        let maybe_focused_ref = tilemap_query.get(parent.get()).unwrap();

        //Select
        //TODO Remove any already selected
        if mouse_buttons.just_pressed(MouseButton::Left) {
            //Unselect previous
            if let Some(focused_ref) = maybe_focused_ref {
                commands.entity(focused_ref.0).remove::<Focused>();
            }
            //Focus hovered tile
            commands.entity(tile_entity).insert(Focused);
            //Set FocusRef in tilemap
            commands.entity(parent.get()).insert(FocusedRef(tile_entity));
        }

        if mouse_buttons.just_pressed(MouseButton::Right) {
            //Unselect previous
            if let Some(focused_ref) = maybe_focused_ref {
                commands.entity(focused_ref.0).remove::<Focused>();
            }
        }
    }
}

pub fn ladder_tile_mouse_system_old(
    mouse_buttons: Res<Input<MouseButton>>,
    mut scroll_events: EventReader<MouseWheel>,
    tilemap_query: Query<&LadderTileMap>,
    mut tile_query: Query<(&mut Tile, &TilePosition, &Parent), With<Hovered>>,
) {
    for (mut tile, tile_position, parent) in tile_query.iter_mut() {
        let tilemap = tilemap_query.get(parent.get()).unwrap();

        //TODO impl further mouse interface
        if mouse_buttons.just_pressed(MouseButton::Left) {
            let is_coil_column = tile_position.x == tilemap.width()-1;
            let contact_or_coil = match is_coil_column {
                false => ContactOrCoil::Contact,
                true => ContactOrCoil::Coil,
            };

            *tile = Tile::BoolElement(BoolElement {
                contact_or_coil,
                address: "Z09".into(),
                polarity: Polarity::NO,
            });
        }

        if mouse_buttons.just_pressed(MouseButton::Right) {
            let (is_none, is_wire) = match *tile {
                Tile::None => (true, false),
                Tile::Wire(_) => (false, true),
                _ => (false, false),
            };
            let is_coil_column = tile_position.y == tilemap.width()-1;

            *tile = match (is_none, is_wire, is_coil_column) {
                (false, _    , _    ) => Tile::None,
                (true , _    , true ) => tile.clone(), //TODO Opt, cloning self
                (true , false, false) => Tile::Wire(Wire::default()),
                (true , true , false) => Tile::None,
            };
        }

        for event in scroll_events.iter() {
            //TODO handle each event.unit differently
            //TODO handle scroll values
            match *tile {
                Tile::None => { },
                Tile::BoolElement(ref mut bool_element) => bool_element.polarity.invert(),
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
                (_, pos, size) if pos.x == 0 || pos.x == size.x-1 => Tile::Wire(Wire::Vert),
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
        tilemap.tiles =
            (0..tilemap.width()).map(|x| {
                (0..tilemap.height()).map(|y| {
                    let tile_entity = spawn_tile(&mut commands, Tile::default(), Vec2::new(x as f32, y as f32));
                    commands.entity(tilemap_entity)
                        .push_children(&vec![tile_entity]);
                    tile_entity
                }).collect()
            }).collect()
        ;
    }
}

fn spawn_tile(
    mut commands: &mut Commands,
    tile: Tile,
    position: Vec2,
) -> Entity {
    let label_text = tile.label_string();

    let tile_entity = commands.spawn((
        Name::new(format!("Tile: {}", position)),
        tile,
        TilePosition(position.as_uvec2()),
        ShapeBundle {
            transform: Transform::from_translation(
                (position*TILE_SIZE).extend(Z_ORDER_SPRITE)
            ),
            path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                svg_path_string: Tile::default().path_string(),
                svg_doc_size_in_px: Vec2::ZERO, //TODO size
            }),
            ..default()
        },
        Stroke::new(Color::BLACK, 1.0),
    )).id();

    let tile_label_entity = spawn_tilelabel(&mut commands, label_text);
    commands.entity(tile_entity)
    .push_children(&vec![tile_label_entity])
    .id()
}

fn spawn_tilelabel(
    commands: &mut Commands,
    label_text: String
) -> Entity {
    //Label
    let style = TextStyle {
        font_size: 24.0,
        color: Color::BLACK,
        ..default()
    };

    let new_label_text = Text::from_section(label_text, style)
    .with_alignment(TextAlignment::Center);

    commands.spawn((
        TileLabel{},
        Text2dBundle {
            text: new_label_text.clone(),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(32.0, 64.0, 1.0), //TODO Label size
            ..default()
        },
    ))
    .id()
}

//Adds references to TileLabel child as TileLabelRef component on parent
pub fn tile_label_reference_system(
    mut commands: Commands,
    label_query: Query<(Entity, &Parent), Added<TileLabel>>,
    tile_query: Query<Entity, With<Tile>>,
) {
    for (label_entity, parent) in label_query.iter() {
        let parent_tile_entity = tile_query.get(parent.get()).unwrap();
        commands.entity(parent_tile_entity).insert(
            TileLabelRef(label_entity)
        );
    }
}

pub fn ladder_tile_label_update_system(
    mut tile_query: Query<(&Tile, &TileLabelRef), Changed<Tile>>,
    mut label_query: Query<&mut Text, With<TileLabel>>,
) {
    for (tile, tile_label_ref) in tile_query.iter_mut() {
        let mut label_text = label_query.get_mut(tile_label_ref.0).unwrap();
        *label_text = Text::from_section(
            tile.label_string(),
            TextStyle {
                font_size: 24.0,
                color: Color::BLACK,
                ..default()
            }
        ).with_alignment(TextAlignment::Center);
    }
}

pub fn ladder_tile_label_update_system_sans_ref(
    mut tile_query: Query<(&Tile, &Children), Changed<Tile>>,
    mut label_query: Query<&mut Text, With<TileLabel>>,
) {
    for (changed_tile, children) in tile_query.iter_mut() {
        //Can't use for loop with iter_many_mut
        //for mut label_text in label_query.iter_many_mut(children) {
        let mut iter = label_query.iter_many_mut(children);
        let mut label_text = iter.fetch_next().unwrap();
        *label_text = Text::from_section(
            changed_tile.label_string(),
            TextStyle {
                font_size: 24.0,
                color: Color::BLACK,
                ..default()
            }
        ).with_alignment(TextAlignment::Center);
    }
}
