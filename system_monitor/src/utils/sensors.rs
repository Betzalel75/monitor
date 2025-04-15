use std::{fs, path::Path};

use crate::models::sensors::Sensor;

// Lit la valeur d'un capteur depuis le système de fichiers
pub fn read_sensor_value(sensor: &Sensor) -> Result<u32, String> {
    fs::read_to_string(&sensor.path)
        .map_err(|e| format!("Erreur E/S: {}", e))?
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("Erreur conversion: {}", e))
}

// Détecte les capteurs de température CPU
pub fn detected_temperatures() -> Vec<Sensor> {
    let mut capteurs = Vec::new();
    let repertoire = Path::new("/sys/class/hwmon");

    if let Ok(entrees) = fs::read_dir(repertoire) {
        for entree in entrees.flatten() {
            let chemin = entree.path();
            if chemin.is_dir() {
                if let Ok(nom) = fs::read_to_string(chemin.join("name")) {
                    let nom = nom.trim();
                    if nom == "coretemp" || nom.starts_with("k10temp") {
                        let mut n = 1;
                        loop {
                            let fichier_temp = chemin.join(format!("temp{}_input", n));
                            if !fichier_temp.exists() {
                                break;
                            }

                            let etiquette = chemin
                                .join(format!("temp{}_label", n))
                                .exists()
                                .then(|| {
                                    fs::read_to_string(chemin.join(format!("temp{}_label", n)))
                                })
                                .transpose()
                                .ok()
                                .flatten()
                                .unwrap_or_default();

                            let etiquette = match etiquette.trim() {
                                "" => format!("Cœur {}", n),
                                e => e.to_string(),
                            };

                            // || etiquette.contains("Package")
                            if etiquette.contains("Core") || etiquette.contains("CPU") {
                                capteurs.push(Sensor {
                                    path: fichier_temp,
                                    label: etiquette,
                                    max: None,
                                });
                            }
                            n += 1;
                        }
                    }
                }
            }
        }
    }
    capteurs
}

// Détecte les capteurs de ventilateurs
pub fn detected_fans() -> Vec<Sensor> {
    let mut capteurs = Vec::new();
    let repertoire = Path::new("/sys/class/hwmon");

    if let Ok(entrees) = fs::read_dir(repertoire) {
        for entree in entrees.flatten() {
            let chemin = entree.path();
            if chemin.is_dir() {
                let mut n = 1;
                loop {
                    let fichier_fan = chemin.join(format!("fan{}_input", n));
                    if !fichier_fan.exists() {
                        break;
                    }

                    // Lecture de la vitesse maximale si disponible
                    let fichier_max = chemin.join(format!("fan{}_max", n));
                    let max_rpm = fs::read_to_string(&fichier_max)
                        .ok()
                        .and_then(|s| s.trim().parse::<u32>().ok());

                    let etiquette = chemin
                        .join(format!("fan{}_label", n))
                        .exists()
                        .then(|| fs::read_to_string(chemin.join(format!("fan{}_label", n))))
                        .transpose()
                        .ok()
                        .flatten()
                        .unwrap_or_else(|| format!("Ventilateur {}", n).into());

                    capteurs.push(Sensor {
                        path: fichier_fan,
                        label: etiquette.trim().to_string(),
                        max: max_rpm,
                    });
                    n += 1;
                }
            }
        }
    }
    capteurs
}
