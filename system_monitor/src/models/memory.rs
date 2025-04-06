
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub ram_used: f32,
    pub ram_total: f32,
    pub swap_used: f32,
    pub swap_total: f32,
    pub disk_used: f32,
    pub disk_total: f32,
}

impl Default for MemoryInfo {
    fn default() -> Self {
        MemoryInfo {
            ram_used: 0.0,
            ram_total: 0.0,
            swap_used: 0.0,
            swap_total: 0.0,
            disk_used: 0.0,
            disk_total: 0.0,
        }
    }
}
