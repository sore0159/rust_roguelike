use super::Game;

pub trait Tic {
    fn tic(&mut self);
}

impl Game {
    pub fn tic(&mut self) {
        self.time += 1;
        // self.pc.tic();
        println!("TIC {}", self.time);
    }
}
