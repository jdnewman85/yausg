use bevy::prelude::*;

pub mod tile;
pub mod tilemap;
pub mod systems;
pub mod util_systems;

//TODO
const TILE_SIZE: Vec2 = Vec2::new(64.0, 64.0);

#[derive(Component)]
pub struct FocusedRef(Entity);

#[derive(Component)]
pub struct Focused;

#[derive(Component)]
pub struct HoveredRef(Entity);

#[derive(Component)]
pub struct Hovered;

#[derive(Component)]
pub struct TileMapCursorRef(Entity);

#[derive(Component)]
pub struct TileMapCursor;

