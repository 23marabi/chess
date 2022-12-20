mod structures;
use structures::*;

pub trait Logic {
    fn validate_move(&self, board: BaseBoard) -> bool;
    fn make_move(&self, board: BaseBoard) -> BaseBoard;
}
