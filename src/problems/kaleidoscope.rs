// use std::collections::HashSet;
// use image::{RgbImage, Rgb};

// use crate::{ExactCoverProblem, ExactCoverSolver};

// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum Colour { Black, Red, Blue, Yellow }

// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Coord(pub i8, pub i8);

// impl Coord {
//     fn valid(&self) -> bool {
//         let Coord(x,y) = *self;
//         0 <= x && x < 8 && 0 <= y && y < 8
//     }
// }

// pub const NUM_PIECES: usize = 18;

// // First index = data idx, second index = length (size)
// const PIECE_INDICES: [(usize, usize); NUM_PIECES] = [
//     (0,1), // Red dot
//     (2,1), // Black dot
//     (4,2), // Domino
//     (8,3), // Red 3-line
//     (14,3), // Black 3-line
//     (20,3), // Red 3-corner
//     (26,3), // Black 3-corner
//     (32,4), // Ls in some order...
//     (40,4),
//     (48,4),
//     (56,4),
//     (64,4), // The S
//     (72,4), // The Z
//     (80,4), // The red T
//     (88,4), // The black T
//     (96,4), // The box
//     (104,4), // The 4-line
//     (112,8), // The 8-line
// ];


// const B: Colour = Colour::Black;
// const R: Colour = Colour::Red;
// const L: Colour = Colour::Blue;
// const Y: Colour = Colour::Yellow;

// const SQUARE_DATA: [(Coord, Colour); 128] = [
//     // Idx 0, length 1: Red dot front
//     (Coord(0,0), R),
//     // Idx 1, length 1: red dot back
//     (Coord(0,0), B),
//     // Idx 2, length 1: black dot back
//     (Coord(0,0), B),
//     // Idx 3, length 1: black dot back
//     (Coord(0,0), L),

//     // Idx 4, length 2: domino front
//     (Coord(0,0), R), (Coord(1,0), B),
//     // Idx 6, length 2: domino back
//     (Coord(0,0), B), (Coord(1,0), L),

//     // Idx 8, length 3: red three-line front
//     (Coord(0,0), R), (Coord(1,0), B), (Coord(2,0), R),
//     // Idx 11, length 3: red three-line back
//     (Coord(0,0), L), (Coord(1,0), B), (Coord(2,0), Y),
//     // Idx 14, length 3: black three-line front
//     (Coord(0,0), B), (Coord(1,0), R), (Coord(2,0), B),
//     // Idx 17, length 3: black three-line back
//     (Coord(0,0), B), (Coord(1,0), Y), (Coord(2,0), B),

//     // Idx 20, length 3: red three-corner front
//     (Coord(0,0), R), (Coord(1,0), B), (Coord(1,1), R),
//     // Idx 23, length 3: red three-corner back
//     (Coord(0,0), L), (Coord(1,0), B), (Coord(1,1), Y),
//     // Idx 26, length 3: black three-corner front
//     (Coord(0,0), B), (Coord(1,0), R), (Coord(1,1), B),
//     // Idx 29, length 3: black three-corner back
//     (Coord(0,0), B), (Coord(1,0), Y), (Coord(1,1), B),

//     // Idx 32, length 4: L, two reds on the long side, front
//     (Coord(0,0), B), (Coord(1,0), R), (Coord(1,1), B), (Coord(1,2), R),
//     // Idx 36, length 4: L, two reds on the long side, back
//     (Coord(0,0), Y), (Coord(1,0), B), (Coord(2,0), L), (Coord(2,1), B),

//     // Idx 40, length 4: L, two blacks on the long side, front
//     (Coord(0,0), R), (Coord(1,0), B), (Coord(1,1), R), (Coord(1,2), B),
//     // Idx 44, length 4: L, two blacks on the long side, back
//     (Coord(0,0), L), (Coord(1,0), B), (Coord(2,0), Y), (Coord(2,1), B),

//     // Idx 48, length 4: J, two reds on the long side, front
//     (Coord(0,0), R), (Coord(1,0), B), (Coord(2,0), R), (Coord(2,1), B),
//     // Idx 52, length 4: J, two reds on the long side, back
//     (Coord(0,0), L), (Coord(1,0), B), (Coord(1,1), Y), (Coord(1,2), B),

