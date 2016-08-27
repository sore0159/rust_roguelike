extern crate tcod;

mod display;
mod geometry;
mod game;
mod pc;
mod scene;
mod creature;

fn main() {
    let mut root_screen = display::init();
    let mut game = game::Game::new();

    root_screen.render_game(&game);

    while !(root_screen.console.window_closed() || game.quit) {
        let keypress = root_screen.console.wait_for_keypress(true);
        if let Some(act) = game.parse_input(keypress) {
            // if let Some(act) = commands::parse_input(keypress) {
            game.execute(act);
            root_screen.render_game(&game);
        }
    }
}
