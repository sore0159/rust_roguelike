use super::Game;

pub trait Tic {
    fn tic(&mut self);
}

impl Game {
    pub fn tic(&mut self) {
        self.time += 1;
        let s = &mut self.scenes[self.current_scene];
        s.tic(self.time, &mut self.pc);
        println!("TIC {}", self.time);
    }
}
