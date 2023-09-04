use bevy::{prelude::*, ecs::query::Has};
use bevy_prototype_lyon::prelude::*;

use super::*;
use super::tile::*;
use super::tilemap::*;
use super::spawns::*;

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
                let cursor_entity = spawn_tile_cursor(&mut commands, mouse_tile_position.0);
                commands.entity(tilemap_entity)
                    .push_children(&vec![cursor_entity])
                    .insert(TileMapCursorRef(cursor_entity)
                );
            },
            Some(cursor_tile_ref) => {
                let (mut cursor_transform, mut cursor_tile_position) = cursor_query.get_mut(cursor_tile_ref.0).unwrap();
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
        let cursor_entity = tilemap_cursor_ref.0;
        commands.entity(tilemap_entity).remove::<TileMapCursorRef>();
        commands.entity(cursor_entity).despawn_recursive();
    }
}

pub fn tile_hover_system(
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
pub fn tilemap_hover_removal_system(
    mut commands: Commands,
    mut removed_entities: RemovedComponents<MouseTilePosition>,
    unhovered_tilemap_query: Query<&HoveredRef, Without<MouseTilePosition>>
) {
    for tilemap_entity in &mut removed_entities {
        let Ok(tilemap_hover_ref) = unhovered_tilemap_query.get(tilemap_entity) else {
            dbg!("TODO FIX ME: MouseTilePosition removed, but no HoveredRef in query");
            return;
        };
        commands.entity(tilemap_hover_ref.0).remove::<Hovered>();
        commands.entity(tilemap_entity).remove::<HoveredRef>();
    }
}

//TODO Rename to unhover
pub fn ladder_tile_unhighlight_system(
    mut commands: Commands,
    mut removed_hovered_entities: RemovedComponents<Hovered>,
) {
    for unhovered_entity in &mut removed_hovered_entities {
        //dbg!("ladder_tile_unhighlight_system");
        commands.entity(unhovered_entity).insert(NeedsStyleUpdate);
    }
}
//TODO Rename to unhover
pub fn ladder_tile_focus_unhighlight_system(
    mut commands: Commands,
    mut removed_focused_entities: RemovedComponents<Focused>,
) {
    for unfocused_entity in &mut removed_focused_entities {
        //dbg!("ladder_tile_focus_unhighlight_system");
        commands.entity(unfocused_entity).insert(NeedsStyleUpdate);
    }
}

pub fn tile_style_system(
    mut commands: Commands,
    mut tile_query: Query<
        (Entity, &mut Stroke, Has<Hovered>, Has<Focused>),
        Or<(Added<Hovered>, Added<Focused>, Added<NeedsStyleUpdate>)>
    >,
) {
    for (tile_entity, mut stroke, hovered, focused) in tile_query.iter_mut() {
    //dbg!("tile_style_system");
        commands.entity(tile_entity).remove::<NeedsStyleUpdate>();
        *stroke = match (hovered, focused) {
            (false, false) => Stroke::new(Color::BLACK, 2.0),
            (false, true) => Stroke::new(Color::GREEN, 2.0),
            (true, false) => Stroke::new(Color::BLACK, 4.0),
            (true, true) => Stroke::new(Color::GREEN, 4.0),
        };
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

pub fn ladder_init_system(
    mut commands: Commands,
    mut tilemap_query: Query<(&mut LadderTileMap, Entity), Added<LadderTileMap>>,
) {
    for (mut tilemap, tilemap_entity) in tilemap_query.iter_mut() {
        tilemap.tiles =
            (0..tilemap.width()).map(|x| {
                (0..tilemap.height()).map(|y| {
                    spawn_tile(&mut commands, Tile::default(), Vec2::new(x as f32, y as f32))
                }).collect()
            }).collect()
        ;

        let children: Vec<Entity> = tilemap.tiles.iter().flatten().copied().collect();
        commands.entity(tilemap_entity)
            .push_children(&children);
    }
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

