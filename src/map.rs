use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::{to_cp437, BTerm, RGB};
use std::cmp::{max, min};

use super::Rect;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub const WIDTH: i32 = 80;
pub const HEIGHT: i32 = 50;
pub const AREA: i32 = WIDTH * HEIGHT;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH as usize) + x as usize
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; AREA as usize];

    // Make the boundary walls
    for x in 0..WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, HEIGHT - 1)] = TileType::Wall;
    }

    for y in 0..HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(WIDTH - 1, y)] = TileType::Wall;
    }

    // Randomly place walls
    let mut rng = RandomNumberGenerator::new();

    for _i in 0..AREA {
        let x = rng.roll_dice(1, WIDTH - 1);
        let y = rng.roll_dice(1, HEIGHT - 1);
        let idx = xy_idx(x, y);

        if idx != xy_idx(WIDTH / 2, HEIGHT / 2) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; AREA as usize];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let width = rng.range(MIN_SIZE, MAX_SIZE);
        let height = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, WIDTH - width - 1) - 1;
        let y = rng.roll_dice(1, HEIGHT - height - 1) - 1;
        let new_room = Rect::new(x, y, width, height);
        let mut ok = true;

        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false;
            }
        }

        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < AREA as usize {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < AREA as usize {
            map[idx] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
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

        x += 1;
        if x > WIDTH - 1 {
            x = 0;
            y += 1;
        }
    }
}
