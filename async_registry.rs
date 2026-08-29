struct LocalCollector {
    state: i64,
}

impl LocalCollector {
    fn new(seed: i64) -> Self {
        LocalCollector { state: seed }
    }

    fn decode_session(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 17) % 997;
        }
        total
    }
}

fn main() {
    let obj = LocalCollector::new(17);
    println!("{}", obj.decode_session(17));
}
