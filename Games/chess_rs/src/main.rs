use anyhow::{self, ensure, bail};
use thiserror;

const BOARD_LEN: usize = 64;
const BOARD_SIDE: usize = 8;

type Result<T> = anyhow::Result<T, anyhow::Error>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    Black,
    White
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Figure {
    Queen,
    King,
    Bishop,
    Knight,
    Rook,
    Pawn
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Piece {
    player: Player,
    figure: Figure,
    moved: bool
}

#[derive(Debug)]
struct Position(u8, u8);

#[derive(Debug)]
struct Move {
    piece: Piece,
    player: Player,
    start: Position,
    goal: Position
}

struct ChessGame { 
    board: [Option<Piece>; BOARD_LEN],
    turn: Player
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ChessError {
    #[error("There has been a move out of turn")]
    MoveOutOfTurn,
    #[error("That cell doesn't have any piece")]
    NoPieceInCell
}

impl Default for ChessGame {
    fn default() -> ChessGame {
        let mut board = ChessGame {
            board: [None; BOARD_LEN],
            turn: Player::White
        };
        // PAW
        for col in 0..8u8 {
            [(Player::Black, 6u8), (Player::White, 1u8)].iter().for_each(|(player, row)| {
                board.board[ChessGame::point2idx(col, *row)] = Some(Piece {
                    figure: Figure::Pawn,
                    player: *player,
                    moved: false
                });
            });
        }
        // ROOK
        for col in [0u8, 7u8] {
            [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
                board.board[ChessGame::point2idx(col, *row)] = Some(Piece {
                    figure: Figure::Rook,
                    player: *player,
                    moved: false
                });
            });
        }
        // KNIGHT
        for col in [1u8, 6u8] {
            [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
                board.board[ChessGame::point2idx(col, *row)] = Some(Piece {
                    figure: Figure::Knight,
                    player: *player,
                    moved: false
                });
            });
        }
        // BISHOP
        for col in [2u8, 5u8] {
            [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
                board.board[ChessGame::point2idx(col, *row)] = Some(Piece {
                    figure: Figure::Bishop,
                    player: *player,
                    moved: false
                });
            });
        }
        // QUEENS
        [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
            board.board[ChessGame::point2idx(3u8, *row)] = Some(Piece {
                figure: Figure::Queen,
                player: *player,
                moved: false
            });
        });
        // KINGS
        [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
            board.board[ChessGame::point2idx(4u8, *row)] = Some(Piece {
                figure: Figure::King,
                player: *player,
                moved: false
            });
        });
        board
    }
}

impl ChessGame {
    fn new() -> Self {
        Self::default()
    }
    fn point2idx(col: u8, row: u8) -> usize {
        ((row as usize) * BOARD_SIDE) + (col as usize)
    }
    fn get_piece(&self, col: u8, row: u8) -> Option<Piece> {
        let Some(opt_piece) = self.board.get(ChessGame::point2idx(col, row)) else {
            return None;
        };
        opt_piece.to_owned()
    }
    fn set_cell(&mut self, col: u8, row: u8, piece: Option<Piece>) {
        let index = ChessGame::point2idx(col, row);
        self.board[index] = piece;
    }
    fn play(mut self, movement: Move) -> Result<()> {
        ensure!(self.turn == movement.player, ChessError::MoveOutOfTurn);
        let Position(s_col, s_row) = movement.start;
        let Some(piece) = self.get_piece(s_col.to_owned(), s_row.to_owned()) else {
            bail!(ChessError::NoPieceInCell);
        };
        // check if valid movement, match figure
        let Position(g_col, g_row) = movement.goal;
        self.set_cell(s_col, s_row, None);
        self.set_cell(g_col, g_row, Some(piece));
        anyhow::Ok(())
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point2idx() {
        let index = ChessGame::point2idx(4, 1);
        assert_eq!(index, 12usize);
    }

    #[test]
    fn check_default() {
        let index = ChessGame::point2idx(4, 1);
        dbg!(index);
        let board = ChessGame::new();
        assert_eq!(board.get_piece(4, 1).map(|piece| piece.figure), Some(Figure::Pawn));
    }
}
