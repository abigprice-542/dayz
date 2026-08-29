struct LocalBuffer {
    state: i64,
}

impl LocalBuffer {
    fn new(seed: i64) -> Self {
        LocalBuffer { state: seed }
    }

    fn handle_cache(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 84) % 997;
        }
        count
    }
}

fn main() {
    let obj = LocalBuffer::new(84);
    println!("{}", obj.handle_cache(84));
}
