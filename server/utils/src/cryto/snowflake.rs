use std::time::{SystemTime, UNIX_EPOCH};

/*
fn main() {
    let mut snowflake = Snowflake::new(1);
    for _ in 0..10 {
        let id = snowflake.generate();
        println!("Generated ID: {}", id);
    }
}
*/

pub struct Snowflake {
    epoch: u64,
    worker_id: u16,
    sequence: u16,
    last_timestamp: u64,
}

impl Snowflake {
    pub fn new(worker_id: u16) -> Self {
        Snowflake {
            epoch: 1620396000000, // 2021-05-07T00:00:00Z in milliseconds since Unix epoch
            worker_id,
            sequence: 0,
            last_timestamp: 0,
        }
    }

    pub fn generate(&mut self) -> u64 {
        let timestamp = self.get_timestamp();
        if timestamp < self.last_timestamp {
            panic!("Clock went backwards! Last timestamp: {}, current timestamp: {}", self.last_timestamp, timestamp);
        }

        if timestamp == self.last_timestamp {
            self.sequence = self.sequence.overflowing_add(1).0;
            if self.sequence == 0 {
                // We've run out of sequence numbers for this millisecond, wait until the next one
                while self.get_timestamp() == timestamp {}
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;
        ((timestamp - self.epoch) << 22) | ((self.worker_id as u64) << 12) | (self.sequence as u64)
    }

    pub fn get_timestamp(&self) -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards!").as_millis() as u64
    }
}
