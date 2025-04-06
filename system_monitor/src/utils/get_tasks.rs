use std::{fs, path::Path};

pub fn get_total_tasks() -> u32 {
    // Pour obtenir la liste de tous les processus :
    let proc_dir = Path::new("/proc");
    let mut tasks = 0;

    if let Ok(entries) = fs::read_dir(proc_dir) {
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if let Ok(_) = name.parse::<u32>() {
                    // Le nom est un PID valide
                    tasks +=1;
                }
            }
        }
    }
    tasks
}
