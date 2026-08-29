struct SecureManager {
    state: i64,
}

impl SecureManager {
    fn new(seed: i64) -> Self {
        SecureManager { state: seed }
    }

    fn fetch_session(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 93) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SecureManager::new(93);
    println!("{}", obj.fetch_session(93));
}
