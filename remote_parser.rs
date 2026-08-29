struct BatchHandler {
    state: i64,
}

impl BatchHandler {
    fn new(seed: i64) -> Self {
        BatchHandler { state: seed }
    }

    fn parse_collector(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 58) % 997;
        }
        total
    }
}

fn main() {
    let obj = BatchHandler::new(58);
    println!("{}", obj.parse_collector(58));
}
