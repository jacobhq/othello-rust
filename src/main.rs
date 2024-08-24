enum Tile {
    Black,
    White,
}

impl Tile {
    pub fn flip(&self) -> Self {
        match &self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct BitBoard(pub u64);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Position {
    bb_sides: [BitBoard; 2],
    bb_pieces: [[BitBoard; 2]; 2],
}

pub struct Sides;
impl Sides {
    pub const BLACK: usize = 1;
    pub const WHITE: usize = 0;
}

fn main() {
    let a = 1 << 9;
    println!("Hello, world!");
}