//     // Idx 56, length 4: J, two blacks on the long side, front
//     (Coord(0,0), B), (Coord(1,0), R), (Coord(2,0), B), (Coord(2,1), R),
//     // Idx 60, length 4: J, two blacks on the long side, back
//     (Coord(0,0), Y), (Coord(1,0), B), (Coord(1,1), L), (Coord(1,2), B),

//     // Idx 64, length 4: S front
//     (Coord(0,0), R), (Coord(0,1), B), (Coord(1,1), R), (Coord(1,2), B),
//     // Idx 68, length 4: S back
//     (Coord(0,0), Y), (Coord(1,0), B), (Coord(1,1), L), (Coord(2,1), B),

//     // Idx 72, length 4: Z front
//     (Coord(0,0), R), (Coord(1,0), B), (Coord(1,1), R), (Coord(2,1), B),
//     // Idx 76, length 4: Z back
//     (Coord(0,0), L), (Coord(0,1), B), (Coord(1,1), Y), (Coord(1,2), B),

//     // Idx 80, length 4: Red T, front
//     (Coord(0,0), R), (Coord(1,0), B), (Coord(2,0), R), (Coord(1,1), R),
//     // Idx 84, length 4: Red T, back
//     (Coord(0,0), B), (Coord(1,0), L), (Coord(2,0), B), (Coord(1,1), B),

//     // Idx 88, length 4: Black T, front
//     (Coord(0,0), B), (Coord(1,0), R), (Coord(2,0), B), (Coord(1,1), B),
//     // Idx 92, length 4: Black T, back
//     (Coord(0,0), Y), (Coord(1,0), B), (Coord(2,0), L), (Coord(1,1), Y),

//     // Idx 96, length 4: Box front
//     (Coord(0,0), R), (Coord(1,0), B), (Coord(1,1), R), (Coord(0,1), B),
//     // Idx 100, length 4: Box back
//     (Coord(0,0), L), (Coord(1,0), B), (Coord(1,1), Y), (Coord(0,1), B),

//     // Idx 104, length 4: 4-line front
//     (Coord(0,0), R), (Coord(1,0), B), (Coord(2,0), R), (Coord(3,0), B),
//     // Idx 108, length 4: 4-line back
//     (Coord(0,0), L), (Coord(1,0), B), (Coord(2,0), Y), (Coord(3,0), B),

//     // Idx 112, length 8: 8-line front
//     (Coord(0,0), R), (Coord(1,0), B), (Coord(2,0), R), (Coord(3,0), B),
//     (Coord(4,0), R), (Coord(5,0), B), (Coord(6,0), R), (Coord(7,0), B),
//     // Idx 120, length 8: 8-line back
//     (Coord(0,0), L), (Coord(1,0), B), (Coord(2,0), Y), (Coord(3,0), B),
//     (Coord(4,0), L), (Coord(5,0), B), (Coord(6,0), Y), (Coord(7,0), B),
// ];


// // Each colour consists of 2 bits, so a board has 2*64 = 128 bits
// // which is 16 bytes. We can put this in a u128 perhaps?
// pub struct Board([[Colour; 8]; 8]);
// // Could impl the Index trait, I guess. Probably should

// impl Board {
//     // pub fn from(arr: [[Colour; 8]; 8]) -> Self {
//     //     let z = 0u128;
//     //     for (j, row) in arr.iter().enumerate() {
//     //         for (i, c) in row.iter().enumerate() {
//     //             let idx = 8*j + i;
//     //             z |= (c as u8) << (2*idx);
//     //         }
//     //     }

//     //     Self { z }
//     // }

//     // pub fn to_exact_cover
// }

// // There should be 5,720 rotations:
// // PIECE 0, len: 128
// // PIECE 1, len: 128
// // PIECE 2, len: 448
// // PIECE 3, len: 384
// // PIECE 4, len: 384
// // PIECE 5, len: 392
// // PIECE 6, len: 392
// // PIECE 7, len: 336
// // PIECE 8, len: 336
// // PIECE 9, len: 336
// // PIECE 10, len: 336
// // PIECE 11, len: 336
// // PIECE 12, len: 336
// // PIECE 13, len: 336
// // PIECE 14, len: 336
// // PIECE 15, len: 392
// // PIECE 16, len: 320
// // PIECE 17, len: 64
// // Encode this at compile time?
// pub fn generate_piece_rotations() -> [Vec<(Vec<(Coord, Colour)>, EdgeMask)>; NUM_PIECES] {
//     let mut all = [const { vec![] }; NUM_PIECES];

