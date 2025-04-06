use crate::{
    models::network::{InterfaceStats, NetData},
    utils::read::read_file,
};

impl InterfaceStats {
    pub fn new(name: String) -> Self {
        InterfaceStats {
            name,
            ipv4: None,
            rx: NetData::default(),
            tx: NetData::default(),
        }
    }

    pub fn update(&mut self) {
        if let Some(netdev) = read_file("/proc/net/dev") {
            for line in netdev.lines() {
                if line.contains(&self.name) {
                    let stats: Vec<&str> = line.split_whitespace().collect();
                    if stats.len() >= 17 {
                        self.update_rx(&stats[1..9]);
                        self.update_tx(&stats[9..17]);
                    }
                    break;
                }
            }
        }
    }

    fn update_rx(&mut self, stats: &[&str]) {
        self.rx = NetData::from_slice(stats);
    }

    fn update_tx(&mut self, stats: &[&str]) {
        self.tx = NetData::from_slice(stats);
    }
}

impl Default for NetData {
    fn default() -> Self {
        NetData {
            bytes: 0,
            packets: 0,
            errs: 0,
            drop: 0,
            fifo: 0,
            frame_or_colls: 0,
            compressed: 0,
            multicast_or_carrier: 0,
        }
    }
}

impl NetData {
    fn from_slice(stats: &[&str]) -> Self {
        NetData {
            bytes: stats[0].parse().unwrap_or(0),
            packets: stats[1].parse().unwrap_or(0),
            errs: stats[2].parse().unwrap_or(0),
            drop: stats[3].parse().unwrap_or(0),
            fifo: stats[4].parse().unwrap_or(0),
            frame_or_colls: stats[5].parse().unwrap_or(0),
            compressed: stats[6].parse().unwrap_or(0),
            multicast_or_carrier: stats[7].parse().unwrap_or(0),
        }
    }
}
