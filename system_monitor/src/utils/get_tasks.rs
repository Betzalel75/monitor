use std::{fs, path::Path};

use crate::models::process::ProcessInfo;
use super::process::refresh_all_processes;

pub fn get_tasks() -> Vec<ProcessInfo> {
    // // Pour obtenir la liste de tous les processus :
    if refresh_all_processes() {}
    let proc_dir = Path::new("/proc");
    let mut tasks = Vec::new();

    if let Ok(entries) = fs::read_dir(proc_dir) {
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if let Ok(pid) = name.parse::<u32>() {
                    // Le nom est un PID valide
                    let mut process = ProcessInfo::new(pid);
                    process.update();
                    tasks.push(process);
                }
            }
        }
    }
    tasks
}
