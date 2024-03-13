use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::{to_cp437, BTerm, RGB};

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

pub fn new_map() -> Vec<TileType> {
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
