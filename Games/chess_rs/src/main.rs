use chessgame::{self, Result};
use chessgame::{board::{Position, Player}, game::ChessGame};

fn main() -> Result<()> {
    let mut game = ChessGame::default();
    println!("{}", &game.board);
    game.play(Player::White, Position(4, 1), Position(4, 3))?;
    game.play(Player::Black, Position(3, 6), Position(3, 5))?;
    game.play(Player::White, Position(3, 0), Position(5, 2))?;
    println!("{}", game.board);
    Ok(())
}
