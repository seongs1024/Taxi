use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

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

impl Distribution<Tile> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tile {
        match rng.gen_range(0, 4) {
            0 => Tile::R,
            1 => Tile::G,
            2 => Tile::B,
            3 => Tile::Y,
            _ => unreachable!(),
        }
    }
}

pub struct Env {
    pub width: usize,
    pub height: usize,
    finished: bool,
    tiles: Vec<Vec<Option<Tile>>>,
    taxi_pos: (usize, usize),
    passenger: Tile,
    dest: Tile,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WIDTH: {}, HEIGHT: {}\nFINISHED: {}\nTAXI_POS: ({}, {})\nPASS: {}, DEST: {}\n",
            self.width, self.height, self.finished, self.taxi_pos.0, self.taxi_pos.1, self.passenger, self.dest);

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
        let mut rng = rand::thread_rng();

        let mut tiles = vec![vec![None; width]; height];

        tiles[0][0] = Some(Tile::R);
        tiles[width - 1][0] = Some(Tile::G);
        tiles[0][height - 1] = Some(Tile::Y);
        tiles[width - 1][height - 2] = Some(Tile::B);

        // taxi_pos (5x5)
        // passenger_loc (RGYBT)

        Self {
            width,
            height,
            finished: false,
            tiles: tiles,
            taxi_pos: (rng.gen_range(0, width), rng.gen_range(0, height)),
            passenger: rng.gen::<Tile>(),
            dest: rng.gen::<Tile>(),
        }
    }
}
