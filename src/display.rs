pub use tcod::console::{Root, FontLayout, Renderer};
pub use tcod::{Console, BackgroundFlag};
use geometry::{Point, Bound};
use game::Game;


pub const CON_W: i32 = 100;
pub const CON_H: i32 = 50;

pub fn init() -> Screen<Root> {
    // let mut root = Root::initializer()
    let root = Root::initializer()
        .size(CON_W, CON_H)
        .title("Awesome Adventure Four")
        .font("terminal10x16_gs_tc.png", FontLayout::Tcod)
        .renderer(Renderer::GLSL)
        .init();
    Screen {
        bound: Bound::new(0, CON_W - 1, 0, CON_H - 1),
        console: root,
    }
}

pub trait ThingRender {
    fn get_location(&self) -> &Point;
    fn get_display(&self) -> Option<char>;
}

pub struct Screen<T: Console> {
    pub bound: Bound,
    pub console: T,
}

impl<T: Console> Screen<T> {
    pub fn render<K: ThingRender>(&mut self, thing: &K) {
        let c: char = match thing.get_display() {
            None => return,
            Some(x) => x,
        };
        let loc = thing.get_location();
        if self.bound.contains(loc) {
            self.console.put_char(loc.x, loc.y, c, BackgroundFlag::Set);
        }
    }
    pub fn render_box(&mut self, thing: Box<&ThingRender>) {
        let c: char = match thing.get_display() {
            None => return,
            Some(x) => x,
        };
        let loc = thing.get_location();
        if self.bound.contains(loc) {
            self.console.put_char(loc.x, loc.y, c, BackgroundFlag::Set);
        }

    }
    pub fn render_loc<'a, K: Iterator<Item = Box<&'a ThingRender>>>(&mut self, things: K) {
        for t in things {
            self.render_box(t);
        }
    }
}

impl Screen<Root> {
    pub fn render_game(&mut self, game: &Game) {
        self.console.clear();
        let s = game.get_scene();

        self.render_loc(s.renderables(Some(&game.pc)).into_iter());
        self.console.flush();
    }
}
