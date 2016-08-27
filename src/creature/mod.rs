use geometry::Point;
use display::ThingRender;
use game::commands::Direction;

pub struct Creature {
    pub location: Point,
    pub display: char,
}

impl ThingRender for Creature {
    fn get_location(&self) -> &Point {
        &self.location
    }
    fn get_display(&self) -> Option<char> {
        Some(self.display)
    }
}

impl Creature {
    pub fn new(display: char, loc: Point) -> Self {
        Creature {
            display: display,
            location: loc,
        }
    }
    pub fn move_dir(&mut self, d: Direction) {
        match d {
            Direction::Up => self.location = self.location.offset_y(-1),
            Direction::Down => self.location = self.location.offset_y(1),
            Direction::Left => self.location = self.location.offset_x(-1),
            Direction::Right => self.location = self.location.offset_x(1),
        };
    }
}
