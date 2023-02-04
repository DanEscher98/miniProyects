use chessgame::{self, Result};
use chessgame::{board::Board, game::ChessGame};
use std::fmt::Display;

fn main() -> Result<()> {
    let game = ChessGame::default();
    print!("{}", game.board);
    Ok(())
}
