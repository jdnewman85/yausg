use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::*;
use super::tile::*;
use super::tilemap::*;
use super::TILE_SIZE;

pub fn spawn_tile(
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
            spatial: SpatialBundle{
                transform: Transform::from_translation(
                    (position*TILE_SIZE).extend(Z_ORDER_SPRITE)
                ),
                ..default()
            },
            path: GeometryBuilder::build_as(&shapes::SvgPathShape {
                svg_path_string: Tile::default().path_string(),
                svg_doc_size_in_px: Vec2::ZERO, //TODO size
            }),
            ..default()
        },
        Stroke::new(Color::BLACK, 2.0),
    )).id();

    let tile_label_entity = spawn_tilelabel(&mut commands, label_text);
    commands.entity(tile_entity)
    .push_children(&vec![tile_label_entity])
    .id()
}

pub fn spawn_tile_cursor(
    commands: &mut Commands,
    tile_position: UVec2,
) -> Entity {
    let cursor_path = format!("M 0,0 H {} V {} H 0 Z", TILE_SIZE.x, TILE_SIZE.y);
    commands.spawn((
        TileMapCursor,
        TilePosition(tile_position),
        ShapeBundle {
            spatial: SpatialBundle{
                transform: Transform::from_translation(
                    (tile_position.as_vec2()*TILE_SIZE).extend(Z_ORDER_CURSOR)
                ),
                ..default()
            },
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


pub fn spawn_tilelabel(
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
        TileLabel,
        Text2dBundle {
            text: new_label_text.clone(),
            text_anchor: bevy::sprite::Anchor::Center,
            transform: Transform::from_xyz(32.0, 64.0, 1.0), //TODO Label size
            ..default()
        },
    ))
    .id()
}