//     for (i, &(data_idx, data_len)) in PIECE_INDICES.iter().enumerate() {
//         let mut set = HashSet::new();
//         let idx1 = data_idx;
//         let idx2 = idx1+data_len;
//         let idx3 = idx2+data_len;
//         let front_data = &SQUARE_DATA[idx1..idx2];
//         rot_refl(front_data, &mut set);
//         let back_data = &SQUARE_DATA[idx2..idx3];
//         rot_refl(back_data, &mut set);

//         let result = &mut all[i];
//         result.extend(set.into_iter()
//             .map(|v| {
//                 let e = edge_mask(v.iter().map(|&(c,_)| c));
//                 (v, e)
//             }));
//         result.sort_unstable();
//     }

//     all
// }

// fn rot1(c: Coord) -> Coord { let Coord(x,y) = c; Coord(x,y) }
// fn rot2(c: Coord) -> Coord { let Coord(x,y) = c; Coord(-y,x) }
// fn rot3(c: Coord) -> Coord { let Coord(x,y) = c; Coord(-x,-y) }
// fn rot4(c: Coord) -> Coord { let Coord(x,y) = c; Coord(y,-x) }

// const ROTATIONS: [fn(Coord) -> Coord; 4] = [rot1, rot2, rot3, rot4];

// fn rot_refl(
//     data: &[(Coord, Colour)],
//     set: &mut HashSet<Vec<(Coord, Colour)>>,
// ) {
//     // Likely way wider bounds than needed, but just in case.
//     // This preprocessing step does not need to be optimised
//     // (in fact the results could be embedded in at compile time).
//     for j in -16..=16 {
//         for i in -16..=16 {
//             for rot in ROTATIONS {
//                 if let Some(pos) = transform(data, i, j, rot) {
//                     set.insert(pos);
//                 }
//             }
//         }
//     }
// }

// fn transform(
//     data: &[(Coord, Colour)],
//     i: i8,
//     j: i8,
//     rot: fn(Coord) -> Coord,
// ) -> Option<Vec<(Coord, Colour)>> {
//     let mut new = Vec::with_capacity(data.len());
//     for (Coord(x,y), colour) in data {
//         let new_coord = rot(Coord(x+i,y+j));
//         if new_coord.valid() {
//             new.push((new_coord, *colour));
//         } else {
//             return None;
//         }
//     }
//     new.sort_unstable();

//     Some(new)
// }


// // 82
// pub const NUM_CONSTRAINTS: usize = NUM_PIECES + 8*8;

// pub fn kaleidoscope_cover(board: &Board) {
//     let matching_rows = generate_piece_rotations().into_iter()
//         .enumerate()
//         .flat_map(|(i,p)| p.into_iter()
//             .filter(|(placement,_)|
//                 placement.iter().all(|&(Coord(x,y), c)|
//                     board.0[y as usize][x as usize] == c)
//             )
//             .map(move |(placement,edgemask)| (i, placement, edgemask))
//         ).collect::<Vec<_>>();

//     let exact_cover_problem_ones = matching_rows
//         .iter()
//         .map(move |(i, placement, _)| {
//             // Each row is the piece's ID in reverse order
//             // plus each square
//             let mut row = Vec::with_capacity(1+placement.len());
//             row.push(NUM_PIECES - 1 - i);
//             for &(Coord(x,y), _) in placement.iter() {
//                 row.push(NUM_PIECES + 8*(y as usize) + (x as usize));
//             }
//             row.sort_unstable();
//             row.into_iter()
//         });
//     let exact_cover_problem = ExactCoverProblem::new(
//         exact_cover_problem_ones, NUM_CONSTRAINTS, 0,
//     ).unwrap();

//     let mut solver = ExactCoverSolver::new(&exact_cover_problem);
//     let mut y = 0u64;
//     // for soln in solver.iter_solutions() {
//     //     y += 1;
//     //     if y % 1_000 == 0 {
//     //         println!("Churned through {y} solutions...");
//     //     }

