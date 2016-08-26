extern crate tcod;

mod display;
mod geometry;
mod game;
mod pc;
mod commands;

fn main() {
    println!("Hello world!");
    let root = display::init();
    let game = game::Game::new();
    game.render(&mut root);
}
