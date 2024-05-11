use super::*;

type Move = String;

fn optimal(state: GameState) -> GameState {
    todo!()
}

impl GameState {
    pub fn get_moves(self) -> Vec<(GameState, Move)> {
        let mut moves: Vec<(GameState, Move)> = Default::default();
        let coords =
            (0..self.board.len()).flat_map(move |x| (0..self.board[0].len()).map(move |y| (x, y)));
        // let coord_pairs = coords
        //     .clone()
        //     .flat_map(move |here| coords.clone().map(move |there| (here, there)))
        //     .filter(|(here, there)| here != there);
        // for (here, there) in coord_pairs {
        //     if self.board[here.1][here.0].can_cap(
        //         (
        //             there.0 as i32 - here.0 as i32,
        //             there.1 as i32 - here.1 as i32,
        //         ),
        //         self.board,
        //     ) {
        //         todo!()
        //     }
        // }
        for (x, y) in coords {
            if self.board[y][x].allegiance() == self.turn_player.side() {
                moves.append(&mut self.board[y][x].find_legal_moves((x, y), self))
            }
        }
        moves
    }
}

impl Piece {
    fn moveset(self) -> (u8, u16, u8) {
        match self {
            Piece::None => (0, 0, 0),
            Piece::PawnW => (0b000_00_101, 0, 0),
            Piece::PawnB => (0b101_00_000, 0, 0),
            Piece::KnightW => (0x0, 0b01010_11_00_11_01010, 0),
            Piece::KnightB => (0x0, 0b01010_11_00_11_01010, 0),
            Piece::BishopW => (0b101_00_101, 0, 0b101_00_101),
            Piece::BishopB => (0b101_00_101, 0, 0b101_00_101),
            Piece::RookW => (0b010_11_010, 0, 0b010_11_010),
            Piece::RookB => (0b010_11_010, 0, 0b010_11_010),
            Piece::QueenW => (0xff, 0, 0xff),
            Piece::QueenB => (0xff, 0, 0xff),
            Piece::KingW => (0xff, 0, 0),
            Piece::KingB => (0xff, 0, 0),
        }
    }
    fn can_cap(self, here: (usize, usize), there: (usize, usize), b: Board) -> bool {
        let pos = (
            there.0 as i32 - here.0 as i32,
            there.1 as i32 - here.1 as i32,
        );
        match self {
            Piece::None => false,
            Piece::PawnW => pos.1 == -1 && pos.0.abs() == 1,
            Piece::PawnB => pos.1 == 1 && pos.0.abs() == 1,
            Piece::KnightW => {
                (pos.0.abs() == 2 && pos.1.abs() == 1) || (pos.0.abs() == 1 && pos.1.abs() == 2)
            }
            Piece::KnightB => {
                (pos.0.abs() == 2 && pos.1.abs() == 1) || (pos.0.abs() == 1 && pos.1.abs() == 2)
            }
            Piece::BishopW => pos.0 == pos.1 || pos.0 == -pos.1,
            Piece::BishopB => pos.0 == pos.1 || pos.0 == -pos.1,
            Piece::RookW => pos.0 == 0 || pos.1 == 0,
            Piece::RookB => pos.0 == 0 || pos.1 == 0,
            Piece::QueenW => pos.0 == 0 || pos.1 == 0 || (pos.0 + pos.1).abs() == 0,
            Piece::QueenB => pos.0 == 0 || pos.1 == 0 || (pos.0 - pos.1).abs() == 0,
            Piece::KingW => pos.0.abs() == 1 || pos.1.abs() == 1,
            Piece::KingB => pos.0.abs() == 1 || pos.1.abs() == 1,
        }
    }

    fn find_legal_moves(self, here: (usize, usize), gs: GameState) -> Vec<(GameState, String)> {
        let coords =
            (0..gs.board.len()).flat_map(move |x| (0..gs.board[0].len()).map(move |y| (x, y)));
        match self {
            Piece::None => vec![],
            Piece::PawnW => {
                let mut m = vec![];
                for (x, y) in [
                    (here.0.wrapping_sub(1), here.1.wrapping_sub(1)),
                    (here.0 + 1, here.1.wrapping_sub(1)),
                ]
                .into_iter()
                .filter(|(x, y)| *x < gs.board[0].len() && *y < gs.board.len())
                {
                    if gs.board[y][x].allegiance() == -self.allegiance() {
                        m.push((
                            GameState {
                                board: gs
                                    .set_pieces(vec![(here.0, here.1, Piece::None), (x, y, self)])
                                    .board,
                                turn_player: gs.turn_player.next(),
                                turns_elapsed: gs.turns_elapsed + 1,
                            },
                            "test".into(),
                        ))
                    }
                }
                m
            }
            Piece::PawnB => todo!(),
            Piece::KnightW => todo!(),
            Piece::KnightB => todo!(),
            Piece::BishopW => todo!(),
            Piece::BishopB => todo!(),
            Piece::RookW => todo!(),
            Piece::RookB => todo!(),
            Piece::QueenW => todo!(),
            Piece::QueenB => todo!(),
            Piece::KingW => todo!(),
            Piece::KingB => todo!(),
        }
    }
}

// fn legal_moves(gs: GameState, x: usize, y: usize) -> Vec<(GameState, Move)> {
//     match gs.board[y][x] {
//         Piece::None => vec![],
//         Piece::PawnW => match gs.turn_player {
//             Player::White => {
//                 todo!()
//             }
//             _ => vec![],
//         },
//         Piece::PawnB => match gs.turn_player {
//             Player::Black => todo!(),
//             _ => vec![],
//         },
//         Piece::KnightW => match gs.turn_player {
//             Player::White => todo!(),
//             _ => vec![],
//         },
//         Piece::KnightB => match gs.turn_player {
//             Player::Black => todo!(),
//             _ => vec![],
//         },
//         Piece::BishopW => match gs.turn_player {
//             Player::White => todo!(),
//             _ => vec![],
//         },
//         Piece::BishopB => match gs.turn_player {
//             Player::Black => todo!(),
//             _ => vec![],
//         },
//         Piece::RookW => match gs.turn_player {
//             Player::White => todo!(),
//             _ => vec![],
//         },
//         Piece::RookB => match gs.turn_player {
//             Player::Black => todo!(),
//             _ => vec![],
//         },
//         Piece::QueenW => match gs.turn_player {
//             Player::White => todo!(),
//             _ => vec![],
//         },
//         Piece::QueenB => match gs.turn_player {
//             Player::Black => todo!(),
//             _ => vec![],
//         },
//         Piece::KingW => match gs.turn_player {
//             Player::White => todo!(),
//             _ => vec![],
//         },
//         Piece::KingB => match gs.turn_player {
//             Player::Black => todo!(),
//             _ => vec![],
//         },
//     }
// }
