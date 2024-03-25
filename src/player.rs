use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};

use super::{Map, Player, Position, RunState, State, Viewshed};
use crate::map::{MAP_HEIGHT, MAP_WIDTH};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut ppos = ecs.write_resource::<Point>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if !map.blocked[destination_idx] {
            pos.x = min(MAP_WIDTH - 1, max(0, pos.x + delta_x));
            ppos.x = pos.x;
            pos.y = min(MAP_HEIGHT - 1, max(0, pos.y + delta_y));
            ppos.y = pos.y;
            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        Some(key) => match key {
            VirtualKeyCode::Left
            | VirtualKeyCode::A
            | VirtualKeyCode::H
            | VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right
            | VirtualKeyCode::D
            | VirtualKeyCode::L
            | VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up
            | VirtualKeyCode::W
            | VirtualKeyCode::K
            | VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down
            | VirtualKeyCode::S
            | VirtualKeyCode::J
            | VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.ecs),

            _ => return RunState::Paused,
        },
        None => return RunState::Paused,
    }

    RunState::Running
}
