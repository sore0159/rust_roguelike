pub use tcod::console::{Root, FontLayout, Renderer};
pub use tcod::{Console, BackgroundFlag};
use geometry::{Point, Bound};
use game::Game;


pub const CON_W: i32 = 100;
pub const CON_H: i32 = 50;

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
    // fn render<T: Console>(&self, con: &mut Console, bound: &Bound) {
    // fn render<'a, T: Console>(&self, screen: &'a mut Screen<'a, T>) {
    // let c: char = match self.get_display() {
    // None => return,
    // Some(x) => x,
    // };
    // let loc = self.get_location();
    // if screen.bound.contains(loc) {
    // screen.console.put_char(loc.x, loc.y, c, BackgroundFlag::Set);
    // if bound.contains(loc) {
    // con.put_char(loc.x, loc.y, c, BackgroundFlag::Set);
    // }
    // }
    //
}

pub struct Screen<'a, T: 'a, Console> {
    pub bound: &'a Bound,
    pub console: &'a mut T,
}

impl<'a, T: 'a, Console> Screen<'a, T> {
    pub fn render(&mut self, thing: ThingRender) {
        //
    }
}
