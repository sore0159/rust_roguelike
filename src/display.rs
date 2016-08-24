use tcod::console::{Root, FontLayout, Renderer};
use tcod::{Console, BackgroundFlag};
use geometry::{Point, Bound};


const CON_W: i32 = 100;
const CON_H: i32 = 50;

pub fn init() -> Root {
    Root::initializer()
        .size(CON_W, CON_H)
        .title("Awesome Adventure Four")
        .font("terminal10x16_gs_tc.png", FontLayout::Tcod)
        .renderer(Renderer::GLSL)
        .init()
}

pub trait ThingRender {
    fn get_location(&self) -> &Point;
    fn get_display(&self) -> Option<char>;
    fn render<T: Console>(&self, screen: &mut Screen<T>) {
        let c: char = match self.get_display() {
            None => return,
            Some(x) => x,
        };
        let loc = self.get_location();
        if screen.bound.contains(loc) {
            screen.console.put_char(loc.x, loc.y, c, BackgroundFlag::Set);
        }
    }
}

pub struct Screen<'a, T: Console> {
    pub bound: &'a Bound,
    pub console: T,
}
