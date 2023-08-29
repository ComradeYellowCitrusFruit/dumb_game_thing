use std::cmp::{max, min};
use crate::board::{self, Board};

pub enum Color {
    White,
    Black,
}

pub struct CpuPlayer {
    pub color: Color,
    level: i8,
}

impl CpuPlayer {
    pub fn new(c: Color, level: i8) -> CpuPlayer {
        CpuPlayer { color: c, level: max(min(level, 10), 1) }
    }

    pub fn next_move(&self, b: Board) -> Board {
        let depth = (self.level as i32) + max((self.level as i32)/2, 1);
        let skew = max(5 - self.level, 0);

        b
    }
}