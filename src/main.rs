#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::BError;

mod map;
mod app;
mod player;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
}

fn main() -> BError {
    app::app()
}
