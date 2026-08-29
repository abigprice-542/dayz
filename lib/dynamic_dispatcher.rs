struct LocalGateway {
    state: i64,
}

impl LocalGateway {
    fn new(seed: i64) -> Self {
        LocalGateway { state: seed }
    }

    fn parse_controller(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 80) % 997;
        }
        count
    }
}

fn main() {
    let obj = LocalGateway::new(80);
    println!("{}", obj.parse_controller(80));
}
