use pc::PC;
use display::{CON_H, CON_W};
use geometry::Point;
pub mod tic;
pub mod commands;
use self::commands::Action;
use scene::Scene;

pub struct Game {
    pub quit: bool,
    pub time: u32,
    pub pc: PC,
    pub scenes: Vec<Scene>,
    pub current_scene: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            quit: false,
            time: 0,
            pc: PC::new(Point {
                x: CON_W / 2,
                y: CON_H / 2,
            }),
            scenes: Vec::new(),
            current_scene: 0,
        }
    }

    pub fn execute(&mut self, a: Action) {
        match a {
            Action::Quit => self.quit = true,
            Action::Move(x) => {
                self.pc.move_dir(x);
                self.tic();
            }
        }
    }
}
