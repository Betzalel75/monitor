use std::str::FromStr;
#[allow(warnings)]
use log::{info, warn};

use crate::{
    models::process::{ProcessInfo, ProcessList, TaskStats},
    utils::{process::get_accurate_process_infos, get_tasks::get_tasks, read::read_file},
};

impl ProcessInfo {
    pub fn new(pid: u32) -> Self {
        ProcessInfo {
            pid,
            name: String::new(),
            state: TaskStats::default(),
            cpu_usage: 0.0,
            mem_usage: 0.0,
        }
    }

    pub fn update(&mut self) {
        let status_path = format!("/proc/{}/status", self.pid);
        if let Some(status) = read_file(&status_path) {
            for line in status.lines() {
                if line.starts_with("Name:") {
                    self.name = line.split_whitespace().nth(1).unwrap_or("").to_string();
                } else if line.starts_with("State:") {
                    self.state = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("")
                        .parse()
                        .unwrap_or(TaskStats::default());
                }
            }
        }
       
        match get_accurate_process_infos(self.pid) {
            Some((cpu_usage, mem_usage)) => {
                self.cpu_usage = cpu_usage as f32;
                self.mem_usage = mem_usage as f32;
                // info!("CPU {}% RAM {}%", self.cpu_usage, self.mem_usage);
            }
            _ => {
                warn!("Erreur lors de la mise a jour")
            }
        }
    }
}

impl ProcessList {
    pub fn new() -> Self {
        let tasks = get_tasks();
        let total_tasks = tasks.len() as u32;
        Self { tasks, total_tasks }
    }
    pub fn update(&mut self) {
        let new_tasks = get_tasks();
        self.total_tasks = new_tasks.len() as u32;
        self.tasks = new_tasks;
    }
}

impl ToString for TaskStats {
    fn to_string(&self) -> String {
        match self {
            TaskStats::Running => "running".to_string(),
            TaskStats::Sleeping => "sleeping".to_string(),
            TaskStats::DiskSleep => "disk sleep".to_string(),
            TaskStats::Stopped => "stopped".to_string(),
            TaskStats::TracingStop => "tracing stop".to_string(),
            TaskStats::Zombie => "zombie".to_string(),
            TaskStats::Dead => "dead".to_string(),
            TaskStats::WakeKill => "wakekill".to_string(),
            TaskStats::Waking => "waking".to_string(),
            TaskStats::Parked => "parked".to_string(),
            TaskStats::Idle => "idle".to_string(),
            TaskStats::Locked => "locked".to_string(),
            TaskStats::Uninterruptible => "uninterruptible".to_string(),
        }
    }
}

impl FromStr for TaskStats {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(TaskStats::Running),
            "S" => Ok(TaskStats::Sleeping),
            "D" => Ok(TaskStats::DiskSleep),
            "T" => Ok(TaskStats::Stopped),
            "t" => Ok(TaskStats::TracingStop),
            "Z" => Ok(TaskStats::Zombie),
            "X" => Ok(TaskStats::Dead),
            "K" => Ok(TaskStats::WakeKill),
            "W" => Ok(TaskStats::Waking),
            "P" => Ok(TaskStats::Parked),
            "I" => Ok(TaskStats::Idle),
            "L" => Ok(TaskStats::Locked),
            "U" => Ok(TaskStats::Uninterruptible),
            _ => Err(()),
        }
    }
}


// Méthode pour obtenir tous les PIDs actifs
pub fn get_all_pids() -> Vec<u32> {
    if let Ok(cache) = SYSTEM_CACHE.lock() {
        cache
            .system
            .processes()
            .iter()
            .map(|(pid, _)| pid.as_u32())
            .collect()
    } else {
        warn!("Impossible d'acquérir le verrou sur la cache système pour lister les PIDs");
        Vec::new()
    }
}