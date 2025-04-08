
#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: TaskStats,
    pub cpu_usage: f32,
    pub mem_usage: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskStats {
    Running,             // R : En cours d'exécution ou prêt à être exécuté
    Sleeping,            // S : Sommeil interrompable (en attente d'un événement ou d'une ressource)
    DiskSleep,           // D : Sommeil ininterrompu (attente d'une opération d'E/S sur le disque)
    Stopped,             // T : Arrêté (sur un signal)
    TracingStop,         // t : Arrêté par un débogueur (depuis Linux 2.6.33)
    Zombie,              // Z : Terminé mais non encore récupéré par le parent
    Dead,                // X : Mort
    WakeKill,            // K : Réveillé par un signal de type "kill" (Linux 2.6.33 à 3.13)
    Waking,              // W : En cours de réveil (Linux 2.6.33 à 3.13)
    Parked,              // P : En attente stationnaire (Linux 3.9 à 3.13)
    Idle,                // I : Inactif (Linux, macOS, FreeBSD)
    Locked,              // L : Verrouillé (FreeBSD)
    Uninterruptible,     // D : Sommeil ininterrompu (attente d'une opération d'E/S sur le disque)
}


impl Default for TaskStats {
    fn default() -> Self {
        TaskStats::Stopped
    }
}

#[derive(Default, Clone)]
pub struct ProcessList{
    pub tasks: Vec<ProcessInfo>,
    pub total_tasks: u32
}