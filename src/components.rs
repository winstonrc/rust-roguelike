use bracket_lib::terminal::{FontCharType, Point, RGB};
use specs::{prelude::*, Component};

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
}
