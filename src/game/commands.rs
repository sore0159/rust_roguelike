use tcod::input::{Key, KeyCode};
use geometry::Direction;
use super::Game;

pub enum Action {
    Move(Direction),
    Quit,
}

impl Game {
    pub fn parse_input(&self, keypress: Key) -> Option<Action> {
        if !keypress.pressed {
            return None;
        }
        match keypress.code {
            KeyCode::Escape => Some(Action::Quit),
            KeyCode::Char => {
                match keypress.printable {
                    'a' => Some(Action::Move(Direction::Left)),
                    'u' => Some(Action::Move(Direction::Right)),
                    'e' => Some(Action::Move(Direction::Up)),
                    'o' => Some(Action::Move(Direction::Down)),
                    _ => {
                        println!("Uncaught Printable KeyPress: {:?}", keypress);
                        None
                    }
                }
            }
            _ => {
                println!("Uncaught KeyPress: {:?}", keypress);
                None
            }
        }
    }
}
