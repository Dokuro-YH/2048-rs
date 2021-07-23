use lazy_static::lazy_static;

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
}

lazy_static! {
    pub static ref MOVES: Moves = {
        let mut up = vec![0; 65536];
        let mut down = vec![0; 65536];
        let mut left = vec![0; 65536];
        let mut right = vec![0; 65536];
        let mut scores = vec![0; 65536];

        for row in 0..65536 {
            let mut line = [
                (row >> 0) & 0xF,
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

            let result = (line[0] <<  0) |
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
