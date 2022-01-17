#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::BError;

mod map;
mod app;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

fn main() -> BError {
    app::app()
}
