extern crate tcod;
extern crate rand;
extern crate rustyrogue;

use tcod::console::{ Console, Root };
use tcod::input::KeyCode;

use rustyrogue::util::{ Point, Bound };
use rustyrogue::game::Game;
use rustyrogue::character::Character;
use rustyrogue::npc::NPC;
use rustyrogue::traits::Updates;

pub fn render(root: &mut Root, objs: &mut Vec<Box<Updates>>) {
    root.clear();
    for i in objs.iter() {
        i.render(root);
    }
    root.flush();
}

pub fn update(objs: &mut Vec<Box<Updates>>, key: KeyCode, game: &Game) {
    for i in objs.iter_mut() {
        i.update(key, game);
    }
}

fn main() {
    const WINDOW_WIDTH: i32 = 80;
    const WINDOW_HEIGHT: i32 = 50;
    let window_bounds = Bound { min: Point { x: 0, y: 0 },
                                max: Point { x: WINDOW_WIDTH - 1, y: WINDOW_HEIGHT - 1 } };
    let mut game = Game{ end: false, window_bounds: window_bounds };
    let mut root = Root::initializer().size(WINDOW_WIDTH, WINDOW_HEIGHT).title("Rusty Rogue").init();

    let player = Box::new(Character::new(Point { x: 40, y: 25 }, '@')) as Box<Updates>;
    let dog = Box::new(NPC::new(Point { x: 10, y: 10 }, 'd')) as Box<Updates>;
    let mut objs: Vec<Box<Updates>> = vec![player, dog];

    //Initialize the window content
    render(&mut root, &mut objs);

    //Game loop
    while !(root.window_closed() || game.end) {
        //Wait for user input
        let keypress = root.wait_for_keypress(true);

        //Update the game state
        match keypress.code {
            KeyCode::Escape => game.end = true,
            _ => (),
        }

        //Update each of the objects in the game
        update(&mut objs, keypress.code, &game);

        //Render the game
        render(&mut root, &mut objs);

    }
}
