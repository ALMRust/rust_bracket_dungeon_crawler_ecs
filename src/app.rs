use crate::player::Player;
use crate::prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(
                Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)
            ),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

pub fn app() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler") .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
