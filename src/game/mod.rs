use pc::PC;
use display::{CON_H, CON_W};
use geometry::Point;

pub struct Game {
    pub pc: PC,
}

impl Game {
    pub fn new() -> Self {
        Game {
            pc: pc::new(Point {
                x: CON_W / 2,
                y: CON_H / 2,
            }),
        }
    }
}
