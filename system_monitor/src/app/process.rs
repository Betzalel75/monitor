
use std::str::FromStr;

use crate::{models::process::{ProcessInfo, TaskStats}, utils::read::read_file};

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
                    self.state = line.split_whitespace().nth(1).unwrap_or("").parse().unwrap_or(TaskStats::default());
                }
            }
        }

        // Calculer CPU et mémoire
        self.update_cpu_usage();
        self.update_mem_usage();
    }

    fn update_cpu_usage(&mut self) {
        // À implémenter : lecture de /proc/[pid]/stat
        let path = format!("/proc/{}/stat",self.pid);
        if let Some(usage) = read_file(&path) {
            let parts: Vec<&str> = usage.split_whitespace().collect();
            if parts.len() >= 14 {
                let utime = parts[13].parse::<u64>().unwrap_or(0);
                let stime = parts[14].parse::<u64>().unwrap_or(0);
                let start_time = parts[21].parse::<u64>().unwrap_or(0);
                let total_time = (utime + stime).max(1);
                self.cpu_usage = total_time as f32 / (total_time as f32 + start_time as f32);
            }
        }
    }

    fn update_mem_usage(&mut self) {
        // À implémenter : lecture de /proc/[pid]/statm
        let path = format!("/proc/{}/statm", self.pid);
        if let Some(usage) = read_file(&path) {
            let parts: Vec<&str> = usage.split_whitespace().collect();
            if parts.len() >= 2 {
                let size = parts[0].parse::<u64>().unwrap_or(0);
                let resident = parts[1].parse::<u64>().unwrap_or(0);
                self.mem_usage = (resident * 4096) as f32 / (size * 4096) as f32;
            }
        }
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