//     //     // This is for printing solutions as u128 edge masks.
//     //     // Should keep this idea, probably.
//     //     // let mut em = 0u128;
//     //     // for row_idx in soln.0 {
//     //     //     let (piece_idx, plcment, edge_mask) = &matching_rows[row_idx];
//     //     //     em |= edge_mask;
//     //     //     // println!("{plcment:?}");
//     //     // }
//     //     // println!("{em:0128b}");
//     // }
//     solver.for_solutions(|_| {
//         y += 1;
//         if y % 100_000 == 0 {
//             println!("Churned through {y} solutions...");
//         }
//     }, None);
//     println!("{y} solutions in total.");


//     // let ones = generate_piece_rotations().iter()
//     //     .enumerate()
//     //     .flat_map(|(i,p)| p.iter()
//     //         .filter(|&(placement,_)|
//     //             placement.iter().all(|&(Coord(x,y), c)|
//     //                 board.0[y as usize][x as usize] == c)
//     //         )
//     //         // make this nice
//     //         .map(move |(placement,_)| {
//     //             // Each row is the piece's ID in reverse order
//     //             // plus each square
//     //             let mut row = Vec::with_capacity(1+placement.len());
//     //             row.push(NUM_PIECES - 1 - i);
//     //             for &(Coord(x,y), _) in placement {
//     //                 row.push(NUM_PIECES + 8*(y as usize) + (x as usize));
//     //             }
//     //             row.sort_unstable();
//     //             row.into_iter()
//     //         })
//     //     )
//     //     .collect::<Vec<_>>();
//     // // for o in ones {
//     // //     println!("{o:?}");
//     // // }
//     // // unimplemented!();
//     // ExactCoverProblem::new(ones.into_iter(), NUM_CONSTRAINTS, 0).unwrap()
// }


// pub const HOT_AIR_BALLOON: Board = Board([
//     [B,L,B,L,B,L,B,L],
//     [Y,B,Y,B,R,B,Y,B],
//     [B,Y,B,R,R,R,B,Y],
//     [L,B,R,R,B,R,R,B],
//     [B,L,B,R,R,R,B,L],
//     [Y,B,Y,B,R,B,Y,B],
//     [B,Y,B,Y,B,Y,B,Y],
//     [L,B,L,B,R,B,L,B],
// ]);

// pub const BABY_ELEPHANT: Board = Board([
//     [B,R,B,R,B,R,B,R],
//     [R,B,R,B,R,B,R,B],
//     [B,R,B,R,B,R,B,R],
//     [R,B,R,R,R,B,R,B],
//     [R,B,B,B,R,R,B,R],
//     [B,R,B,B,B,B,R,B],
//     [R,R,R,B,B,B,R,R],
//     [B,R,R,B,R,B,R,B],
// ]);

// pub const QUESTION_MARK: Board = Board([
//     [B,L,B,Y,B,L,B,Y],
//     [Y,B,L,R,R,B,L,B],
//     [B,Y,B,B,R,Y,B,L],
//     [L,B,Y,R,R,B,Y,B],
//     [B,L,B,R,B,L,B,Y],
//     [Y,B,L,B,Y,B,L,B],
//     [B,Y,B,R,B,Y,B,L],
//     [L,B,Y,B,L,B,Y,B],
// ]);

// pub const CHECKERBOARD: Board = Board([
//     [R,B,R,B,R,B,R,B],
//     [B,R,B,R,B,R,B,R],
//     [R,B,R,B,R,B,R,B],
//     [B,R,B,R,B,R,B,R],
//     [R,B,R,B,R,B,R,B],
//     [B,R,B,R,B,R,B,R],
//     [R,B,R,B,R,B,R,B],
//     [B,R,B,R,B,R,B,R],
// ]);

// // // There are 8*(8-1)*2 = 112 internal edges on an 8x8 square board.
// // // Use the low 112 bits of a u128 to encode this. A solution
// // // is then just an edge mask (visually at least).
// // // In this form the copies of two-black solutions are
// // // indistinguishable. May want to filter those out.
// type EdgeMask = u128;

