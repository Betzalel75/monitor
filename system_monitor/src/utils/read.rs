use std::{fs, path::Path};

use sysinfo::Disks;

use crate::models::sensors::Sensor;

pub fn read_file(path: &str) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

/// Retourne l'espace utilisé (total - disponible) pour `/`
/// Et pour `/home` seulement s’il s’agit d’un point de montage distinct.
/// Format : (utilisé_sur_slash, Option<utilisé_sur_home>)
pub fn get_mount_usage() -> (u64, u64) {
    let system = Disks::new_with_refreshed_list();

    let mut used_root = 0;
    let mut total_disk = 0;

    for disk in system.iter() {
        let mount_point = disk.mount_point();

        if mount_point == Path::new("/") {
            used_root += disk.total_space() - disk.available_space();
            total_disk += disk.total_space();
        } else if mount_point == Path::new("/home") {
            used_root += disk.total_space() - disk.available_space();
            total_disk += disk.total_space();
        }
    }

    (total_disk, used_root)
}

// Lit la valeur d'un capteur depuis le système de fichiers
pub fn read_sensor_value(sensor: &Sensor) -> Result<u32, String> {
    fs::read_to_string(&sensor.path)
        .map_err(|e| format!("Erreur E/S: {}", e))?
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("Erreur conversion: {}", e))
}
