extern crate rand;
extern crate tcod;

use util::Point;
use util::Contains::{ DoesContain, DoesNotContain };
use game::Game;
use traits::Updates;
use self::tcod::console::{ Console, Root, BackgroundFlag };
use self::tcod::input::KeyCode;

pub struct Character {
    pub position: Point,
    pub display_char: char,
}

impl Character {
    pub fn new(position: Point, display_char: char) -> Character {
        Character { position, display_char }
    }
}

impl Updates for Character {
    fn update(&mut self, key: KeyCode, game: &Game) {
        let mut offset = Point { x: 0, y: 0 };
        match key {
            KeyCode::Up => {
                offset.y = -1;
            },
            KeyCode::Down => {
                offset.y = 1;
            },
            KeyCode::Right => {
                offset.x = 1;
            },
            KeyCode::Left => {
                offset.x = -1;
            },
            _ => (),
        }
        
        match game.window_bounds.contains(self.position.offset(&offset)) {
            DoesContain => self.position = self.position.offset(&offset),
            DoesNotContain => (),
        }
    }

    fn render(&self, root: &mut Root) {
        root.put_char(self.position.x, self.position.y, self.display_char, BackgroundFlag::Set);
    }
}