// fn edge_mask(
//     coords: impl Iterator<Item = Coord>,
// ) -> EdgeMask {
//     let mut mask_inner = 0u128;
//     for Coord(x,y) in coords {
//         // edges (x,y,R),(x,y,D),(x-1,y,R),(x,y-1,D).
//         // only -1 if non-zero, only the thing i f7
//         // let (x,y) = (x as usize, y as usize);
//         // i8s are just large enough to << all the way to the end
//         // with +127.
//         let mut mask = 0u128;
//         if x < 7 { mask |= 1 << (8*y + x); }
//         if y < 7 { mask |= 1 << (64 + 8*y + x); }
//         if x > 0 { mask |= 1 << (8*y + x-1); }
//         if y > 0 { mask |= 1 << (64 + 8*(y-1) + x); }

//         mask_inner ^= mask;
//     }
//     mask_inner
// }


// const SQ_PIXEL_SIZE: u32 = 31;
// const WHITE_RGB: Rgb<u8> = Rgb([0xf0, 0xf0, 0xf0]);
// const GREY_RGB: Rgb<u8> = Rgb([0x60, 0x60, 0x60]);
// const BLACK_RGB: Rgb<u8> = Rgb([0x20, 0x20, 0x20]);
// const RED_RGB: Rgb<u8> = Rgb([0xe5, 0x12, 0x12]);
// const BLUE_RGB: Rgb<u8> = Rgb([0x0f, 0x6b, 0xff]);
// const YELLOW_RGB: Rgb<u8> = Rgb([0xdd, 0xaa, 0x00]);

// // all 1s is not a valid edge mask so use that as a less wasteful Option
// pub fn draw_board(board: &Board, /*edgem: Option<EdgeMask>*/) -> RgbImage {
//     let s1 = SQ_PIXEL_SIZE+1;
//     let image_size = 8*s1 + 1;
//     let mut img = RgbImage::new(image_size, image_size);

//     for y in 0..image_size {
//         for x in 0..image_size {
//             let x_mod = x.rem_euclid(s1);
//             let y_mod = y.rem_euclid(s1);
//             let x_div = x.div_euclid(s1) as usize;
//             let y_div = y.div_euclid(s1) as usize;
//             let px_colour = if x_mod == 0 || y_mod == 0 {
//                 // // Top-left corner of a square, or right/bottom
//                 // // edge of the entire image
//                 // if x_mod == y_mod || x_mod == 8 || y_mod == 8 {
//                 //     WHITE_RGB
//                 // } else {
//                 //     // x > y means a down

//                 // }
//                 // match x_mod.cmp(&y_mod) {
//                 //     // A corner
//                 //     Ordering::Eq => WHITE_RGB
//                 // }
//                 WHITE_RGB
//             } else {
//                 if x_div >= 8 || y_div >= 8 {
//                     println!("{x}, {y}, {x_div}, {y_div}, {x_mod}, {y_mod}, {s1}");
//                 }
//                 let colour = board.0[y_div][x_div];
//                 match colour {
//                     Colour::Black => BLACK_RGB,
//                     Colour::Red => RED_RGB,
//                     Colour::Blue => BLUE_RGB,
//                     Colour::Yellow => YELLOW_RGB,
//                 }
//             };
//             img.put_pixel(x, y, px_colour);
//         }
//     }

//     img.save("test_file.png").unwrap();

//     img
// }







// // fn edge_masks

// // // Precalculated edge matrix.
// // const EDGE_MASKS: [EdgeMask; 64] = [

// // ];

// // fn precalculate_edge_masks(coord: Coord) {
// //     let mut edge_masks = [0u128; 64];

// //     for j in 0..8 {
// //         for i in 0..8 {
// //             let idx = i + 8*j;
// //             // compute the edge mask
// //         }
// //     }
// // }


// // fn get_soln(z: impl Iterator<Item = &[Coord]>) -> EdgeMask {
// //     let mut soln = 0u128;

// //     // The edge boundary of a single piece is the set of edges
// //     // bordered by exactly one square coord within the piece.
// //     // (Bordered by two = internal edge, by zero = not adjacent).
// //     // So XORing individual edge masks obtains the piece boundary.
// //     // We can then OR the piece boundary masks together to
// //     // obtain the final solution mask.
// //     for piececoords in z {
// //         let piecemask = 0u128;
// //         for Coord(i,j) in piececoords {
// //             piecemask ^= EDGE_MASKS[i + 8*j];
// //         }
// //         soln |= piecemask;
// //     }

// //     soln
// // }

