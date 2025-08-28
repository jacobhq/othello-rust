use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub enum IllegalMoveError {
    CellOccupied,
    DoesntTurnOver,
    CantMoveOffBoard
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct PointVec(i8, i8);

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

#[derive(Copy, Clone, Debug)]
#[repr(i8)]
pub(crate) enum Color {
    WHITE = 1,
    BLACK = -1,
}

impl From<Color> for i8 {
    fn from(c: Color) -> i8 {
        c as i8
    }
}

#[derive(Clone)]
pub(crate) struct Board([[i8; 8]; 8]);

impl Board {
    fn new() -> Self {
        let mut board = [[0; 8]; 8];

        board[3][3] = Color::WHITE.into();
        board[3][4] = Color::BLACK.into();
        board[4][3] = Color::BLACK.into();
        board[4][4] = Color::WHITE.into();

        Self(board)
    }

    fn get(&self, pos: PointVec) -> Result<i8, IllegalMoveError> {
        let row = match self.0.get(pos.1 as usize) {
            Some(r) => r,
            None => return Err(IllegalMoveError::CantMoveOffBoard),
        };

        let cell = match row.get(pos.0 as usize) {
            Some(&c) => c,
            None => return Err(IllegalMoveError::CantMoveOffBoard),
        };

        Ok(cell)
    }

    fn set(&mut self, pos: PointVec, color: Color) -> Result<i8, IllegalMoveError> {
        let row = match self.0.get_mut(pos.1 as usize) {
            Some(r) => r,
            None => return Err(IllegalMoveError::CantMoveOffBoard),
        };

        let cell = match row.get_mut(pos.0 as usize) {
            Some(c) => c,
            None => return Err(IllegalMoveError::CantMoveOffBoard),
        };

        let old_value = *cell;

        *cell = color.into();

        Ok(old_value)
    }

    pub(crate) fn legal_moves(&self, color: Color) -> Vec<PointVec> {
        let mut moves = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                let pos = PointVec(x, y);
                if self.is_legal_move(color, pos) {
                    moves.push(pos);
                }
            }
        }
        moves
    }

    pub(crate) fn game_over(&self) -> bool {
        self.legal_moves(Color::WHITE).is_empty() && self.legal_moves(Color::BLACK).is_empty()
    }

    pub(crate) fn score(&self) -> (usize, usize) {
        let mut white = 0;
        let mut black = 0;
        for row in &self.0 {
            for &cell in row {
                if cell == Color::WHITE.into() {
                    white += 1;
                } else if cell == Color::BLACK.into() {
                    black += 1;
                }
            }
        }
        (white, black)
    }

    fn is_legal_move(&self, color: Color, pos: PointVec) -> bool {
        match self.get(pos) {
            Ok(value) => {
                if value != 0 {
                    return false;
                }
            }
            Err(_) => return false,
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

    pub(crate) fn play(&mut self, color: Color, pos: PointVec) -> Result<(), IllegalMoveError> {
        if self.is_legal_move(color, pos) {
            match self.0[pos.1 as usize][pos.0 as usize] {
                0 => {
                    self.0[pos.1 as usize][pos.0 as usize] = color.into();

                    let mut flipped_any = false;

                    for d in Direction::all() {
                        let mut looking = pos + d.vector();
                        let mut to_flip = Vec::new();

                        while let Ok(cell) = self.get(looking) {
                            if cell == 0 {
                                break;
                            }
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
                    if flipped_any {
                        self.set(pos, color)?;
                        Ok(())
                    } else {
                        Err(IllegalMoveError::DoesntTurnOver)
                    }
                }
                _ => Err(IllegalMoveError::CellOccupied),
            }
        } else {
            Err(IllegalMoveError::DoesntTurnOver)
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Print column headers 0 to 7
        write!(f, "  ")?; // space before top row of numbers
        for col in 0..=7 {
            write!(f, "{}", col)?;
        }
        writeln!(f)?;

        // Print each row with row index 0 to 7
        for (row_idx, row) in self.0.iter().enumerate() {
            write!(f, "{} ", row_idx)?; // row number + space
            for &cell in row {
                match cell {
                    -1 => write!(f, "#")?,
                    1 => write!(f, "@")?,
                    _ => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct Game {
    pub(crate) board: Board,
    pub(crate) current_turn: Color,
}

impl Game {
    pub(crate) fn new() -> Self {
        Game {
            board: Board::new(),
            current_turn: Color::WHITE,
        }
    }

    pub(crate) fn play_turn(&mut self, pos: PointVec) -> Result<(), IllegalMoveError> {
        self.board.play(self.current_turn, pos)
    }

    pub(crate) fn play_whole_game(&mut self) -> Result<(), ()> {
        while !self.board.game_over() {
            println!("It is {:?}'s turn.", self.current_turn);
            print!("{}", self.board);

            let legal = self.board.legal_moves(self.current_turn);
            if legal.is_empty() {
                println!("{:?} has no legal moves, skipping turn.", self.current_turn);
                self.current_turn = match self.current_turn {
                    Color::WHITE => Color::BLACK,
                    Color::BLACK => Color::WHITE,
                };
                continue;
            }

            println!("Enter your next move (x y): ");
            let _ = stdout().flush();

            let mut input = String::new();
            stdin().read_line(&mut input).map_err(|_| ())?;
            let mut parts = input.trim().split_whitespace();

            let x = match parts.next().and_then(|x| x.parse::<i8>().ok()) {
                Some(val) => val,
                None => {
                    println!("Invalid x coordinate, try again.");
                    continue;
                }
            };
            let y = match parts.next().and_then(|y| y.parse::<i8>().ok()) {
                Some(val) => val,
                None => {
                    println!("Invalid y coordinate, try again.");
                    continue;
                }
            };

            let pos = PointVec(x, y);
            match self.play_turn(pos) {
                Ok(_) => {
                    self.current_turn = match self.current_turn {
                        Color::WHITE => Color::BLACK,
                        Color::BLACK => Color::WHITE,
                    };
                }
                Err(e) => {
                    println!("Illegal move: {:?}", e);
                }
            }
        }

        // Game is over → show final score
        let (white, black) = self.board.score();
        println!(
            "Game over! Final score: White = {}, Black = {}",
            white, black
        );
        Ok(())
    }
}
