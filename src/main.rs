mod structures;
use structures::{BaseBoard, Board, File, Move, Piece, Position};

fn main() {
    let test_board = BaseBoard::new();
    println!("{:#?}", test_board.as_array());
}