// // // Large buffer with indices into it a bit like the above.
// // // for each piece, an index into the buffer, we have piece lengths
// // // stored already so we just need a number for the amount of distinct
// // // placements. and then there will be (Coord, Colour) pairs for each.
// // const ALL_PIECE_ROTATION_DATA: [(Coord, Colour); 4204] = [];
// // // First item: the index. Second item: number of placements
// // // (so the buffer is piece length * num placements)
// // const ALL_PIECE_ROTATION_DATA_INDICES: [(usize, usize); NUM_PIECES] = [];

// // // Exact cover problem

// // fn exactcover(board: Board) -> ExactCoverProblem {
// //     // 82 columns: put the 18 pieces first, then the 64 squares.
// //     let mut cover = vec![];

// //     // One row per unique valid placement of a piece provided it satisfies
// //     // the board
// //     for (p_i,(idx,bufl)) in ALL_PIECE_ROTATION_DATA_INDICES.iter().enumerate() {
// //         let piece_length = PIECE_INDICES[p_i].1;
// //         for i in 0..bufl {
// //             let q = idx + piece_length*i;
// //             // Relevant of (coord, colour) data for this piece.
// //             let rel_data = &ALL_PIECE_ROTATION_DATA[q..q+piece_length];

// //             // Check all match the board
// //             if rel_data.map(|(c, cl)| board[c] == cl).all() {
// //                 let mut ec_row = Vec::with_capacity(1+piece_length);
// //                 ec_row.push(p_i);
// //                 for (Coord(i,j), _) in rel_data.iter() {
// //                     ec_row.push(NUM_PIECES + i + 8*j);
// //                 }
// //                 cover.push(ec_row);
// //             }
// //         }
// //     }

// //     // Construct the cover object. There are no secondary columns
// //     cover
// // }


// // const ROTATIONS = [
// //     (x,y) => [x,y],
// //     (x,y) => [-y,x],
// //     (x,y) => [-x,-y],
// //     (x,y) => [y,-x],
// // ];

// // const transform = (side, f) => {
// //     const results = [];
// //     for (const cell of side) {
// //         let [[x,y], colour] = cell;
// //         let [x_,y_] = f(x,y);
// //         // console.log(cell, x,y,colour,x_,y_);
// //         if (0 <= x_ && x_ < 8 && 0 <= y_ && y_ < 8) {
// //             results.push([[x_,y_], colour]);
// //         } else {
// //             return null;
// //         }
// //     }
// //     return results.sort();
// // }

// // const SETS = new Map();
// // for (const [pieceName, sides] of PIECE_SET.entries()) {
// //     const placementSet = new Set();
// //     for (const side of sides) {
// //         for (let i = -8; i <= 8; i++) {
// //             for (let j = -8; j <= 8; j++) {
// //                 for (const rotation of ROTATIONS) {
// //                     const pos = transform(side, (x,y) => rotation(x+i,y+j));
// //                     if (pos !== null) {
// //                         placementSet.add(JSON.stringify(pos));
// //                     }
// //                 }
// //             }
// //         }
// //     }
// //     SETS.set(pieceName, placementSet);
// // }
// // const ALL_PLACEMENTS = new Map();
// // for (const [pieceName, placementStrs] of SETS.entries()) {
// //     const list = [];
// //     for (const placementStr of placementStrs) {
// //         const placement = JSON.parse(placementStr);
// //         list.push(placement);
// //     }
// //     ALL_PLACEMENTS.set(pieceName, list);
// // }

// // const NUM_PIECES = PIECE_SET.size;
// // const PIECE_NAME_INDEX = new Map();

// // const f = () => {
// //     let i = 0;
// //     for (const pieceName of PIECE_SET.keys()) {
// //         PIECE_NAME_INDEX.set(pieceName, i);
// //         i += 1;
// //     }
// // }
// // f();


// // /* Examples */


