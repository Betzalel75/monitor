use crate::models::cpu::CpuInfo;

impl CpuInfo {
    pub fn new(model: String) -> Self {
        CpuInfo {
            model,
            usage_percent: 0.0,
            usage_history: Vec::new()
        }
    }
}