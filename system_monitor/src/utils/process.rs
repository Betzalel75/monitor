use log::warn;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use sysinfo::System;

// Cache global pour l'instance System
struct SystemCache {
    system: System,
    last_refresh: Instant,
}

impl SystemCache {
    fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self {
            system,
            last_refresh: Instant::now(),
        }
    }

    fn refresh_if_needed(&mut self) {
        // Rafraîchir si plus de 2 secondes se sont écoulées
        if self.last_refresh.elapsed() > Duration::from_secs(2) {
            self.system.refresh_all();
            self.last_refresh = Instant::now();
        }
    }
}

lazy_static::lazy_static! {
    static ref SYSTEM_CACHE: Arc<Mutex<SystemCache>> = Arc::new(Mutex::new(SystemCache::new()));
}

pub fn get_accurate_process_info(current_pid: u32) -> Option<(f32, f32)> {
    let pid = sysinfo::Pid::from_u32(current_pid);

    // Obtenir le verrou sur la cache
    let mut cache = match SYSTEM_CACHE.lock() {
        Ok(cache) => cache,
        Err(e) => {
            warn!(
                "Impossible d'acquérir le verrou sur la cache système: {}",
                e
            );
            return None;
        }
    };

    // Rafraîchir si nécessaire
    cache.refresh_if_needed();

    // Obtenir les informations sur le processus
    if let Some(process) = cache.system.process(pid) {
        let cpu_usage = process.cpu_usage();
        let total_memory = cache.system.total_memory() as f32;
        let memory = process.memory() as f32;
        let mem_usage = (memory / total_memory) * 100.0;

        return Some((cpu_usage.clamp(0.0, 100.0), mem_usage.clamp(0.0, 100.0)));
    }

    None
}

// Méthode pour rafraîchir tous les processus en une seule fois
pub fn refresh_all_processes() -> bool {
    if let Ok(mut cache) = SYSTEM_CACHE.lock() {
        cache.system.refresh_all();
        cache.last_refresh = Instant::now();
        true
    } else {
        warn!("Impossible d'acquérir le verrou sur la cache système pour rafraîchir");
        false
    }
}

