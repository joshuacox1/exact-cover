use std::collections::HashSet;

use arrayvec::ArrayVec;


/// A colour of a square in Kaleidoscope.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SquareColour { Black = 0, Red = 1, Blue = 2, Yellow = 3 }
use SquareColour::*;

use crate::{solver::ExactCoverProblem, sparse_binary_matrix::SparseBinaryMatrix};

use super::ExactCoverRepresentable;

/// A square to index the board with. Ensured to be between 0 and 7 inclusive.
pub struct Square { x: u8, y: u8 }

pub const SIDE_LENGTH: u8 = 8;

impl Square {
    /// Fallibly make a new board square.
    pub fn new(x: u8, y: u8) -> Result<Self, ()> {
        if 0 <= x && x < SIDE_LENGTH && 0 <= y && y < SIDE_LENGTH {
            Ok(Self { x, y })
        } else {
            Err(())
        }
    }

    pub fn x(&self) -> u8 { self.x }
    pub fn y(&self) -> u8 { self.y }
}



struct SquareAndColour(u8);

impl SquareAndColour {
    /// A square and color combo exactly fits in a byte; there are 64 squares
    /// and 4 colours.
    pub const fn encode(x: u8, y: u8, color: SquareColour) -> Self {
        SquareAndColour(x << 5 | y << 2 | color as u8)
    }

    /// A square and color combo exactly fits in a byte; there are 64 squares
    /// and 4 colours.
    pub const fn encode_sq(sq: Square, color: SquareColour) -> Self {
        SquareAndColour(sq.x << 5 | sq.y << 2 | color as u8)
    }

    pub const fn decode(sq_color: Self) -> (Square, SquareColour) {
        (Square {
            x : sq_color.0 >> 5,
            y : sq_color.0 >> 2 & 0b111,
        },
        match sq_color.0 & 0b11 {
            0b00 => SquareColour::Black, 0b01 => SquareColour::Red,
            0b10 => SquareColour::Blue, 0b11 => SquareColour::Yellow,
            _ => unreachable!(),
        })
    }
}





/// A piece in Kaleidoscope. L/J or S/Z refer to how the piece looks on the
/// red/black side.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KaleidoscopePiece {
    RedBlack1,
    BlackBlue1,
    Domino,
    Corner3Red2,
    Corner3Black2,
    Line3Red2,
    Line3Black2,
    L4RedLongEnd,
    L4RedShortEnd,
    J4RedLongEnd,
    J4RedShortEnd,
    S4,
    Z4,
    T4Red3,
    T4Black3,
    Box4,
    Line4,
    Line8,
}

const NUM_KALEIDOSCOPE_PIECES: usize = 18;

const PIECE_LENGTHS: [u8; NUM_KALEIDOSCOPE_PIECES] = [
    1,1,2,3,3,3,3,4,4,4,4,4,4,4,4,4,4,8,
];

const MAX_PIECE_SIZE: usize = 8;

// const fn q() -> ArrayVec<(Square, SquareColour), 2> {
//     let mut k = ArrayVec::<(Square, SquareColour), 2>::new_const();
//     k.push((Square { x: 3, y: 5 }, SquareColour::Black));
//     k
// }

const DUMMY: SquareAndColour = SquareAndColour(u8::MAX);

const SQUARE_LOOKUP_TABLE2: [[SquareAndColour; MAX_PIECE_SIZE]; 2] = [
    // RedBlack1: red/black side
    [
        SquareAndColour::encode(0, 0, Red),
        DUMMY,DUMMY,DUMMY,DUMMY,DUMMY,DUMMY,DUMMY,
    ],
    // RedBlack1: black/blue/yellow side
    [
        (Square { x: 0, y: 0}, SquareColour::Black),
        DUMMY,DUMMY,DUMMY,DUMMY,DUMMY,DUMMY,DUMMY,
    ],
    // 
    // RedBlack1: one side, then the other.
    // (Square(0,0), SquareColour::Red),
    // (Square(0,0), SquareColour::Black),
    // // BlackBlue1: one side, then the other.
    // (Square(0,0), SquareColour::Black),
    // (Square(0,0), SquareColour::Blue),
    // // Domino: one side, then the other.
    // (Square(0,0), SquareColour::Red),
    // (Square(0,1), SquareColour::Black),
    // (Square(0,0), SquareColour::Blue),
    // (Square(0,1), SquareColour::Black),
];

