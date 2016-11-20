use geometry::{Direction, Point};
use display::{Renderable, Pix, Pixi};

pub struct PC {
    pub location: Point,
}

impl Renderable for PC {
    fn get_pix(&self) -> Pixi {
        Pixi::One(Pix {
            loc: self.location,
            display: '@',
            z_lvl: 1,
        })
    }
}

impl PC {
    pub fn new(loc: Point) -> Self {
        PC { location: loc }
    }
    pub fn move_dir(&mut self, d: Direction) {
        self.location = self.location.go(d);
    }
}
