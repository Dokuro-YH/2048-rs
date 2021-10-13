pub trait Storage {
    fn board(&self) -> u64;

    fn set_board(&mut self, board: u64);

    fn best(&self) -> u64;

    fn set_best(&mut self, best: u64);
}

impl Storage for () {
    fn board(&self) -> u64 {
        0
    }

    fn set_board(&mut self, _: u64) {}

    fn best(&self) -> u64 {
        0
    }

    fn set_best(&mut self, _: u64) {}
}

pub struct InMemoryStorage {
    board: u64,
    best: u64,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        InMemoryStorage { board: 0, best: 0 }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage for InMemoryStorage {
    fn board(&self) -> u64 {
        self.board
    }

    fn set_board(&mut self, board: u64) {
        self.board = board
    }

    fn best(&self) -> u64 {
        self.best
    }

    fn set_best(&mut self, best: u64) {
        self.best = best
    }
}
