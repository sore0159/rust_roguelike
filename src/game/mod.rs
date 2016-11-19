use pc::PC;
use display::{CON_H, CON_W};
use geometry::Point;
pub mod tic;
pub mod commands;
use self::commands::{Action, Direction};
use scene::Scene;
use creature::Creature;

pub struct Game {
    pub quit: bool,
    pub time: u32,
    pub pc: PC,
    pub scenes: Vec<Scene>,
    pub current_scene: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut pc = PC::new(Point {
            x: CON_W / 2,
            y: CON_H / 2,
        });
        let mut scene = Scene::new("Starting Scene");
        scene.creatures.push(Creature::new('A', pc.location.go(&Direction::Up)));
        scene.pc_loc = Some(pc.location);
        Game {
            quit: false,
            time: 0,
            pc: pc,
            scenes: vec![scene],
            current_scene: 0,
        }
    }

    pub fn execute(&mut self, a: Action) {
        match a {
            Action::Quit => self.quit = true,
            Action::Move(x) => {
                let mut flag = false;
                {
                    let ref mut s = self.scenes[self.current_scene];
                    if s.space_open(self.pc.location.go(&x)) {
                        flag = true;
                    }
                }
                if flag {
                    self.move_pc(&x);
                }
                self.tic();
            }
        }
    }
    pub fn move_pc(&mut self, dir: &Direction) {
        self.pc.move_dir(dir);
        self.get_scene_mut().pc_loc = Some(self.pc.location);
    }
    pub fn get_scene_mut(&mut self) -> &mut Scene {
        &mut self.scenes[self.current_scene]
    }
    pub fn get_scene(&self) -> &Scene {
        &self.scenes[self.current_scene]
    }
}
