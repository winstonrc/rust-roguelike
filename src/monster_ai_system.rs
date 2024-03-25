use bracket_lib::geometry::Point;
use bracket_lib::pathfinding::a_star_search;
use bracket_lib::terminal::{console, DistanceAlg};
use specs::prelude::*;

use super::{Map, Monster, Name, Position, Viewshed};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (mut viewshed, _monster, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            let distance =
                DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);

            if distance < 1.5 {
                // Attack will go here
                console::log(format!("{} shouts insults.", name.name));
                return;
            }

            if viewshed.visible_tiles.contains(&*player_pos) {
                let path = a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );

                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