// // An array to use as a lookup table for piece colours.
// const SQUARE_LOOKUP_TABLE: [(Square, SquareColour); 62] = [
//     // RedBlack1: one side, then the other.
//     (Square(0,0), SquareColour::Red),
//     (Square(0,0), SquareColour::Black),
//     // BlackBlue1: one side, then the other.
//     (Square(0,0), SquareColour::Black),
//     (Square(0,0), SquareColour::Blue),
//     // Domino: one side, then the other.
//     (Square(0,0), SquareColour::Red),
//     (Square(0,1), SquareColour::Black),
//     (Square(0,0), SquareColour::Blue),
//     (Square(0,1), SquareColour::Black),
//     // Corner3Red2: red/black side.
//     (Square(0,0), SquareColour::Red),
//     (Square(0,1), SquareColour::Black),
//     (Square(0,0), SquareColour::Blue),
//     // Corner3Red2: black/blue/yellow side.
//     (Square(0,1), SquareColour::Black),
// ];

/// Which side a Kaleidoscope piece is on.
pub enum KaleidoscopePieceSide { RedBlack, BlueYellowBlack }

// impl KaleidoscopePiece {
//     pub fn length
// }



// const PIECE_SET = new Map([
//     ["O1R", [
//         [[[0,0],"R"]],
//         [[[0,0],"B"]],
//     ].sort()],
//     ["O1B", [
//         [[[0,0],"B"]],
//         [[[0,0],"L"]],
//     ].sort()],
//     ["I2", [
//         [[[0,0],"R"], [[0,1],"B"]],
//         [[[0,0],"B"], [[0,1],"L"]],
//     ].sort()],
//     ["I3R", [
//         [[[0,0],"R"], [[0,1],"B"], [[0,2],"R"]],
//         [[[0,0],"L"], [[0,1],"B"], [[0,2],"Y"]],
//     ].sort()],
//     ["I3B", [
//         [[[0,0],"B"], [[0,1],"R"], [[0,2],"B"]],
//         [[[0,0],"B"], [[0,1],"Y"], [[0,2],"B"]],
//     ].sort()],
//     ["L3R", [
//         [[[0,0],"R"], [[1,0],"B"], [[1,1],"R"]],
//         [[[0,0],"L"], [[1,0],"B"], [[1,1],"Y"]],
//     ].sort()],
//     ["L3B", [
//         [[[0,0],"B"], [[1,0],"R"], [[1,1],"B"]],
//         [[[0,0],"B"], [[1,0],"Y"], [[1,1],"B"]],
//     ].sort()],
//     ["L4BS", [
//         [[[2,0],"R"], [[1,0],"B"], [[0,0],"R"], [[0,1], "B"]],
//         [[[2,1],"B"], [[2,0],"L"], [[1,0],"B"], [[0,0], "Y"]],
//     ].sort()],
//     ["L4RS", [
//         [[[2,0],"B"], [[1,0],"R"], [[0,0],"B"], [[0,1], "R"]],
//         [[[2,1],"B"], [[2,0],"Y"], [[1,0],"B"], [[0,0], "L"]],
//     ].sort()],
//     ["G4BS", [
//         [[[2,1],"B"], [[2,0],"R"], [[1,0],"B"], [[0,0], "R"]],
//         [[[2,0],"B"], [[1,0],"Y"], [[0,0],"B"], [[0,1], "L"]],
//     ].sort()],
//     ["G4RS", [
//         [[[2,1],"R"], [[2,0],"B"], [[1,0],"R"], [[0,0], "B"]],
//         [[[2,0],"B"], [[1,0],"L"], [[0,0],"B"], [[0,1], "Y"]],
//     ].sort()],
//     ["S4", [
//         [[[0,0],"R"], [[0,1],"B"], [[1,1],"R"], [[1,2], "B"]],
//         [[[0,0],"Y"], [[1,0],"B"], [[1,1],"L"], [[2,1], "B"]],
//     ].sort()],
//     ["Z4", [
//         [[[0,0],"R"], [[1,0],"B"], [[1,1],"R"], [[2,1], "B"]],
//         [[[0,0],"L"], [[0,1],"B"], [[1,1],"Y"], [[1,2], "B"]],
//     ].sort()],
//     ["T4R", [
//         [[[0,0],"R"], [[1,0],"B"], [[2,0],"R"], [[1,1], "R"]],
//         [[[0,0],"B"], [[1,0],"L"], [[2,0],"B"], [[1,1], "B"]],
//     ].sort()],
//     ["T4B", [
//         [[[0,0],"B"], [[1,0],"R"], [[2,0],"B"], [[1,1], "B"]],
//         [[[0,0],"Y"], [[1,0],"B"], [[2,0],"L"], [[1,1], "Y"]],
//     ].sort()],
//     ["O4", [
//         [[[0,0],"B"], [[1,0],"R"], [[0,1],"R"], [[1,1], "B"]],
//         [[[0,0],"B"], [[1,0],"L"], [[0,1],"Y"], [[1,1], "B"]],
//     ].sort()],
//     ["L4", [
//         [[[0,0],"B"], [[1,0],"R"], [[2,0],"B"], [[3,0], "R"]],
//         [[[0,0],"B"], [[1,0],"Y"], [[2,0],"B"], [[3,0], "L"]],
//     ].sort()],
//     ["L8", [
//         [[[0,0],"B"], [[1,0],"R"], [[2,0],"B"], [[3,0], "R"], [[4,0],"B"], [[5,0],"R"], [[6,0],"B"], [[7,0], "R"]],
//         [[[0,0],"B"], [[1,0],"Y"], [[2,0],"B"], [[3,0], "L"], [[4,0],"B"], [[5,0],"Y"], [[6,0],"B"], [[7,0], "L"]],
//     ].sort()],
// ]);

