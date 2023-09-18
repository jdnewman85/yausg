use bevy::prelude::*;

pub mod tile;
pub mod tilemap;
pub mod spawners;
pub mod systems;
pub mod util_systems;

//TODO
const TILE_SIZE: Vec2 = Vec2::new(64.0, 64.0);

//TODO
const Z_ORDER_SPRITE: f32 = 1.0;
const Z_ORDER_CURSOR: f32 = 0.1;


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

#[derive(Component)]
pub struct NeedsStyleUpdate;

