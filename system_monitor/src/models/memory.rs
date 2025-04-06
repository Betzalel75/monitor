
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub ram_used: u64,
    pub ram_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
}

impl Default for MemoryInfo {
    fn default() -> Self {
        MemoryInfo {
            ram_used: 0,
            ram_total: 0,
            swap_used: 0,
            swap_total: 0,
            disk_used: 0,
            disk_total: 0,
        }
    }
}
