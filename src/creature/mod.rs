use display::{Pixi, Pix, Renderable};
use geometry::{Direction, Point};

pub struct Creature {
    pub location: Point,
    pub display: char,
}

impl Renderable for Creature {
    fn get_pix(&self) -> Pixi {
        Pixi::One(Pix {
            loc: self.location,
            display: self.display,
            z_lvl: 1,
        })
    }
}

impl Creature {
    pub fn new(display: char, loc: Point) -> Self {
        Creature {
            display: display,
            location: loc,
        }
    }
    pub fn move_dir(&mut self, d: Direction) {
        self.location = self.location.go(d);
    }
}
