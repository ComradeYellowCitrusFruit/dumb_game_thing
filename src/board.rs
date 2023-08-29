use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
enum PieceType {
    WhitePawn,
    BlackPawn,
    WhiteBishop,
    BlackBishop,
    WhiteKnight,
    BlackKnight,
    WhiteRook,
    BlackRook,
    WhiteKing,
    BlackKing,
    WhiteQueen,
    BlackQueen,
    Blank,
    OffBoard,
}

impl PieceType {
    pub fn is_white(&self) -> bool {
        (*self as u8) % 2 == 0
    }

    pub fn is_black(&self) -> bool {
        (*self as u8) % 2 == 1
    }
}

struct Board {
    pieces: [PieceType; 64],
    none: PieceType,
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 <= 8 && index.1 <= 8 {
            &mut self.pieces[index.0 + 8 * index.1]
        } else {
            self.none = PieceType::OffBoard;
            &mut self.none
        }
    }
}

impl Index<(usize, usize)> for Board {
    type Output = PieceType;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 <= 8 && index.1 <= 8 {
            &self.pieces[index.0 + 8 * index.1]
        } else {
            &self.none
        }
    }
}

impl Board {
    pub fn default_position() -> Self {
        let mut b = Board { pieces: [PieceType::Blank; 64], none: PieceType::OffBoard };

        for i in 0usize..7usize {
            b[(i, 6)] = PieceType::WhitePawn;
            b[(i, 1)] = PieceType::BlackPawn;
        }

        b[coords_from_an(('a', 8))] = PieceType::WhiteRook;
        b[coords_from_an(('h', 8))] = PieceType::WhiteRook;
        b[coords_from_an(('b', 8))] = PieceType::WhiteKnight;
        b[coords_from_an(('g', 8))] = PieceType::WhiteKnight;
        b[coords_from_an(('c', 8))] = PieceType::WhiteBishop;
        b[coords_from_an(('f', 8))] = PieceType::WhiteBishop;
        b[coords_from_an(('d', 8))] = PieceType::WhiteQueen;
        b[coords_from_an(('e', 8))] = PieceType::WhiteKing;

        b[coords_from_an(('a', 0))] = PieceType::BlackRook;
        b[coords_from_an(('h', 0))] = PieceType::BlackRook;
        b[coords_from_an(('b', 0))] = PieceType::BlackKnight;
        b[coords_from_an(('g', 0))] = PieceType::BlackKnight;
        b[coords_from_an(('c', 0))] = PieceType::BlackBishop;
        b[coords_from_an(('f', 0))] = PieceType::BlackBishop;
        b[coords_from_an(('d', 0))] = PieceType::BlackQueen;
        b[coords_from_an(('e', 0))] = PieceType::BlackKing;

        b
    }
}

pub fn coords_from_an(an: (char, i8)) -> (usize, usize) {
    let mut c: (usize, usize) = (0, 0);
    c.0 = match an.0 {
        'a' | 'A' => {
            0
        },
        'b' | 'B' => {
            1
        },
        'c' | 'C' => {
            2
        },
        'd' | 'D' => {
            3
        },
        'e' | 'E' => {
            4
        },
        'f' | 'F' => {
            5
        },
        'g' | 'G' => {
            6
        },
        'h' | 'H' => {
            7
        },
        _ => 8,
    };

    c.1 = (an.1 - 1) as usize;
    c
}