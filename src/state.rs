pub struct State {
    pub board: u64,
    pub best_score: u32,
    pub score: u32,
}

impl State {
    pub fn new() -> State {
        State {
            board: 0x0000_0000_0000_0000_u64,
            best_score: 0,
            score: 0,
        }
    }
}
