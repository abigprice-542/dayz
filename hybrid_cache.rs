struct FastGateway {
    state: i64,
}

impl FastGateway {
    fn new(seed: i64) -> Self {
        FastGateway { state: seed }
    }

    fn sync_session(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 62) % 997;
        }
        acc
    }
}

fn main() {
    let obj = FastGateway::new(62);
    println!("{}", obj.sync_session(62));
}
