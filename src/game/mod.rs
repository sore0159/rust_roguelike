use pc::PC;
use display::{ThingRender, Root, CON_H, CON_W};
use geometry::{Point, Bound};

pub struct Game {
    pub pc: PC,
}

impl Game {
    pub fn new() -> Self {
        Game {
            pc: PC::new(Point {
                x: CON_W / 2,
                y: CON_H / 2,
            }),
        }
    }
    pub fn render(&self, root: &mut Root) {
        let b = Bound::new(0, CON_W, 0, CON_H);
        self.pc.render(root, &b);
    }
}
