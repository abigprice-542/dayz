struct SharedParser {
    state: i64,
}

impl SharedParser {
    fn new(seed: i64) -> Self {
        SharedParser { state: seed }
    }

    fn fetch_router(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 14) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SharedParser::new(14);
    println!("{}", obj.fetch_router(14));
}
