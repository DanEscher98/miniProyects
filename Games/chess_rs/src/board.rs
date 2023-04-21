use std::fmt; 
use colored::*;
use std::io::Write;
use std::str;

pub const BOARD_LEN: usize = 64;
pub const BOARD_SIDE: usize = 8;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'd'];
        let Some(letter) = letters.get(self.x as usize) else {
            return Err(std::fmt::Error);
        };
        if self.y >= 8 {
            return Err(std::fmt::Error);
        }
        write!(f, "{}{}", letter, self.y)
    }
}

impl Position {
    pub fn is_valid(&self) -> bool {
        self.x < BOARD_SIDE && self.y < BOARD_SIDE
    }
}

#[derive(Debug)]
pub struct Move {
    pub piece: Piece,
    pub player: Player,
    pub start: Position,
    pub goal: Position
}

impl Move {
    fn new(player: Player, figure: Figure, moved: bool, start: Position, goal: Position) -> Self {
        Move {
            piece: Piece { player, figure, position: start, moved },
            player,
            start,
            goal
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Black,
    White
}


impl std::ops::Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Figure {
    Queen,
    King,
    Bishop,
    Knight,
    Rook,
    Pawn
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Figure::Pawn => write!(f, "♟️"),
            Figure::Rook => write!(f, "R"),
            Figure::Queen => write!(f, "Q"),
            Figure::King => write!(f, "K"),
            Figure::Bishop => write!(f, "B"),
            Figure::Knight => write!(f, "N"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub player: Player,
    pub figure: Figure,
    pub position: Position,
    pub moved: bool
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {}{}", self.player, self.figure, self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point2idx() {
        let index = Board::point2idx(Position(4, 1));
        assert_eq!(index, 12usize);
    }
}

pub struct Board(pub [Option<Piece>; BOARD_LEN]);

pub trait PlayableBoard<P> {
    fn get_cell(&self, pos: Position) -> Option<P>;
    fn set_cell(&mut self, pos: Position, piece: Option<P>);
    fn point2idx(pos: Position) -> usize;
    fn idx2point(idx: usize) -> Position;
}

impl PlayableBoard<Piece> for Board {
    fn point2idx(pos: Position) -> usize {
        let Position(col, row) = pos;
        ((row as usize) * BOARD_SIDE) + (col as usize)
    }

    fn idx2point(idx: usize) -> Position {
        let col = idx % 8;
        let row = (BOARD_LEN - 1 - idx) / 8;
        Position(col as u8, row as u8)
    }

    fn get_cell(&self, pos: Position) -> Option<Piece> {
        self.0.get(Self::point2idx(pos))
            .and_then(|opt_piece| opt_piece.to_owned())
    }

    fn set_cell(&mut self, pos: Position, piece: Option<Piece>) {
        let index = Self::point2idx(pos);
        self.0[index] = piece;
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_board = Vec::new();
        for idx in 0..BOARD_LEN {
            let Position(col, row) = Board::idx2point(idx);
            let cell = self.get_cell(Position(col, row));
            let figure = cell.map_or(String::from(" ").normal(), |piece| {
                if piece.player == Player::White {
                    format!("{}", piece.figure).white().bold()
                } else {
                    format!("{}", piece.figure).blue().bold()
                }
            });
            //let Position(x, y) = Board::idx2point(idx);
            // if (idx / 8) % 2 == (idx % 8) % 2  {
            if (col % 2 == 0) ^ (row % 2 == 0) {
                write!(&mut fmt_board, "{} {} {}", "[".white(), figure, "]".white()).unwrap();
            } else {
                write!(&mut fmt_board, "{} {} {}", "[".blue(), figure, "]".blue()).unwrap();
            }
            if col == 7 {
                writeln!(&mut fmt_board).unwrap();
            }
        }
        let string = str::from_utf8(&fmt_board[..]).map_err(|_| fmt::Error)?;
        write!(f, "{string}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn idx2point() {
        assert_eq!(Position(0, 0), Board::idx2point(56));
        assert_eq!(Position(7, 7), Board::idx2point(7));
        assert_eq!(Position(5, 2), Board::idx2point(45));
        assert_eq!(Position(2, 5), Board::idx2point(18));
    }
}
