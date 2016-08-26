use game::Game;

pub enum Action {
    Move(Direction),
    Quit,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// pub fn execute(a: Action, g: &mut Game) -> Result<()> {
//
// }
