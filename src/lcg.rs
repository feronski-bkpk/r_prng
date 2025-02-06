pub struct LCG {
    state: u64,
    a: u64,
    c: u64,
    m: u64
}

impl LCG {
    pub fn new(seed: u64) -> Self {
        LCG {
            state: seed,
            a: 1664525,
            c: 1013904223,
            m: 2u64.pow(32),
        }
    }

    pub fn next(&mut self) -> u32 {
        self.state = (self.a * self.state + self.c) % self.m;
        (self.state & 0xFFFFFFFF) as u32
    }

    pub fn next_u8(&mut self) -> u8 {
        (self.next() >> 24) as u8
    }
}