use crate::{
    models::sensors::{FanInfo, FanInfos, ThermalInfo, ThermalInfos},
    utils::{
        read::read_sensor_value,
        sensors::{detected_fans, detected_temperatures},
    },
};

impl FanInfos {
    pub fn new() -> Self {
        let mut fans: Vec<FanInfo> = Vec::new();
        for sensor in detected_fans() {
            match read_sensor_value(&sensor) {
                Ok(value) => {
                    let id = sensor.label;
                    let velocity = value as f32;
                    fans.push(FanInfo::new(id, velocity));
                }
                Err(_) => {}
            }
        }
        Self { fans }
    }
    pub fn update(&mut self) {
        for sensor in detected_fans() {
            match read_sensor_value(&sensor) {
                Ok(value) => {
                    // update if existe if else create
                    if let Some(fan) = self.fans.iter_mut().find(|f| f.id == sensor.label) {
                        let velocity = match sensor.max {
                            Some(max) => (value as f32 / max as f32) * 100.0,
                            None => 0.0,
                        };
                        fan.update(velocity);
                    } else {
                        let velocity = match sensor.max {
                            Some(max) => (value as f32 / max as f32) * 100.0,
                            None => 0.0,
                        };
                        self.fans.push(FanInfo::new(sensor.label, velocity));
                    }
                }
                Err(_) => {}
            }
        }
    }
}

impl ThermalInfos {
    pub fn new() -> Self {
        let mut thermals: Vec<ThermalInfo> = Vec::new();
        for sensor in detected_temperatures() {
            match read_sensor_value(&sensor) {
                Ok(value) => {
                    let id = sensor.label;
                    let state = value as f32;
                    thermals.push(ThermalInfo::new(id, state));
                }
                Err(_) => {}
            }
        }
        Self { thermals }
    }
    pub fn update(&mut self) {
        for sensor in detected_temperatures() {
            match read_sensor_value(&sensor) {
                Ok(value) => {
                    // update if existe if else create
                    if let Some(thermal) = self.thermals.iter_mut().find(|t| t.id == sensor.label) {
                        let new_state = (value as f32 / 1000.0).floor().clamp(0.0, 100.0);
                        thermal.update(new_state);
                    } else {
                        let state = (value as f32 / 1000.0).floor().clamp(0.0, 100.0);
                        self.thermals.push(ThermalInfo::new(sensor.label, state));
                    }
                }
                Err(_) => {}
            }
        }
    }
}

impl FanInfo {
    pub fn new(id: String, velocity: f32) -> Self {
        Self {
            id,
            velocity,
            history: [0.0; 9], // Initialise avec 9 zéros
        }
    }
    fn update(&mut self, velocity: f32) {
        self.velocity = velocity;
        self.history.copy_within(1.., 0); // Décalage des éléments 1.. vers 0..
        self.history[8] = velocity;
    }
}

impl ThermalInfo {
    pub fn new(id: String, state: f32) -> Self {
        Self {
            id,
            state,
            history: [0.0; 9], // Initialise avec 9 zéros
        }
    }
    fn update(&mut self, new_state: f32) {
        self.state = new_state;
        self.history.copy_within(1.., 0); // Décalage des éléments 1.. vers 0..
        self.history[8] = new_state;
    }
}
