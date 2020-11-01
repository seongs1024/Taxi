use std::fmt;

const DEFAULT_SIZE: usize = 5;

#[derive(Clone)]
pub enum Tile {
    R,
    G,
    B,
    Y,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::R => "R",
            Self::G => "G",
            Self::B => "B",
            Self::Y => "Y",
        })
    }
}

pub struct Env {
    pub width: usize,
    pub height: usize,
    finished: bool,
    tiles: Vec<Vec<Option<Tile>>>,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tiles.iter()
            .map(|v| { v.iter()
                .map(|e| match e {
                    Some(e) => format!("{:2}", e),
                    None => format!("{:2}", "-"),
                })
                .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
        )
   }
}

impl Default for Env {
    fn default() -> Self {
        Self::new(DEFAULT_SIZE, DEFAULT_SIZE)
    }
}

impl Env {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            finished: false,
            tiles: vec![vec![None; width]; height],
        }
    }
}
