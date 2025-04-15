use std::path::PathBuf;

pub struct Sensor {
    pub path: PathBuf,
    pub label: String,
    pub max: Option<u32>, // RPM max optionnel
}

#[derive(Debug, Default, Clone)]
pub struct ThermalInfos {
    pub thermals: Vec<ThermalInfo>,
}

#[derive(Debug, Default, Clone)]
pub struct FanInfos {
    pub fans: Vec<FanInfo>,
}

#[derive(Debug, Default, Clone)]
pub struct FanInfo {
    pub id: String,
    pub velocity: f32,
    pub history: [f32; 9],
}

#[derive(Debug, Default, Clone)]
pub struct ThermalInfo {
    pub id: String,
    pub state: f32,
    pub history: [f32; 9],
}
