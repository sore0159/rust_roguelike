use geometry::Point;
use display::ThingRender;

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
    pub fn update(&mut self) {
        //
    }
}
