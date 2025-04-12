#[derive(Default, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub number_of_cores: u32,
    pub cores: Vec<CoreInfo>,
    pub stats: Vec<CoreStats>,
}

#[derive(Default, Clone)]
pub struct CoreInfo {
    pub id: String,
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
}

#[derive(Default, Clone, Debug)]
pub struct CoreStats {
    pub id: String,
    pub usage: f32,
    pub history: [f32; 9], // Historique des 9 dernières valeurs
}
