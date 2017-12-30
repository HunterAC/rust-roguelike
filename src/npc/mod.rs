extern crate tcod;
extern crate rand;

use util::{Point};
use util::Contains::{ DoesContain, DoesNotContain };
use game::Game;
use traits::Updates;
use self::tcod::input::KeyCode;
use self::rand::{ thread_rng, Rng };
use self::tcod::console::{ Console, Root, BackgroundFlag };

pub struct NPC {
    pub position: Point,
    pub display_char: char,
}

impl NPC {
    pub fn new(position: Point, display_char: char) -> NPC {
        NPC { position, display_char }
    }
}

impl Updates for NPC {
    fn update(&mut self, _key: KeyCode, game: &Game) {
        let offset_x: i32 = thread_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            DoesContain => self.position = self.position.offset_x(offset_x),
            DoesNotContain => (),
        }

        let offset_y: i32 = thread_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            DoesContain => self.position = self.position.offset_y(offset_y),
            DoesNotContain => (),
        }
    }

    fn render(&self, root: &mut Root) {
        root.put_char(self.position.x, self.position.y, self.display_char, BackgroundFlag::Set);
    }
}