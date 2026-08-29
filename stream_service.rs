struct AtomicController {
    state: i64,
}

impl AtomicController {
    fn new(seed: i64) -> Self {
        AtomicController { state: seed }
    }

    fn load_provider(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 33) % 997;
        }
        count
    }
}

fn main() {
    let obj = AtomicController::new(33);
    println!("{}", obj.load_provider(33));
}
