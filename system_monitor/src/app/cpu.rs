use std::thread;

use crate::{
    models::cpu::{CoreInfo, CoreStats, CpuInfo},
    utils::read::read_file,
};

impl CpuInfo {
    pub fn new(model: String) -> Self {
        let content = read_file("/proc/stat").unwrap_or_default();
        let cores = Self::parse_proc_stat(&content);

        CpuInfo {
            model,
            number_of_cores: cores.len() as u32,
            cores,
            stats: Vec::new(),
        }
    }

    pub fn update_stats(&mut self) {
        let content = read_file("/proc/stat").unwrap_or_default();
        let new_cores = Self::parse_proc_stat(&content);

        let previous_cores = self.cores.clone();
        let mut handles = vec![];

        for current in new_cores.clone() {
            if let Some(prev) = previous_cores.iter().find(|p| p.id == current.id) {
                let prev_clone = prev.clone();
                let current_clone = current.clone();

                let handle = thread::spawn(move || {
                    let usage = prev_clone.calculate_cpu_usage(&current_clone) as f32;
                    usage
                });
                handles.push(handle);
            }
        }

        // Attente et collecte des résultats
        // Mise à jour des stats avec historique
        for (i, handle) in handles.into_iter().enumerate() {
            if let Ok(usage) = handle.join() {
                if let Some(core_stat) = self.stats.get_mut(i) {
                    core_stat.update(usage);
                } else {
                    self.stats
                        .push(CoreStats::new(format!("CPU{}", i + 1), usage));
                }
            }
        }
        
        self.cores = new_cores;
    }

    fn parse_proc_stat(content: &str) -> Vec<CoreInfo> {
        let mut cores = Vec::new();

        for line in content.lines() {
            if line.starts_with("cpu") && line.chars().nth(3).map_or(false, |c| c.is_ascii_digit())
            {
                let mut parts = line.split_whitespace();
                let id = parts.next().unwrap_or("").to_string();
                let numbers: Vec<u64> = parts.filter_map(|s| s.parse().ok()).collect();

                if numbers.len() >= 8 {
                    cores.push(CoreInfo::new(
                        id, numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5],
                        numbers[6], numbers[7],
                    ));
                }
            }
        }

        cores
    }
}

impl CoreInfo {
    pub fn new(
        id: String,
        user: u64,
        nice: u64,
        system: u64,
        idle: u64,
        iowait: u64,
        irq: u64,
        softirq: u64,
        steal: u64,
    ) -> Self {
        CoreInfo {
            id,
            user,
            nice,
            system,
            idle,
            iowait,
            irq,
            softirq,
            steal,
        }
    }

    pub fn calculate_cpu_usage(&self, current: &CoreInfo) -> f64 {
        let prev_idle = self.idle + self.iowait;
        let curr_idle = current.idle + current.iowait;

        let prev_total =
            self.user + self.nice + self.system + prev_idle + self.irq + self.softirq + self.steal;
        let curr_total = current.user
            + current.nice
            + current.system
            + curr_idle
            + current.irq
            + current.softirq
            + current.steal;

        let delta_total = curr_total.saturating_sub(prev_total);
        let delta_idle = curr_idle.saturating_sub(prev_idle);

        if delta_total == 0 {
            return 0.0;
        }

        ((delta_total - delta_idle) as f64 / delta_total as f64 * 100.0).floor().clamp(0.0, 100.0)
    }
}

impl CoreStats {
    fn new(id: String, usage: f32) -> Self {
        CoreStats {
            id,
            usage,
            history: [0.0; 9], // Initialise avec 9 zéros
        }
    }

    fn update(&mut self, new_usage: f32) {
        self.usage = new_usage;
        self.history.copy_within(1.., 0); // Décalage des éléments 1.. vers 0..
        self.history[8] = new_usage;
    }
}
