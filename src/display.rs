pub use tcod::console::{Root, FontLayout, Renderer};
pub use tcod::{Console, BackgroundFlag};
use geometry::{Point, Bound};
use game::Game;
use std::collections::HashMap;

pub const CON_W: i32 = 100;
pub const CON_H: i32 = 50;

pub struct Screen<T: Console> {
    pub bound: Bound,
    pub console: T,
    pub drawn: HashMap<Point, usize>,
}

pub struct Pix {
    pub loc: Point,
    pub display: char,
    pub z_lvl: usize,
}
pub enum Pixi {
    None,
    One(Pix),
    Many(Vec<Pix>),
}

pub trait Renderable {
    fn get_pix(&self) -> Pixi;
}

impl<T: Console> Screen<T> {
    pub fn clear(&mut self) {
        self.console.clear();
        self.drawn.clear();
    }
    pub fn draw(&mut self, p: Pix) {
        if !self.bound.contains(p.loc) {
            return;
        }
        let z = self.drawn.entry(p.loc).or_insert(p.z_lvl);
        if *z > p.z_lvl {
            return;
        }
        *z = p.z_lvl;
        self.console.put_char(p.loc.x, p.loc.y, p.display, BackgroundFlag::Set);

    }
    pub fn render<K: Renderable>(&mut self, thing: &K) {
        match thing.get_pix() {
            Pixi::One(p) => self.draw(p),
            Pixi::Many(v) => {
                for p in v {
                    self.draw(p);
                }
            }
            Pixi::None => {}
        };

    }
}

impl Screen<Root> {
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
            drawn: HashMap::new(),
        }
    }
    pub fn render_game(&mut self, game: &Game) {
        self.clear();
        self.render(game.get_scene());
        self.render(&game.pc);
        self.console.flush();
    }
}
