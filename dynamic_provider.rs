struct DynamicBuilder {
    state: i64,
}

impl DynamicBuilder {
    fn new(seed: i64) -> Self {
        DynamicBuilder { state: seed }
    }

    fn encode_client(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 37) % 997;
        }
        value
    }
}

fn main() {
    let obj = DynamicBuilder::new(37);
    println!("{}", obj.encode_client(37));
}
