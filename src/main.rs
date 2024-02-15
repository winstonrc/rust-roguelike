use bracket_lib::prelude::*;
use specs::{prelude::*, Component};

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

fn main() -> BError {
    let context: BTerm = BTermBuilder::simple80x50()
        .with_title("Roguelike")
        .build()?;

    let mut gs: State = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: to_cp437('☺'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .build();
    }

    main_loop(context, gs)
}
