use crate::{models::system::SystemInfo, utils::{self, parser::parse_cpu_model}};
use std::env;

impl SystemInfo {
    pub fn new() -> Self {
        let os_type = env::consts::OS.to_string();
        let user = env::var("USER").unwrap_or("unknown".to_string());
        let hostname = utils::read::read_file("/etc/hostname").unwrap_or("unknown".to_string());
        let cpu_model = match utils::read::read_file("/proc/cpuinfo"){
            Some(content) => parse_cpu_model(&content),
            None => "unknown".to_string(),
        };
        Self {
            os_type,
            user,
            hostname,
            total_tasks: 0,
            cpu_model,
        }
    }
    pub fn set_tastks(&mut self,tasks: u32){
        self.total_tasks = tasks;
    }
    pub fn get_total_tasks(&self) -> u32{
        self.total_tasks
    }
}
