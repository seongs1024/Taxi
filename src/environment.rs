use std::fmt;

const DEFAULT_SIZE: i32 = 5;

pub struct Env {
    pub width: i32,
    pub height: i32,
    finished: bool,
    //q_table
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WIDTH: {}, HEIGHT: {}", self.width, self.height)
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new(DEFAULT_SIZE, DEFAULT_SIZE)
    }
}

impl Env {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            finished: false,
        }
    }
}
