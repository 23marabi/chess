use std::fmt;

/// A type alias for the basic 8x8 chess board.
//pub type BaseBoard = [[Piece; 8]; 8];
pub struct BaseBoard {
    rank8: [Piece; 8],
    rank7: [Piece; 8],
    rank6: [Piece; 8],
    rank5: [Piece; 8],
    rank4: [Piece; 8],
    rank3: [Piece; 8],
    rank2: [Piece; 8],
    rank1: [Piece; 8],
}

/// An `Enum` of the side of the piece, Black or White.
#[derive()]
pub enum Side {
    White,
    Black,
}

/// Different possible pieces, including `None` for a blank space. Also includes the `Side` the
/// piece is on.
#[derive()]
pub enum Piece {
    Pawn(Side),
    Knight(Side),
    Bishop(Side),
    Rook(Side),
    Queen(Side),
    King(Side),
    None,
}

/// The "file" position of a piece. A-H
#[derive()]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

/// Tuple of a position a piece can be in, including Rank and `File`
#[derive()]
pub struct Position(pub File, pub i8);

/// Everything needed to represent a move with chess notation.
pub struct Move {
    pub piece: Piece,
    pub start_pos: Position,
    pub end_pos: Position,
    pub captured: bool,
}

/// Display a `Piece` in chess notation format - single letter.
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn(_) => write!(f, ""),
            Piece::Knight(_) => write!(f, "N"),
            Piece::Bishop(_) => write!(f, "B"),
            Piece::Rook(_) => write!(f, "R"),
            Piece::Queen(_) => write!(f, "Q"),
            Piece::King(_) => write!(f, "K"),
            Piece::None => write!(f, "ERR"),
        }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn(s) => write!(f, "Pawn({})", s),
            Piece::Knight(s) => write!(f, "Knight({})", s),
            Piece::Bishop(s) => write!(f, "Bishop({})", s),
            Piece::Rook(s) => write!(f, "Rook({})", s),
            Piece::Queen(s) => write!(f, "Queen({})", s),
            Piece::King(s) => write!(f, "King({})", s),
            Piece::None => write!(f, "Blank"),
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::White => write!(f, "W"),
            Side::Black => write!(f, "B"),
        }
    }
}

/// Display a `File` in chess notation format.
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            File::A => write!(f, "a"),
            File::B => write!(f, "b"),
            File::C => write!(f, "c"),
            File::D => write!(f, "d"),
            File::E => write!(f, "e"),
            File::F => write!(f, "f"),
            File::G => write!(f, "g"),
            File::H => write!(f, "h"),
        }
    }
}

/// Display a `Move` as chess notation.
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.captured {
            write!(
                f,
                "{}{}{}x{}{}",
                self.piece, self.start_pos.0, self.start_pos.1, self.end_pos.0, self.end_pos.1
            )
        } else {
            write!(
                f,
                "{}{}{}{}{}",
                self.piece, self.start_pos.0, self.start_pos.1, self.end_pos.0, self.end_pos.1
            )
        }
    }
}

/// Traits for the Boards
pub trait Board {
    fn new() -> Self;
    fn as_array(&self) -> [&[Piece; 8]; 8];
}

// impl fmt::Display for BaseBoard {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let row_str: &str = "{}{}{}{}{}{}{}{}";
//         let mut rows: [&str; 8];
//         let mut rows_str: &str;
//         for i in 0..8 {
//             rows[i] = format!(row_str, self.as_array()[i]);
//         }
//         for x in 0..8 {
//             rows_str = format!("{}{}", rows_str, rows[x]);
//         }
//         write!(f, rows_str)
//     }
// }

impl Board for BaseBoard {
    /// Implement the ability to create a new blank 8x8 chess board.
    /// TODO: FEN string -> board
    fn new() -> BaseBoard {
        BaseBoard {
            rank8: [
                Piece::Rook(Side::Black),
                Piece::Knight(Side::Black),
                Piece::Bishop(Side::Black),
                Piece::King(Side::Black),
                Piece::Queen(Side::Black),
                Piece::Bishop(Side::Black),
                Piece::Knight(Side::Black),
                Piece::Rook(Side::Black),
            ],
            rank7: [
                Piece::Pawn(Side::Black),
                Piece::Pawn(Side::Black),
                Piece::Pawn(Side::Black),
                Piece::Pawn(Side::Black),
                Piece::Pawn(Side::Black),
                Piece::Pawn(Side::Black),
                Piece::Pawn(Side::Black),
                Piece::Pawn(Side::Black),
            ],
            rank6: [
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
            ],
            rank5: [
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
            ],
            rank4: [
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
            ],
            rank3: [
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
                Piece::None,
            ],
            rank2: [
                Piece::Pawn(Side::White),
                Piece::Pawn(Side::White),
                Piece::Pawn(Side::White),
                Piece::Pawn(Side::White),
                Piece::Pawn(Side::White),
                Piece::Pawn(Side::White),
                Piece::Pawn(Side::White),
                Piece::Pawn(Side::White),
            ],
            rank1: [
                Piece::Rook(Side::White),
                Piece::Knight(Side::White),
                Piece::Bishop(Side::White),
                Piece::King(Side::White),
                Piece::Queen(Side::White),
                Piece::Bishop(Side::White),
                Piece::Knight(Side::White),
                Piece::Rook(Side::White),
            ],
        }
    }

    fn as_array(&self) -> [&[Piece; 8]; 8] {
        [
            &self.rank8,
            &self.rank7,
            &self.rank6,
            &self.rank5,
            &self.rank4,
            &self.rank3,
            &self.rank2,
            &self.rank1,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_format() {
        assert_eq!(
            format!(
                "{}",
                Move {
                    piece: Piece::Rook(Side::White),
                    start_pos: Position(File::A, 3),
                    end_pos: Position(File::B, 6),
                    captured: false
                }
            ),
            "Ra3b6"
        );
    }

    #[test]
    fn test_capture_format() {
        assert_eq!(
            format!(
                "{}",
                Move {
                    piece: Piece::King(Side::White),
                    start_pos: Position(File::H, 2),
                    end_pos: Position(File::D, 5),
                    captured: true
                }
            ),
            "Kh2xd5"
        );
    }
}
