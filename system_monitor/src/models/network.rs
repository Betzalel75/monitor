use std::net::Ipv4Addr;

#[derive(Debug, Clone, Copy)]
pub struct RxStats {
    pub bytes: u64,
    pub packets: u64,
    pub errs: u64,
    pub drop: u64,
    pub fifo: u64,
    pub frame: u64,
    pub compressed: u64,
    pub multicast: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct TxStats {
    pub bytes: u64,
    pub packets: u64,
    pub errs: u64,
    pub drop: u64,
    pub fifo: u64,
    pub colls: u64,
    pub carrier: u64,
    pub compressed: u64,
}

pub struct Interface {
    pub name: String,
    pub ip: Ipv4Addr,
    pub total_received: u64,
    pub total_transmitted: u64,
    pub rx_stats: Option<RxStats>,
    pub tx_stats: Option<TxStats>,
}

#[derive(Default)]
pub struct InterfaceStats{
    pub interfaces: Vec<Interface>,
}