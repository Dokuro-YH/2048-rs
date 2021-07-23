use rand::Rng;

use crate::moves::MOVES;
use crate::ROW_MASK;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    pub board: u64,
}

impl Game {
    /// Constructs a new `Game` and spawn tow tile.
    pub fn new() -> Self {
        let mut game = Self {
            board: 0x0000_0000_0000_0000_u64,
        };

        game.board |= game.spawn_tile();
        game.board |= game.spawn_tile();

        game
    }

    pub fn with(board: u64) -> Self {
        Game { board }
    }

    pub fn execute(&mut self, direction: Direction) {
        let board = self.board;
        let result_board = match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        };

        if board != result_board {
            self.board |= self.spawn_tile();
        }
    }

    pub fn score(&self) -> u64 {
        let table = &MOVES.scores;

        let score = table[((self.board >> 0) & ROW_MASK) as usize]
            + table[(self.board >> 16 & ROW_MASK) as usize]
            + table[(self.board >> 32 & ROW_MASK) as usize]
            + table[(self.board >> 48 & ROW_MASK) as usize];

        score
    }

    pub fn is_complete(&self) -> bool {
        todo!()
    }

    /// Returns a transposed board where row are transformed into columns and vice versa.
    ///
    /// # Example
    ///
    /// ```
    /// use rim::Game;
    ///
    /// let game = Game::with(0xFEDC_BA98_7654_3210);
    /// let result = game.transpose();
    ///
    /// // | F | E | D | C |      | F | B | 7 | 3 |
    /// // | B | A | 9 | 8 |  =>  | E | A | 6 | 2 |
    /// // | 7 | 6 | 5 | 4 |      | D | 9 | 5 | 1 |
    /// // | 3 | 2 | 1 | 0 |      | C | 8 | 4 | 0 |
    ///
    /// assert_eq!(result, 0xFB73_EA62_D951_C840);
    /// ```
    pub fn transpose(&self) -> u64 {
        let board = &self.board;
        let a1 = board & 0xF0F0_0F0F_F0F0_0F0F_u64;
        let a2 = board & 0x0000_F0F0_0000_F0F0_u64;
        let a3 = board & 0x0F0F_0000_0F0F_0000_u64;

        let a = a1 | (a2 << 12) | (a3 >> 12);

        let b1 = a & 0xFF00_FF00_00FF_00FF_u64;
        let b2 = a & 0x00FF_00FF_0000_0000_u64;
        let b3 = a & 0x0000_0000_FF00_FF00_u64;

        b1 | (b2 >> 24) | (b3 << 24)
    }

    pub fn move_up(&self) -> u64 {
        let transposed = self.transpose();
        let mut result = self.board;

        result ^= MOVES.up[((transposed >> 0) & ROW_MASK) as usize] << 0;
        result ^= MOVES.up[((transposed >> 16) & ROW_MASK) as usize] << 4;
        result ^= MOVES.up[((transposed >> 32) & ROW_MASK) as usize] << 8;
        result ^= MOVES.up[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    pub fn move_down(&self) -> u64 {
        let transposed = self.transpose();
        let mut result = self.board;

        result ^= MOVES.down[((transposed >> 0) & ROW_MASK) as usize] << 0;
        result ^= MOVES.down[((transposed >> 16) & ROW_MASK) as usize] << 4;
        result ^= MOVES.down[((transposed >> 32) & ROW_MASK) as usize] << 8;
        result ^= MOVES.down[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    pub fn move_left(&self) -> u64 {
        let board = &self.board;
        let mut result = self.board;

        result ^= MOVES.left[((board >> 0) & ROW_MASK) as usize] << 0;
        result ^= MOVES.left[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.left[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.left[((board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    pub fn move_right(&self) -> u64 {
        let board = &self.board;
        let mut result = self.board;

        result ^= MOVES.right[((board >> 0) & ROW_MASK) as usize] << 0;
        result ^= MOVES.right[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.right[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.right[((board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    /// Returns the count of tiles with a value of `0`
    ///
    /// # Example
    ///
    /// ```
    /// use rim::Game;
    ///
    /// let game = Game::with(0x0100_0020_0001_0001);
    /// let result = game.count_empty();
    ///
    /// assert_eq!(12, result);
    /// ```
    pub fn count_empty(&self) -> u32 {
        let mut empty = 0;

        for i in 0..16 {
            if self.board >> (i * 4) & 0xF == 0 {
                empty += 1
            }
        }

        empty
    }

    /// Returns `1` with 90% chance and `2` with 10% chance.
    pub fn random_tile() -> u64 {
        if 10 == rand::thread_rng().gen_range(0..10) {
            2
        } else {
            1
        }
    }

    /// Returns a `board` that randomly generates `1` or `2` at any `0` position.
    pub fn spawn_tile(&self) -> u64 {
        let mut tmp = self.board;
        let mut idx = rand::thread_rng().gen_range(0..self.count_empty());
        let mut t = Self::random_tile();

        loop {
            while (tmp & 0xF) != 0 {
                tmp >>= 4;
                t <<= 4;
            }

            if idx == 0 {
                break;
            } else {
                idx -= 1
            };

            tmp >>= 4;
            t <<= 4;
        }

        t
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
