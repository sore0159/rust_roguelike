use creature::Creature;
use display::{Pixi, Renderable};
use pc::PC;
use geometry::{Direction, Point};

pub struct Scene {
    pub name: &'static str,
    pub pc_loc: Option<Point>,
    pub creatures: Vec<Creature>,
}

impl Scene {
    pub fn new(name: &'static str) -> Self {
        Scene {
            name: name,
            pc_loc: None,
            creatures: Vec::new(),
        }
    }
    pub fn space_open(&self, p: Point) -> bool {
        if let Some(x) = self.pc_loc {
            if x == p {
                return false;
            }
        }
        for c in &self.creatures {
            if c.location == p {
                return false;
            }
        }
        return true;
    }
    pub fn tic(&mut self, time: u32, pc: &mut PC) {
        let pc_loc = pc.location;
        for c in &mut self.creatures {
            let mut target: Point = if c.location.x % 2 == 0 {
                if c.location.y > 10 {
                    c.location.go(Direction::Up)
                } else {
                    c.location.go(Direction::Left)
                }
            } else {
                if c.location.y < 30 {
                    c.location.go(Direction::Down)
                } else {
                    c.location.go(Direction::Right)
                }
            };
            if target != pc_loc {
                c.location = target;
            }
        }
    }
}

impl Renderable for Scene {
    fn get_pix(&self) -> Pixi {
        let mut v = Vec::with_capacity(self.creatures.len());
        for c in &self.creatures {
            match c.get_pix() {
                Pixi::One(p) => v.push(p),
                Pixi::Many(mut v2) => v.append(&mut v2),
                Pixi::None => {}
            };
        }
        Pixi::Many(v)
    }
}
