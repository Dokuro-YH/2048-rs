use rand::Rng;

use crate::moves::{Direction, Moves};

pub struct Game {
    board: u64,
    score: u64,
}

impl Game {
    fn init(&mut self) {
        if self.count_empty() == 16 {
            self.spawn_tile();
            self.spawn_tile();
        }
        self.score = Moves::get_score(&self.board);
    }

    pub fn new() -> Game {
        let mut game = Game {
            board: 0x0000_0000_0000_0000_u64,
            score: 0
        };

        game.init();

        game
    }

    pub fn with_board(board: u64) -> Game {
        let mut game = Game {
            board,
            score: 0
        };

        game.init();

        game
    }

    pub fn execute(&mut self, direction: Direction) {
        let board = self.board;
        let result_board = match direction {
            Direction::Up => Moves::up(board),
            Direction::Down => Moves::down(board),
            Direction::Left => Moves::left(board),
            Direction::Right => Moves::right(board),
        };

        if board != result_board {
            self.board = result_board;
            self.score = Moves::get_score(&self.board);
            self.spawn_tile();
        }
    }

    pub fn restart(&mut self) {
        self.board = 0x0000_0000_0000_0000_u64;

        self.init();
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn game_over(&self) -> bool {
        if self.count_empty() > 0 {
            return false;
        }

        if Moves::up(self.board) != self.board {
            return false;
        }

        if Moves::down(self.board) != self.board {
            return false;
        }

        if Moves::left(self.board) != self.board {
            return false;
        }

        if Moves::left(self.board) != self.board {
            return false;
        }

        true
    }

    pub fn board(&self) -> u64 {
        self.board
    }

    pub fn grid(&self) -> [[u8; 4]; 4] {
        let mut grid: [[u8; 4]; 4] = [[0; 4]; 4];

        for n in 0..4 {
            let idx = 3 - n;
            let row = (self.board >> (n * 16)) & crate::ROW_MASK;

            grid[idx] = [
                ((row >> 12) & 0xF) as u8,
                ((row >> 8) & 0xF) as u8,
                ((row >> 4) & 0xF) as u8,
                (row & 0xF) as u8,
            ];
        }

        grid
    }

    /// Returns the count of tiles with a value of `0`
    fn count_empty(&self) -> u32 {
        let mut empty = 0;

        for i in 0..16 {
            if self.board >> (i * 4) & 0xF == 0 {
                empty += 1
            }
        }

        empty
    }

    /// Returns a `board` that randomly generates `1` or `2` at any `0` position.
    fn spawn_tile(&mut self) {
        let mut tmp = self.board;
        let mut idx = rand::thread_rng().gen_range(0..self.count_empty());
        let mut t = self::random_tile();

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

        self.board |= t
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

/// Returns `1` with 60% chance and `2` with 40% chance.
fn random_tile() -> u64 {
    if rand::thread_rng().gen_range(0..10) < 4 {
        2
    } else {
        1
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_count_empty() {
        let game = Game::with_board(0x0100_0020_0001_0001);
        let result = game.count_empty();

        assert_eq!(12, result);
    }

    #[test]
    fn test_grid() {
        let game = Game::with_board(0xFEDC_BA98_7654_3210);
        assert_eq!(
            game.grid(),
            [[15, 14, 13, 12], [11, 10, 9, 8], [7, 6, 5, 4], [3, 2, 1, 0]]
        );

        let game = Game::with_board(0x0123_4567_89AB_CDEF);
        assert_eq!(
            game.grid(),
            [[0, 1, 2, 3], [4, 5, 6, 7], [8, 9, 10, 11], [12, 13, 14, 15]]
        );
    }
}
