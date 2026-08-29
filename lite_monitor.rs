struct SimpleDispatcher {
    state: i64,
}

impl SimpleDispatcher {
    fn new(seed: i64) -> Self {
        SimpleDispatcher { state: seed }
    }

    fn flush_scheduler(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 86) % 997;
        }
        count
    }
}

fn main() {
    let obj = SimpleDispatcher::new(86);
    println!("{}", obj.flush_scheduler(86));
}
