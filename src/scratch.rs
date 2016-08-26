
extern crate tcod;

mod geometry;
mod commands;
mod display;

use tcod::input::KeyCode;
use tcod::console::{Root, FontLayout, Renderer};
use display::{CON_H, CON_W};

fn main() {
    let mut con = display::init();
    let mut world = World::new();
    world.render(&mut con);
    let mut exit = false;
    while !(con.window_closed() || exit) {
        // FETCH USER INPUT
        let keypress = con.wait_for_keypress(true);
        if !keypress.pressed {
            continue;
        }
        // UPDATE GAME STATE
        match keypress.code {
            KeyCode::Escape => exit = true,
            KeyCode::Char => {
                match keypress.printable {
                    'a' => {
                        if char_x >= 1 {
                            char_x -= 1
                        }
                    }
                    'u' => {
                        if char_x < (con_w - 1) {
                            char_x += 1;
                        }
                    }
                    'e' => {
                        if char_y >= 1 {
                            char_y -= 1
                        }
                    }
                    'o' => {
                        if char_y < (con_h - 1) {
                            char_y += 1;
                        }
                    }
                    _ => println!("Uncaught KeyPress: {:?}", keypress),
                }
            }
            _ => println!("Uncaught KeyPress: {:?}", keypress),
        }
        // RENDER
        render(&mut con, char_x, char_y);
    }
}

fn render(con: &mut Root, x: i32, y: i32) {
    con.clear();
    con.put_char(x, y, '@', BackgroundFlag::Set);
    con.flush();
}
