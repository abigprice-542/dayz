struct RemoteRegistry {
    state: i64,
}

impl RemoteRegistry {
    fn new(seed: i64) -> Self {
        RemoteRegistry { state: seed }
    }

    fn dispatch_factory(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 36) % 997;
        }
        count
    }
}

fn main() {
    let obj = RemoteRegistry::new(36);
    println!("{}", obj.dispatch_factory(36));
}
