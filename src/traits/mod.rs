extern crate tcod;

use self::tcod::console::{Root};
use self::tcod::input::KeyCode;
use game::Game;

//Types that implement this trait will handle their own updating and rendering
pub trait Updates {
    fn update(&mut self, KeyCode, &Game);
    fn render(&self, &mut Root);
}
