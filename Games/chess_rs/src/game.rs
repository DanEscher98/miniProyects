use crate::board::*;
use anyhow::{self, ensure, bail};
use crate::Result;

pub struct ChessGame { 
    pub board: Board,
    turn: Player
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ChessError {
    #[error("There has been a move out of turn")]
    MoveOutOfTurn(Player),
    #[error("That cell doesn't have any piece")]
    NoPieceInCell,
    #[error("You can't attack an own piece")]
    CantSelfAttack,
    #[error("`{0}` is blocking the move of `{1}`")]
    PieceInTheMiddle(Figure, Figure),
    #[error("Position out of bound")]
    PositionOutOfBounds
}

impl Default for ChessGame {
    fn default() -> ChessGame {
        let mut game = ChessGame {
            board: Board([None; BOARD_LEN]),
            turn: Player::White
        };
        // PAW
        for col in 0..8u8 {
            [(Player::Black, 6u8), (Player::White, 1u8)].iter().for_each(|(player, row)| {
                let position = Position(col, *row);
                game.board.0[Board::point2idx(position)] = Some(Piece {
                    figure: Figure::Pawn,
                    player: *player,
                    position,
                    moved: false
                });
            });
        }
        // ROOK
        for col in [0u8, 7u8] {
            [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
                let position = Position(col, *row);
                game.board.0[Board::point2idx(Position(col, *row))] = Some(Piece {
                    figure: Figure::Rook,
                    player: *player,
                    position,
                    moved: false
                });
            });
        }
        // KNIGHT
        for col in [1u8, 6u8] {
            [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
                let position = Position(col, *row);
                game.board.0[Board::point2idx(position)] = Some(Piece {
                    figure: Figure::Knight,
                    player: *player,
                    position,
                    moved: false
                });
            });
        }
        // BISHOP
        for col in [2u8, 5u8] {
            [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
                let position = Position(col, *row);
                game.board.0[Board::point2idx(position)] = Some(Piece {
                    figure: Figure::Bishop,
                    player: *player,
                    position,
                    moved: false
                });
            });
        }
        // QUEENS
        [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
            let position = Position(3u8, *row);
            game.board.0[Board::point2idx(position)] = Some(Piece {
                figure: Figure::Queen,
                player: *player,
                position,
                moved: false
            });
        });
        // KINGS
        [(Player::Black, 7u8), (Player::White, 0u8)].iter().for_each(|(player, row)| {
            let position = Position(4u8, *row);
            game.board.0[Board::point2idx(position)] = Some(Piece {
                figure: Figure::King,
                player: *player,
                position,
                moved: false
            });
        });
        game
    }
}

impl ChessGame {
    pub fn valid_piece_move(start: Position, goal: Position, piece: Piece) -> bool {
        let player = piece.player;
        match piece.figure {
            Figure::Pawn => {
                /*first move, W(x, y) -> W(x, y+2)
                * normal W(x, y) -> W(x, y+1)
                * attack W(x, y) -> W(x+1, y+1) | W(x-1, y+1)
                * reached opposite side, crown
                * */
                todo!()
            },
            _ => todo!()
        }
    }
    pub fn play(mut self, movement: Move) -> Result<()> {
        ensure!(self.turn == movement.player, ChessError::MoveOutOfTurn(movement.player));
        let Some(own_piece) = self.board.get_cell(movement.start.to_owned()) else {
            bail!(ChessError::NoPieceInCell);
        };
        if let Some(goal_piece) = self.board.get_cell(movement.goal.to_owned()) {
            ensure!(goal_piece.player != own_piece.player, ChessError::CantSelfAttack);
        }
        // check if valid movement, match figure
        self.board.set_cell(movement.start, None);
        self.board.set_cell(movement.goal, Some(own_piece));
        anyhow::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rook_correct_default() {
        let game = ChessGame::default();
        let so_rook = Piece {
            player: Player::White,
            figure: Figure::Rook,
            position: Position(0, 0),
            moved: false
        };
        let piece = game.board.get_cell(Position(0, 0));
        assert_eq!(piece, Some(so_rook));

        let ne_rook = Piece {
            player: Player::Black,
            figure: Figure::Rook,
            position: Position(7, 7),
            moved: false
        };
        let piece = game.board.get_cell(Position(7, 7));
        assert_eq!(piece, Some(ne_rook));
    }
}