// // const GAMES_BOARD = (() => {
// //     const b = "B";
// //     const r = "R";
// //     const l = "L";
// //     const y = "Y";
// //     return [
// //         [l,b,l,b,l,b,l,b],
// //         [b,y,b,y,b,y,b,l],
// //         [l,b,y,b,y,b,y,b],
// //         [b,y,b,l,b,y,b,l],
// //         [l,b,y,b,l,b,y,b],
// //         [b,y,b,y,b,y,b,l],
// //         [l,b,y,b,y,b,y,b],
// //         [b,l,b,l,b,l,b,l],
// //     ];
// // })();
// // const STARSHIP = (() => {
// //     const b = "B";
// //     const r = "R";
// //     const l = "L";
// //     const y = "Y";
// //     return [
// //         [b,l,b,l,b,y,b,l],
// //         [l,b,b,b,b,b,l,b],
// //         [b,b,y,y,y,y,b,y],
// //         [l,b,y,y,b,b,l,b],
// //         [b,b,y,b,l,l,b,l],
// //         [y,b,y,b,l,b,y,b],
// //         [b,l,b,l,b,y,b,y],
// //         [l,b,y,b,l,b,y,b],
// //     ];
// // })();
// // const GOLDFISH = (() => {
// //     const b = "B";
// //     const r = "R";
// //     const l = "L";
// //     const y = "Y";
// //     return [
// //         [b,y,b,l,b,y,b,l],
// //         [y,b,b,b,y,b,l,b],
// //         [b,b,r,r,b,l,b,y],
// //         [l,b,r,r,r,b,y,b],
// //         [b,y,b,r,b,y,b,l],
// //         [y,b,l,b,y,b,l,b],
// //         [b,l,b,y,b,l,b,y],
// //         [l,b,y,b,l,b,y,b],
// //     ];
// // })();

// // const DOMINOES_CLASH = [
// //     ["B","B","R","R","B","B","R","R"],
// //     ["B","R","B","B","R","R","B","B"],
// //     ["R","B","R","R","B","B","R","R"],
// //     ["R","B","R","B","R","R","B","B"],
// //     ["B","R","B","R","B","B","R","R"],
// //     ["B","R","B","R","B","R","B","B"],
// //     ["R","B","R","B","R","B","R","R"],
// //     ["R","B","R","B","R","B","R","B"],
// // ];
// // const SIGNAL_MAN = [
// //     ["B","R","B","R","B","R","B","R"],
// //     ["R","B","R","B","R","B","R","B"],
// //     ["B","R","B","R","R","R","B","R"],
// //     ["R","B","B","B","Y","B","Y","B"],
// //     ["B","Y","Y","L","B","L","Y","B"],
// //     ["B","B","B","L","L","L","B","B"],
// //     ["R","B","B","R","R","R","B","R"],
// //     ["B","B","R","R","B","R","R","B"],
// // ];
// // const LONG_STEM_ROSE = (() => {
// //     const b = "B";
// //     const r = "R";
// //     const l = "L";
// //     const y = "Y";
// //     return [
// //         [l,b,y,b,l,b,y,b],
// //         [b,l,b,r,b,y,b,y],
// //         [y,b,r,r,b,b,y,b],
// //         [b,r,r,r,b,l,b,l],
// //         [l,b,b,b,r,b,l,b],
// //         [b,y,b,l,b,r,b,y],
// //         [y,b,y,b,l,b,r,b],
// //         [b,y,b,l,b,y,b,l],
// //     ];
// // })();
// // const VOLCANO = (() => {
// //     const b = "B";
// //     const r = "R";
// //     const l = "L";
// //     const y = "Y";
// //     return [
// //         [l,b,r,b,r,b,r,b],
// //         [b,l,b,r,b,r,b,r],
// //         [r,b,y,b,r,b,b,b],
// //         [b,r,b,y,b,b,r,r],
// //         [r,b,r,b,r,r,r,b],
// //         [b,r,b,b,r,r,b,r],
// //         [r,b,b,r,r,b,r,b],
// //         [b,r,b,r,b,r,b,r],
// //     ];
// // })();
// // const HOT_AIR_BALLOON = (() => {
// //     const b = "B";
// //     const r = "R";
// //     const l = "L";
// //     const y = "Y";
// //     return [
// //         [b,l,b,l,b,l,b,l],
// //         [y,b,y,b,r,b,y,b],
// //         [b,y,b,r,r,r,b,y],
// //         [l,b,r,r,b,r,r,b],
// //         [b,l,b,r,r,r,b,l],
// //         [y,b,y,b,r,b,y,b],
// //         [b,y,b,y,b,y,b,y],
// //         [l,b,l,b,r,b,l,b],
// //     ];
// // })();
// // const CITY_SQUARE = (() => {
// //     const b = "B";
// //     const r = "R";
// //     return [
// //         [r,b,r,b,r,b,r,b],
// //         [b,b,r,b,r,b,r,r],
// //         [r,r,b,r,b,r,b,b],
// //         [b,b,r,b,r,b,r,r],
// //         [r,r,b,r,b,r,b,b],
// //         [b,b,r,b,r,b,r,r],
// //         [r,r,b,r,b,r,b,b],
// //         [b,r,b,r,b,r,b,r],
// //     ];
// // })();
// // const BAMBI = (() => {
// //     const b = "B";
// //     const r = "R";
// //     return [
// //         [b,r,b,r,b,r,b,r],
// //         [r,b,r,b,r,b,r,b],
// //         [b,r,b,r,r,r,b,r],
// //         [r,b,b,b,r,r,b,b],
// //         [b,r,r,r,r,b,b,r],
// //         [b,b,r,r,r,b,r,b],
// //         [r,b,r,b,r,b,b,r],
// //         [b,b,r,b,r,b,r,b],
// //     ];
// // })();
// // const THE_SPIDER = (() => {
// //     const b = "B";
// //     const r = "R";
// //     const l = "L";
// //     return [
// //         [r,b,r,b,r,b,r,b],
// //         [b,r,b,r,b,r,r,r],
// //         [r,b,r,r,r,b,b,b],
// //         [b,r,r,b,b,b,r,r],
// //         [r,b,r,b,b,l,b,b],
// //         [b,r,b,b,l,b,r,r],
// //         [r,r,b,r,b,r,b,b],
// //         [b,r,b,r,b,r,b,r],
// //     ];
// // })();
// // const CUSTOM = [
// //     ["UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET"],
// //     ["UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET"],
// //     ["UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET"],
// //     ["UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET"],
// //     ["UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET"],
// //     ["UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET"],
// //     ["UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET"],
// //     ["UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET", "UNSET"],
// // ];

