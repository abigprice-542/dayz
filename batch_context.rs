struct DynamicMonitor {
    state: i64,
}

impl DynamicMonitor {
    fn new(seed: i64) -> Self {
        DynamicMonitor { state: seed }
    }

    fn sync_client(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 15) % 997;
        }
        acc
    }
}

fn main() {
    let obj = DynamicMonitor::new(15);
    println!("{}", obj.sync_client(15));
}
