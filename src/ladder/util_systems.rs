use bevy::prelude::*;

use crate::vladder::DebugCpuModule;

use super::tile::*;
use super::tilemap::*;

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