// // const EXAMPLES = [
// //     ["example1", "custom", CUSTOM],
// //     ["example2", "elephant", BABY_ELEPHANT],
// //     ["example3", "starship", STARSHIP],
// //     ["example4", "city-square", CITY_SQUARE],
// //     ["example5", "volcano", VOLCANO],
// //     ["example6", "long-stem-rose", LONG_STEM_ROSE],
// //     ["example7", "bambi", BAMBI],
// //     ["example8", "hot-air-balloon", HOT_AIR_BALLOON],
// //     ["example9", "question-mark", QUESTION_MARK],
// //     ["example10", "signal-man", SIGNAL_MAN],
// //     ["example11", "games-board", GAMES_BOARD],
// //     ["example12", "the-spider", THE_SPIDER],
// // ];

// // /* Now the code */

// // // Takes a board array a[8][8] and turns it into a generator of solutions.
// // // set showPartialSolutions to true to get partial solutions
// // function* solveBoard(board, showPartialSolutions) {
// //     const NUM_CONSTRAINT_COLUMNS = NUM_PIECES + 8*8; // 82
// //     const placementIndex = [];
// //     const constraints = [];
// //     for (const [pieceName, placements] of ALL_PLACEMENTS.entries()) {
// //         for (const placement of placements) {
// //             if (placement.every(cell => {
// //                 const [[x,y], c] = cell;
// //                 return board[y][x] === c; // important y/x flip
// //             })) {
// //                 placementIndex.push([pieceName, placement]);
// //                 // There are 18+64=82 constraint columns: 18, one for
// //                 // each piece, then 64, one for each square.
// //                 const constraint = [PIECE_NAME_INDEX.get(pieceName)];
// //                 for (const cell of placement) {
// //                     const [[x,y], _] = cell;
// //                     constraint.push(NUM_PIECES+x+y*8);
// //                 }
// //                 constraints.push(constraint);
// //             }
// //         }
// //     }

// //     const gen = dlxSolveOnes(
// //         constraints, NUM_CONSTRAINT_COLUMNS, showPartialSolutions);

// //     for (const output of gen) {
// //         let [outputType, result] = output;
// //         const actual = result.map(i => placementIndex[i]);
// //         yield [outputType, actual];
// //     }
// // }
