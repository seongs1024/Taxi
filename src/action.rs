use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

pub enum Action {
    North,
    South,
    East,
    West,
    PickUp,
    DropOff,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::North => "North",
            Self::South => "South",
            Self::East => "East",
            Self::West => "West",
            Self::PickUp => "Pick Up",
            Self::DropOff => "Drop Off",
        })
    }
}

impl Distribution<Tile> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        match rng.gen_range(0, 6) {
            0 => Action::North,
            1 => Action::South,
            2 => Action::East,
            3 => Action::West,
            4 => Action::PickUp,
            5 => Action::DropOff,
            _ => unreachable!(),
        }
    }
}

