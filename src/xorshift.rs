pub struct Xorshift64 {
    state: u64
}

impl Xorshift64 {
    pub fn new(seed: u64) -> Self {
        Xorshift64 { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}