#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone,Copy,Hash,Eq,PartialEq,Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn offset_x(&self, offset: i32) -> Self {
        Point {
            x: self.x + offset,
            y: self.y,
        }
    }
    pub fn offset_y(&self, offset: i32) -> Self {
        Point {
            x: self.x,
            y: self.y + offset,
        }
    }
    pub fn go(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => self.offset_y(-1),
            Direction::Down => self.offset_y(1),
            Direction::Left => self.offset_x(-1),
            Direction::Right => self.offset_x(1),
        }
    }
}

pub struct Bound {
    pub min: Point,
    pub max: Point,
}

impl Bound {
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        let (min_x, max_x): (i32, i32) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let (min_y, max_y): (i32, i32) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
        Bound {
            min: Point {
                x: min_x,
                y: min_y,
            },
            max: Point {
                x: max_x,
                y: max_y,
            },
        }
    }
    pub fn contains(&self, point: Point) -> bool {
        if point.x >= self.min.x && point.x <= self.max.x && point.y >= self.min.y &&
           point.y <= self.max.y {
            true
        } else {
            false
        }
    }
}