// b: &KaleidoscopeBoard



/// A Kaleidoscope puzzle board. Each subarray is a row from bottom to top.
pub struct KaleidoscopeBoard([[SquareColour; SIDE_LENGTH as usize]; SIDE_LENGTH as usize]);

pub struct KaleidoscopeProblem {
    board: KaleidoscopeBoard,
    all_unique_valid_piece_placements: Vec<(KaleidoscopePiece, Vec<Square>)>,
}

impl KaleidoscopeProblem {
    pub fn new(board: KaleidoscopeBoard) -> Self {
        let all = Self::all_unique_valid_piece_placements(&board);
        Self {
            board,
            all_unique_valid_piece_placements: all,
        }
    }

    /// Returns all unique valid placements of any piece on a given puzzle board.
    /// The order is arbitrary.
    fn all_unique_valid_piece_placements(board: &KaleidoscopeBoard) -> Vec<(KaleidoscopePiece, Vec<Square>)> {
        unimplemented!()
    }

    fn exact_cover_problem(&self) -> ExactCoverProblem {
        // 18 + 64 = 82
        let num_cols = NUM_KALEIDOSCOPE_PIECES + SIDE_LENGTH as usize * SIDE_LENGTH as usize;
        let rows = self.all_unique_valid_piece_placements.iter()
            .map(|(piece, sqs)| {
                let piece_iter = std::iter::once(*piece as usize);
                let sq_iter = sqs.iter()
                    .map(|sq| NUM_KALEIDOSCOPE_PIECES + (sq.x + SIDE_LENGTH*sq.y) as usize);
                piece_iter.chain(sq_iter)
            });
        let matrix = SparseBinaryMatrix::from_sparse_rows(rows, num_cols).unwrap();
        crate::solver::ExactCoverProblem::new_standard(matrix)
    }
}
