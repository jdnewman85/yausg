use bevy::prelude::*;

use super::TILE_SIZE;
use super::tile::*;

#[derive(Component)]
pub struct MouseTilePosition(pub UVec2);

#[derive(Component)]
pub struct TilePosition(pub UVec2);

#[derive(Component)]
pub struct LadderTileMap {
    pub size: UVec2,
    pub tiles: Vec<Vec<Entity>>, //TODO Opt? Single vec? For grid, but maybe not for later rung based
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
            size,
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
                func(&mut tile, UVec2::new(x as u32, y as u32), self.size);
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

