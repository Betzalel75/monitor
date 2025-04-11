use std::net::Ipv4Addr;
// use sysinfo::Networks;

use crate::{
    models::network::{Interface, InterfaceStats, RxStats, TxStats},
    utils::read::read_file,
};

impl Interface {
    pub fn new(
        name: String,
        ip: Ipv4Addr,
        total_received: u64,
        total_transmitted: u64,
        rx_stats: Option<RxStats>,
        tx_stats: Option<TxStats>,
    ) -> Interface {
        Interface {
            name,
            ip,
            total_received,
            total_transmitted,
            rx_stats,
            tx_stats,
        }
    }
}

impl InterfaceStats {
    pub fn new() -> Self {
        Self {
            interfaces: Vec::new(),
        }
    }
    
    pub fn initialize(&mut self) {
        if let Some(netdev) = read_file("/proc/net/dev") {
            // Skip the first two lines (headers)
            for line in netdev.lines().skip(2) {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Split the line into interface name and stats
                let mut parts = line.splitn(2, ':');
                if let Some(interface_name) = parts.next() {
                    let interface_name = interface_name.trim().to_string();

                    // Get the stats part
                    if let Some(stats) = parts.next() {
                        let stats: Vec<&str> = stats.split_whitespace().collect();
                        if stats.len() >= 16 {
                            // RX stats (first 8 values)
                            let rx_stats = RxStats::from_slice(&stats[0..8]);

                            // TX stats (next 8 values)
                            let tx_stats = TxStats::from_slice(&stats[8..16]);

                            let ip = Ipv4Addr::new(0, 0, 0, 0);

                            // Create new interface with default IP (you might want to get the real IP)
                            let new_interface = Interface::new(
                                interface_name,
                                ip,
                                rx_stats.bytes as u64,
                                tx_stats.bytes as u64,
                                Some(rx_stats),
                                Some(tx_stats),
                            );

                            self.interfaces.push(new_interface);
                        }
                    }
                }
            }
            // Mettre à jour les adresses IP des interfaces réseau
            let pnet_interfaces = pnet::datalink::interfaces();
            for pnet_interface in &pnet_interfaces {
                for ip_network in &pnet_interface.ips {
                    if let pnet::ipnetwork::IpNetwork::V4(ipv4_network) = ip_network {
                        for interface in &mut self.interfaces {
                            if interface.name == pnet_interface.name {
                                interface.ip = ipv4_network.ip();
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        if let Some(netdev) = read_file("/proc/net/dev") {
            // Skip the first two lines (headers)
            for line in netdev.lines().skip(2) {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // Split the line into interface name and stats
                let mut parts = line.splitn(2, ':');
                if let Some(interface_name) = parts.next() {
                    let interface_name = interface_name.trim().to_string();

                    // Get the stats part
                    if let Some(stats) = parts.next() {
                        let stats: Vec<&str> = stats.split_whitespace().collect();
                        if stats.len() >= 16 {
                            // RX stats (first 8 values)
                            let rx_stats = RxStats::from_slice(&stats[0..8]);

                            // TX stats (next 8 values)
                            let tx_stats = TxStats::from_slice(&stats[8..16]);

                            // Find or create interface
                            let interface = self
                                .interfaces
                                .iter_mut()
                                .find(|i| i.name == interface_name);

                            if let Some(interface) = interface {
                                interface.rx_stats = Some(rx_stats);
                                interface.tx_stats = Some(tx_stats);
                                interface.total_received = rx_stats.bytes as u64;
                                interface.total_transmitted = tx_stats.bytes as u64;
                            } else {
                                // Create new interface with default IP (you might want to get the real IP)
                                self.initialize();
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Default for RxStats {
    fn default() -> Self {
        RxStats {
            bytes: 0.0,
            packets: 0,
            errs: 0,
            drop: 0,
            fifo: 0,
            frame: 0,
            compressed: 0,
            multicast: 0,
        }
    }
}

impl RxStats {
    fn from_slice(stats: &[&str]) -> Self {
        RxStats {
            bytes: stats[0].parse().unwrap_or(0.0),
            packets: stats[1].parse().unwrap_or(0),
            errs: stats[2].parse().unwrap_or(0),
            drop: stats[3].parse().unwrap_or(0),
            fifo: stats[4].parse().unwrap_or(0),
            frame: stats[5].parse().unwrap_or(0),
            compressed: stats[6].parse().unwrap_or(0),
            multicast: stats[7].parse().unwrap_or(0),
        }
    }
}

impl Default for TxStats {
    fn default() -> Self {
        TxStats {
            bytes: 0.0,
            packets: 0,
            errs: 0,
            drop: 0,
            fifo: 0,
            colls: 0,
            carrier: 0,
            compressed: 0,
        }
    }
}

impl TxStats {
    fn from_slice(stats: &[&str]) -> Self {
        TxStats {
            bytes: stats[0].parse().unwrap_or(0.0),
            packets: stats[1].parse().unwrap_or(0),
            errs: stats[2].parse().unwrap_or(0),
            drop: stats[3].parse().unwrap_or(0),
            fifo: stats[4].parse().unwrap_or(0),
            colls: stats[5].parse().unwrap_or(0),
            carrier: stats[6].parse().unwrap_or(0),
            compressed: stats[7].parse().unwrap_or(0),
        }
    }
}
