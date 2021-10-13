use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Struct that contains all available moves per row for up, down, left and right.
/// Also scores the score for a given row.
pub struct Moves {
    pub up: Vec<u64>,
    pub down: Vec<u64>,
    pub left: Vec<u64>,
    pub right: Vec<u64>,
    pub scores: Vec<u64>,
}

impl Moves {
    fn column_from(board: u64) -> u64 {
        (board | (board << 12) | (board << 24) | (board << 36)) & crate::COL_MASK
    }

    /// Returns a transposed board where row are transformed into columns and vice versa.
    fn transpose(board: u64) -> u64 {
        let a1 = board & 0xF0F0_0F0F_F0F0_0F0F_u64;
        let a2 = board & 0x0000_F0F0_0000_F0F0_u64;
        let a3 = board & 0x0F0F_0000_0F0F_0000_u64;

        let a = a1 | (a2 << 12) | (a3 >> 12);

        let b1 = a & 0xFF00_FF00_00FF_00FF_u64;
        let b2 = a & 0x00FF_00FF_0000_0000_u64;
        let b3 = a & 0x0000_0000_FF00_FF00_u64;

        b1 | (b2 >> 24) | (b3 << 24)
    }

    pub fn up(board: u64) -> u64 {
        let transposed = Self::transpose(board);
        let mut result = board;

        result ^= MOVES.up[(transposed & crate::ROW_MASK) as usize];
        result ^= MOVES.up[((transposed >> 16) & crate::ROW_MASK) as usize] << 4;
        result ^= MOVES.up[((transposed >> 32) & crate::ROW_MASK) as usize] << 8;
        result ^= MOVES.up[((transposed >> 48) & crate::ROW_MASK) as usize] << 12;

        result
    }

    pub fn down(board: u64) -> u64 {
        let transposed = Self::transpose(board);
        let mut result = board;

        result ^= MOVES.down[(transposed & crate::ROW_MASK) as usize];
        result ^= MOVES.down[((transposed >> 16) & crate::ROW_MASK) as usize] << 4;
        result ^= MOVES.down[((transposed >> 32) & crate::ROW_MASK) as usize] << 8;
        result ^= MOVES.down[((transposed >> 48) & crate::ROW_MASK) as usize] << 12;

        result
    }

    pub fn left(board: u64) -> u64 {
        let mut result = board;

        result ^= MOVES.left[(board & crate::ROW_MASK) as usize];
        result ^= MOVES.left[((board >> 16) & crate::ROW_MASK) as usize] << 16;
        result ^= MOVES.left[((board >> 32) & crate::ROW_MASK) as usize] << 32;
        result ^= MOVES.left[((board >> 48) & crate::ROW_MASK) as usize] << 48;

        result
    }

    pub fn right(board: u64) -> u64 {
        let mut result = board;

        result ^= MOVES.right[(board & crate::ROW_MASK) as usize];
        result ^= MOVES.right[((board >> 16) & crate::ROW_MASK) as usize] << 16;
        result ^= MOVES.right[((board >> 32) & crate::ROW_MASK) as usize] << 32;
        result ^= MOVES.right[((board >> 48) & crate::ROW_MASK) as usize] << 48;

        result
    }

    pub fn get_score(board: &u64) -> u64 {
        let table = &MOVES.scores;

        table[(board & crate::ROW_MASK) as usize]
            + table[(board >> 16 & crate::ROW_MASK) as usize]
            + table[(board >> 32 & crate::ROW_MASK) as usize]
            + table[(board >> 48 & crate::ROW_MASK) as usize]
    }
}

lazy_static! {
    static ref MOVES: Moves = {
        let mut up = vec![0; 65536];
        let mut down = vec![0; 65536];
        let mut left = vec![0; 65536];
        let mut right = vec![0; 65536];
        let mut scores = vec![0; 65536];

        for row in 0..65536 {
            let mut line = [
                row        & 0xF,
                (row >> 4) & 0xF,
                (row >> 8) & 0xF,
                (row >> 12) & 0xF
            ];

            // calculate score of given row
            let mut s = 0;

            for l in line {
                if l > 1 { s += (l - 1) * (2 << l) }
            };

            scores[row as usize] = s;

            let mut i = 0;

            while i < 3 {
                let mut j = i + 1;

                while j < 4 {
                    if line[j] != 0 { break };
                    j += 1;
                };

                if j == 4 { break };

                if line[i] == 0 {
                    line[i] = line[j];
                    line[j] = 0;
                    continue;
                } else if line[i] == line[j] {
                    if line[i] != 0xF { line[i] += 1 };
                    line[j] = 0;
                }

                i += 1;
            };

            let result = line[0]         |
                         (line[1] <<  4) |
                         (line[2] <<  8) |
                         (line[3] << 12);


            let rev_row = (row    >> 12) & 0x000F | (row    >> 4) & 0x00F0 | (row    << 4) & 0x0F00 | (row    << 12) & 0xF000;
            let rev_res = (result >> 12) & 0x000F | (result >> 4) & 0x00F0 | (result << 4) & 0x0F00 | (result << 12) & 0xF000;

            let row_idx = row     as usize;
            let rev_idx = rev_row as usize;

            right[row_idx] = row                         ^ result;
            left[rev_idx]  = rev_row                     ^ rev_res;
            up[rev_idx]    = Moves::column_from(rev_row) ^ Moves::column_from(rev_res);
            down[row_idx]  = Moves::column_from(row)     ^ Moves::column_from(result);
        };

        Moves {
            up,
            down,
            left,
            right,
            scores,
        }
    };
}

#[cfg(test)]
mod test_super {
    use super::*;

    /// | F | E | D | C |      | F | B | 7 | 3 |
    /// | B | A | 9 | 8 |  =>  | E | A | 6 | 2 |
    /// | 7 | 6 | 5 | 4 |      | D | 9 | 5 | 1 |
    /// | 3 | 2 | 1 | 0 |      | C | 8 | 4 | 0 |
    #[test]
    fn test_transpose() {
        let board = 0xFEDC_BA98_7654_3210;
        let result = Moves::transpose(board);

        assert_eq!(result, 0xFB73_EA62_D951_C840);
    }

    #[test]
    fn test_move_up() {
        let board = 0x1111_0000_0000_1111;
        let result = Moves::up(board);

        assert_eq!(result, 0x2222_0000_0000_0000);
    }

    #[test]
    fn test_move_down() {
        let board = 0x1111_0000_0000_1111;
        let result = Moves::down(board);

        assert_eq!(result, 0x0000_0000_0000_2222);
    }

    #[test]
    fn test_move_left() {
        let board = 0x1001_1001_1001_1001;
        let result = Moves::left(board);

        assert_eq!(result, 0x2000_2000_2000_2000);
    }

    #[test]
    fn test_move_right() {
        let board = 0x1001_1001_1001_1001;
        let result = Moves::right(board);

        assert_eq!(result, 0x0002_0002_0002_0002);
    }
}
