// This file is part of the solibored application.
// It is heavily based on the shakmaty library
// Copyright (C) 2017-2021 Niklas Fiekas <niklas.fiekas@backscattering.de>
// Copyright (C) 2022 Lakin Wecker <niklas.fiekas@backscattering.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::{fmt, fmt::Write};

use crate::{
    bitboard::Bitboard,
    square::{File, Rank, Square},
};

/// [`Piece`] positions on a board.
///
/// # Examples
///
/// ```
/// use soliboard::{Square, Board};
///
/// let board = Board::new();
/// // . . x x x . .
/// // . x x x x x .
/// // x x x x x x x
/// // x x x x x x x
/// // x x x x x x x
/// // . x x x x x .
/// // . . x x x . .
///
/// assert!(board.piece_at(Square::D4));
/// ```
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Board {
    occupied: Bitboard,
}

impl Board {
    pub fn new() -> Board {
        Board {
            occupied: Bitboard(0x00_38_7c_fe_fe_fe_7c_38),
        }
    }

    pub fn empty() -> Board {
        Board {
            occupied: Bitboard(0),
        }
    }

    #[inline]
    pub fn occupied_at(&self, sq: Square) -> bool {
        self.occupied.contains(sq)
    }

    #[inline]
    pub fn remove_at(&mut self, sq: Square) -> bool {
        if self.occupied_at(sq) {
            self.occupied.toggle(sq);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn discard_at(&mut self, sq: Square) {
        self.occupied.discard(sq);
    }

    #[inline]
    pub fn occupy(&mut self, sq: Square) {
        if sq == Square::_A1
            || sq == Square::_A2
            || sq == Square::_A3
            || sq == Square::_A4
            || sq == Square::_A5
            || sq == Square::_A6
            || sq == Square::_A7
            || sq == Square::_B1
            || sq == Square::_B2
            || sq == Square::_B6
            || sq == Square::_B7
            || sq == Square::_C1
            || sq == Square::_C7
            || sq == Square::_G1
            || sq == Square::_G7
            || sq == Square::_A8
            || sq == Square::_B8
            || sq == Square::_C8
            || sq == Square::_D8
            || sq == Square::_E8
            || sq == Square::_F8
            || sq == Square::_G8
            || sq == Square::_H8
        {
            return;
        }
        self.discard_at(sq);
        self.occupied.toggle(sq);
    }

    fn transform<F>(&mut self, f: F)
    where
        F: Fn(Bitboard) -> Bitboard,
    {
        // In order to guarantee consistency, this method cannot be public
        // for use with custom transformations.
        self.occupied = f(self.occupied)
    }

    /// Mirror the board vertically. See [`Bitboard::flip_vertical`].
    pub fn flip_vertical(&mut self) {
        self.transform(Bitboard::flip_vertical);
        self.occupied = Bitboard(self.occupied.0 >> 8)
    }

    /// Mirror the board horizontally. See [`Bitboard::flip_horizontal`].
    pub fn flip_horizontal(&mut self) {
        self.transform(Bitboard::flip_horizontal);
        self.occupied = Bitboard(self.occupied.0 << 1)
    }

    /// Mirror the board at the a1-h8 diagonal.
    /// See [`Bitboard::flip_diagonal`].
    pub fn flip_diagonal(&mut self) {
        self.transform(Bitboard::flip_diagonal);
        self.occupied = Bitboard(self.occupied.0 >> 7)
    }

    /// Mirror the board at the h1-a8 diagonal.
    /// See [`Bitboard::flip_anti_diagonal`].
    pub fn flip_anti_diagonal(&mut self) {
        self.transform(Bitboard::flip_anti_diagonal)
    }

    /// Rotate the board 90 degrees clockwise. See [`Bitboard::rotate_90`].
    pub fn rotate_90(&mut self) {
        self.transform(Bitboard::rotate_90);
        self.occupied = Bitboard(self.occupied.0 << 1)
    }

    /// Rotate the board 180 degrees. See [`Bitboard::rotate_180`].
    pub fn rotate_180(&mut self) {
        self.transform(Bitboard::rotate_180);
        self.occupied = Bitboard(self.occupied.0 >> 7)
    }

    /// Rotate the board 270 degrees clockwise. See [`Bitboard::rotate_270`].
    pub fn rotate_270(&mut self) {
        self.transform(Bitboard::rotate_270);
        self.occupied = Bitboard(self.occupied.0 >> 8)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in Rank::ALL.into_iter().rev() {
            //if rank == Rank::Eighth {
            //continue;
            //}
            for file in File::ALL {
                //if file == File::A {
                //continue;
                //}
                let square = Square::from_coords(file, rank);
                f.write_char(if self.occupied_at(square) { 'o' } else { '.' })?;
                f.write_char(if file < File::H { ' ' } else { '\n' })?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_at() {
        let board = Board::new();
        assert!(!board.occupied_at(Square::_A1));
        assert!(!board.occupied_at(Square::_A2));
        assert!(!board.occupied_at(Square::_A3));
        assert!(!board.occupied_at(Square::_A4));
        assert!(!board.occupied_at(Square::_A5));
        assert!(!board.occupied_at(Square::_A6));
        assert!(!board.occupied_at(Square::_A7));
        assert!(!board.occupied_at(Square::_A8));
        assert!(!board.occupied_at(Square::_B1));
        assert!(!board.occupied_at(Square::_B2));
        assert!(!board.occupied_at(Square::_C1));
        assert!(!board.occupied_at(Square::_C1));
        assert!(board.occupied_at(Square::C2));
        assert!(board.occupied_at(Square::D1));
        assert!(board.occupied_at(Square::D2));
    }

    #[test]
    fn test_set_piece_at() {
        let mut board = Board::new();
        board.occupy(Square::D3);
        assert!(board.occupied_at(Square::D3));
    }

    #[test]
    fn test_board_transformation() {
        let compare_trans = |board1: &Board, trans: &dyn Fn(&mut Board), board2: &Board| {
            let mut board_trans = board1.clone();
            trans(&mut board_trans);
            assert_eq!(board_trans, board2.clone());
        };

        let mut board1: Board = Board::new();
        board1.remove_at(Square::D4);
        board1.remove_at(Square::D3);
        let mut board2: Board = Board::new();
        board2.remove_at(Square::D4);
        board2.remove_at(Square::D5);
        compare_trans(&board1, &Board::flip_vertical, &board2);

        let mut board1: Board = Board::new();
        board1.remove_at(Square::D4);
        board1.remove_at(Square::E4);
        let mut board2: Board = Board::new();
        board2.remove_at(Square::E4);
        board2.remove_at(Square::F4);
        compare_trans(&board1, &Board::flip_horizontal, &board2);

        let mut board1: Board = Board::new();
        board1.remove_at(Square::D4);
        board1.remove_at(Square::E5);
        let mut board2: Board = Board::new();
        board2.remove_at(Square::E3);
        board2.remove_at(Square::F4);
        compare_trans(&board1, &Board::flip_diagonal, &board2);

        let mut board1: Board = Board::new();
        board1.remove_at(Square::D4);
        board1.remove_at(Square::E3);
        let mut board2: Board = Board::new();
        board2.remove_at(Square::F4);
        board2.remove_at(Square::E5);
        compare_trans(&board1, &Board::flip_anti_diagonal, &board2);

        let mut board1: Board = Board::new();
        board1.remove_at(Square::D4);
        board1.remove_at(Square::D5);
        let mut board2: Board = Board::new();
        board2.remove_at(Square::E5);
        board2.remove_at(Square::F5);
        compare_trans(&board1, &Board::rotate_90, &board2);

        let mut board2: Board = Board::new();
        board2.remove_at(Square::F4);
        board2.remove_at(Square::F3);
        compare_trans(&board1, &Board::rotate_180, &board2);

        let mut board2: Board = Board::new();
        board2.remove_at(Square::E3);
        board2.remove_at(Square::D3);
        compare_trans(&board1, &Board::rotate_270, &board2);
    }
}
