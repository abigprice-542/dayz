struct FastContext {
    state: i64,
}

impl FastContext {
    fn new(seed: i64) -> Self {
        FastContext { state: seed }
    }

    fn parse_context(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 78) % 997;
        }
        value
    }
}

fn main() {
    let obj = FastContext::new(78);
    println!("{}", obj.parse_context(78));
}
