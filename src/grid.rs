use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Lane(pub u32);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GridX(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TilePos {
    pub lane: Lane,
    pub x: GridX,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum GridItem {}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub occupied_by: Option<GridItem>,
}

impl Tile {
    pub fn empty() -> Self {
        Self { occupied_by: None }
    }

    pub fn is_empty(&self) -> bool {
        self.occupied_by.is_none()
    }
}

#[derive(Resource)]
pub struct Grid {
    tiles: HashMap<TilePos, Tile>,
    pub width: u32,
    pub height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut tiles = HashMap::new();
        for lane in 0..height {
            for x in 0..width {
                tiles.insert(
                    TilePos {
                        lane: Lane(lane),
                        x: GridX(x),
                    },
                    Tile::empty(),
                );
            }
        }
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn get(&self, lane: Lane, x: GridX) -> Option<&Tile> {
        self.tiles.get(&TilePos { lane, x })
    }

    pub fn is_empty(&self, lane: Lane, x: GridX) -> bool {
        self.get(lane, x)
            .map(|tile| tile.is_empty())
            .unwrap_or(false)
    }

    pub fn place_item(&mut self, lane: Lane, x: GridX, item: GridItem) -> Result<(), &'static str> {
        let pos = TilePos { lane, x };
        if let Some(tile) = self.tiles.get_mut(&pos) {
            if tile.is_empty() {
                tile.occupied_by = Some(item);
                Ok(())
            } else {
                Err("Tile already occupied")
            }
        } else {
            Err("Invalid tile position")
        }
    }

    pub fn remove_item(&mut self, lane: Lane, x: GridX) -> Option<GridItem> {
        let pos = TilePos { lane, x };
        self.tiles
            .get_mut(&pos)
            .and_then(|tile| tile.occupied_by.take())
    }

    pub fn get_item_at(&self, lane: Lane, x: GridX) -> Option<GridItem> {
        self.get(lane, x).and_then(|tile| tile.occupied_by)
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(9, 5));
    }
}
