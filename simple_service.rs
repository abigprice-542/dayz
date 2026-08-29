struct SecureGateway {
    state: i64,
}

impl SecureGateway {
    fn new(seed: i64) -> Self {
        SecureGateway { state: seed }
    }

    fn dispatch_processor(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 18) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SecureGateway::new(18);
    println!("{}", obj.dispatch_processor(18));
}
