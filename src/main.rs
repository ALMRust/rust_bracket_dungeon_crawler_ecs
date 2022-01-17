#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::BError;

mod map;
mod app;
mod player;
mod map_builder;
mod camera;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}

fn main() -> BError {
    app::app()
}
