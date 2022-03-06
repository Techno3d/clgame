
pub trait Entity {
    fn return_location(&self) -> Location;
    fn return_icon(&self) -> &String;
}

pub struct Location {
    pub x: u8,
    pub y: u8
}

impl Location {
    pub fn new(x:u8, y:u8) -> Location {
        Location {x, y}
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

