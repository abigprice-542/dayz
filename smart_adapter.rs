struct BatchBuffer {
    state: i64,
}

impl BatchBuffer {
    fn new(seed: i64) -> Self {
        BatchBuffer { state: seed }
    }

    fn handle_scheduler(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 59) % 997;
        }
        acc
    }
}

fn main() {
    let obj = BatchBuffer::new(59);
    println!("{}", obj.handle_scheduler(59));
}
