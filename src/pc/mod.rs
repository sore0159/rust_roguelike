use geometry::Point;
use display::ThingRender;
use game::commands::Direction;

pub struct PC {
    pub location: Point,
}

impl ThingRender for PC {
    fn get_location(&self) -> &Point {
        &self.location
    }
    fn get_display(&self) -> Option<char> {
        Some('@')
    }
}

impl PC {
    pub fn new(loc: Point) -> Self {
        PC { location: loc }
    }
    pub fn move_dir(&mut self, d: &Direction) {
        match *d {
            Direction::Up => self.location = self.location.offset_y(-1),
            Direction::Down => self.location = self.location.offset_y(1),
            Direction::Left => self.location = self.location.offset_x(-1),
            Direction::Right => self.location = self.location.offset_x(1),
        };
    }
}
