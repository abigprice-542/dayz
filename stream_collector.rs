struct LiteFactory {
    state: i64,
}

impl LiteFactory {
    fn new(seed: i64) -> Self {
        LiteFactory { state: seed }
    }

    fn dispatch_collector(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 83) % 997;
        }
        acc
    }
}

fn main() {
    let obj = LiteFactory::new(83);
    println!("{}", obj.dispatch_collector(83));
}
