use std::fmt::{Display, Formatter};

#[derive(Clone, Copy)]
struct PointVec(i8, i8);

impl std::ops::Add for PointVec {
    type Output = PointVec;

    fn add(self, rhs: Self) -> Self::Output {
        PointVec(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign for PointVec {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::Sub for PointVec {
    type Output = PointVec;

    fn sub(self, rhs: Self) -> Self::Output {
        PointVec(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn vector(self) -> PointVec {
        match self {
            Direction::Up => PointVec(0, 1),
            Direction::Down => PointVec(0, -1),
            Direction::Left => PointVec(-1, 0),
            Direction::Right => PointVec(1, 0),
            Direction::UpLeft => PointVec(-1, 1),
            Direction::UpRight => PointVec(1, 1),
            Direction::DownLeft => PointVec(-1, -1),
            Direction::DownRight => PointVec(1, -1),
        }
    }

    fn all() -> &'static [Direction] {
        &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }
}

#[derive(Copy, Clone)]
#[repr(i8)]
enum Color {
    WHITE = 1,
    BLACK = -1,
}

impl From<Color> for i8 {
    fn from(c: Color) -> i8 {
        c as i8
    }
}

struct Board([[i8; 8]; 8]);

impl Board {
    fn new() -> Self {
        let mut board = [[0; 8]; 8];

        board[3][3] = Color::WHITE.into();
        board[3][4] = Color::BLACK.into();
        board[4][3] = Color::BLACK.into();
        board[4][4] = Color::WHITE.into();

        Self(board)
    }

    fn get(&self, pos: PointVec) -> Result<i8, ()> {
        let row = match self.0.get(pos.1 as usize) {
            Some(r) => r,
            None => return Err(()),
        };

        let cell = match row.get(pos.0 as usize) {
            Some(&c) => c,
            None => return Err(()),
        };

        Ok(cell)
    }

    fn set(&mut self, pos: PointVec, color: Color) -> Result<i8, ()> {
        let row = match self.0.get_mut(pos.1 as usize) {
            Some(r) => r,
            None => return Err(()),
        };

        let cell = match row.get_mut(pos.0 as usize) {
            Some(c) => c,
            None => return Err(()),
        };

        let old_value = *cell;

        *cell = color.into();

        Ok(old_value)
    }


    fn is_legal_move(&self, color: Color, pos: PointVec) -> bool {
        if self.get(pos).unwrap() != 0 {
            return false;
        }
        for d in Direction::all() {
            let mut looking = pos + d.vector();
            let mut seen_opp_piece = false;
            loop {
                let cell = match self.get(looking) {
                    Ok(c) => c,
                    Err(_) => break,
                };

                if cell == 0 {
                    break;
                } else if cell != color.into() {
                    looking += d.vector();
                    seen_opp_piece = true
                } else {
                    if cell == color.into() && seen_opp_piece {
                        return true;
                    } else {
                        break;
                    }
                }
            }
        }
        false
    }

    fn play(&mut self, color: Color, pos: PointVec) -> Result<(), ()> {
        if self.is_legal_move(color, pos) {
            match self.0[pos.0 as usize][pos.1 as usize] {
                0 => {
                    self.0[pos.0 as usize][pos.1 as usize] = color.into();

                    let mut flipped_any = false;

                    for d in Direction::all() {
                        let mut looking = pos + d.vector();
                        let mut to_flip = Vec::new();

                        while let Ok(cell) = self.get(looking) {
                            if cell == 0 { break; }
                            if cell != color.into() {
                                to_flip.push(looking);
                                looking += d.vector();
                            } else {
                                if !to_flip.is_empty() {
                                    for p in to_flip {
                                        self.set(p, color.into())?;
                                    }
                                    flipped_any = true;
                                }
                                break;
                            }
                        }
                    }
                    return if flipped_any {
                        self.set(pos, color)?;
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            for j in i {
                match j {
                    -1 => write!(f, "#")?,
                    1 => write!(f, "@")?,
                    _ => write!(f, ".")?,
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let mut board = Board::new();

    println!("{board}");

    board
        .play(Color::WHITE, PointVec(5, 3))
        .unwrap_or_else(|_| println!("Bad2"));
    println!("{board}");
}
