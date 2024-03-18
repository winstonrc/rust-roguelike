use bracket_lib::pathfinding::{Algorithm2D, BaseMap};
use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::{to_cp437, BTerm, Point, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::Player;

use super::{Rect, Viewshed};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;
pub const MAP_AREA: i32 = MAP_WIDTH * MAP_HEIGHT;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn new() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; MAP_AREA as usize],
            rooms: Vec::new(),
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            revealed_tiles: vec![false; MAP_AREA as usize],
            visible_tiles: vec![false; MAP_AREA as usize],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let width = rng.range(MIN_SIZE, MAX_SIZE);
            let height = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - width - 1) - 1;
            let y = rng.roll_dice(1, map.height - height - 1) - 1;
            let new_room = Rect::new(x, y, width, height);
            let mut ok = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < MAP_AREA as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < MAP_AREA as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, viewshed) in (&mut players, &mut viewsheds).join() {
        let mut x = 0;
        let mut y = 0;

        for tile in map.tiles.iter() {
            let point = Point::new(x, y);

            if viewshed.visible_tiles.contains(&point) {
                match tile {
                    TileType::Floor => {
                        ctx.set(
                            x,
                            y,
                            RGB::from_f32(0.5, 0.5, 0.5),
                            RGB::from_f32(0.0, 0.0, 0.0),
                            to_cp437('.'),
                        );
                    }
                    TileType::Wall => {
                        ctx.set(
                            x,
                            y,
                            RGB::from_f32(0.0, 1.0, 0.0),
                            RGB::from_f32(0.0, 0.0, 0.0),
                            to_cp437('#'),
                        );
                    }
                }
            }

            x += 1;
            if x > MAP_WIDTH - 1 {
                x = 0;
                y += 1;
            }
        }
    }
}
