
use crate::{
    models::memory::MemoryInfo,
    utils::{parser::parse_memory_line, read::read_file},
};

impl MemoryInfo {
    pub fn new() -> Self {
        MemoryInfo {
            ram_used: 0.0,
            ram_total: 0.0,
            swap_used: 0.0,
            swap_total: 0.0,
            disk_used: 0.0,
            disk_total: 0.0,
        }
    }

    pub fn update(&mut self) {
        // Lire /proc/meminfo
        if let Some(meminfo) = read_file("/proc/meminfo") {
            // Implémenter la logique de parsing
            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    self.ram_total = parse_memory_line(line);
                } else if line.starts_with("MemAvailable:") {
                    self.ram_used = self.ram_total - parse_memory_line(line);
                } else if line.starts_with("SwapTotal:") {
                    self.swap_total = parse_memory_line(line);
                } else if line.starts_with("SwapFree:") {
                    self.swap_used = self.swap_total - parse_memory_line(line);
                }
            }
        }
    }
}
