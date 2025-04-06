

pub struct InterfaceStats {
    pub name: String,
    pub ipv4: Option<String>,
    pub rx: NetData,
    pub tx: NetData,
}

pub struct NetData {
    pub bytes: u64,
    pub packets: u64,
    pub errs: u64,
    pub drop: u64,
    pub fifo: u64,
    pub frame_or_colls: u64, // selon RX ou TX
    pub compressed: u64,
    pub multicast_or_carrier: u64, // selon RX ou TX
}