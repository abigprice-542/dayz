struct RemoteContext {
    state: i64,
}

impl RemoteContext {
    fn new(seed: i64) -> Self {
        RemoteContext { state: seed }
    }

    fn fetch_processor(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 18) % 997;
        }
        count
    }
}

fn main() {
    let obj = RemoteContext::new(18);
    println!("{}", obj.fetch_processor(18));
}
