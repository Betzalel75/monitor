#[derive(Clone)]
pub struct SystemInfo {
    pub os_type: String,
    pub user: String,
    pub hostname: String,
    pub total_tasks: u32,
    pub cpu_model: String,
}

impl Default for SystemInfo {
    fn default() -> Self {
        SystemInfo {
            os_type: String::new(),
            user: String::new(),
            hostname: String::new(),
            total_tasks: 0,
            cpu_model: String::new(),
        }
    }
}


pub struct FanInfo {
    pub enabled: bool,
    pub speed_rpm: u32,
    pub level: u8,
    pub speed_history: Vec<u32>,
}

pub struct ThermalInfo {
    pub temperature_celsius: f32,
    pub temp_history: Vec<f32>,
}
