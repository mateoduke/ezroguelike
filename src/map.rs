use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x < SCREEN_WIDTH && point.x >= 0 && point.y < SCREEN_HEIGHT && point.y >= 0
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        let map_idx = map_idx(point.x, point.y);
        self.in_bounds(point) && self.tiles[map_idx] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            return None;
        }
        Some(map_idx(point.x, point.y))
    }
}
