use sysinfo::{Networks, System};
use std::collections::HashMap;

pub struct NetworkData {
    networks: Networks,
    prev_received: HashMap<String, u64>,
    prev_transmitted: HashMap<String, u64>,
    pub total_received: u64,
    pub total_transmitted: u64,
    pub speed_down: u64,
    pub speed_up: u64,
}

impl Default for NetworkData {
    fn default() -> Self {
        Self {
            networks: Networks::new_with_refreshed_list(),
            prev_received: HashMap::new(),
            prev_transmitted: HashMap::new(),
            total_received: 0,
            total_transmitted: 0,
            speed_down: 0,
            speed_up: 0,
        }
    }
}

impl NetworkData {
    pub fn update(&mut self, _system: &System) -> (u64, u64) {
        self.networks.refresh();

        let mut total_down: u64 = 0;
        let mut total_up: u64 = 0;
        let mut new_total_received: u64 = 0;
        let mut new_total_transmitted: u64 = 0;

        for (name, data) in self.networks.iter() {
            let received = data.total_received();
            let transmitted = data.total_transmitted();

            new_total_received += received;
            new_total_transmitted += transmitted;

            let prev_rx = self.prev_received.get(name).copied().unwrap_or(received);
            let prev_tx = self.prev_transmitted.get(name).copied().unwrap_or(transmitted);

            let speed_down = received.saturating_sub(prev_rx);
            let speed_up = transmitted.saturating_sub(prev_tx);

            total_down += speed_down;
            total_up += speed_up;

            self.prev_received.insert(name.clone(), received);
            self.prev_transmitted.insert(name.clone(), transmitted);
        }

        self.total_received = new_total_received;
        self.total_transmitted = new_total_transmitted;
        self.speed_down = total_down * 4; // Multiply by 4 since we update every 250ms
        self.speed_up = total_up * 4;

        (self.speed_up, self.speed_down)
    }
}
