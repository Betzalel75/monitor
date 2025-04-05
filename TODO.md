On va donc définir les **modules métier** principaux à implémenter, chacun avec :

- Sa **responsabilité**
- Les **données à extraire**
- Les **fonctions attendues**
- Éventuellement une idée de **structure de données** en Rust (`struct`)

---

## 🔧 **1. `SystemInfoModule`**
### 🧭 Responsabilité :
Afficher les infos globales sur le système.

### 📄 Données à extraire :
- Type d’OS (ex : via `/proc/version` ou `uname`)
- Utilisateur courant (`whoami` ou variable d'environnement)
- Nom de la machine (`/etc/hostname` ou `hostname`)
- Nombre total de tâches (running, sleeping, etc.)

### 🧠 Structures Rust :
```rust
struct SystemInfo {
    os_type: String,
    user: String,
    hostname: String,
    total_tasks: TaskStats,
}

struct TaskStats {
    running: u32,
    sleeping: u32,
    zombie: u32,
    stopped: u32,
    uninterruptible: u32,
}
```

### 🧩 Fonctions :
- `fn get_system_info() -> SystemInfo`
- `fn parse_proc_stat() -> TaskStats`

---

## 🧠 **2. `CPUModule`**
### 🧭 Responsabilité :
Afficher le modèle, l’utilisation CPU et un graphique de performance.

### 📄 Données :
- Modèle du CPU (`/proc/cpuinfo`)
- % d’utilisation (`/proc/stat` calculé sur intervalle)
- Historique pour le graphe

### 🧠 Structures :
```rust
struct CpuInfo {
    model: String,
    usage_percent: f32,
    usage_history: Vec<f32>,
}
```

### 🧩 Fonctions :
- `fn get_cpu_model() -> String`
- `fn get_cpu_usage() -> f32`
- `fn update_usage_history(...)`

---

## 🌬️ **3. `FanModule`**
### 🧭 Responsabilité :
Afficher état du ventilateur, vitesse et graphique.

### 📄 Données :
- Fichiers dans `/sys/class/hwmon/` (parfois dans `/proc`)
- Vitesse : `fan*_input`
- État : si présent `fan*_enable`

### 🧠 Structures :
```rust
struct FanInfo {
    enabled: bool,
    speed_rpm: u32,
    level: u8,
    speed_history: Vec<u32>,
}
```

### 🧩 Fonctions :
- `fn get_fan_info() -> FanInfo`

---

## 🌡️ **4. `ThermalModule`**
### 🧭 Responsabilité :
Afficher température et graphique.

### 📄 Données :
- `/sys/class/thermal/thermal_zone*/temp`

### 🧠 Structures :
```rust
struct ThermalInfo {
    temperature_celsius: f32,
    temp_history: Vec<f32>,
}
```

### 🧩 Fonctions :
- `fn get_temperature() -> f32`

---

## 💾 **5. `MemoryModule`**
### 🧭 Responsabilité :
Afficher RAM, SWAP et disque.

### 📄 Données :
- `/proc/meminfo`
- `df` pour disque ou `/proc/self/mounts + statvfs`

### 🧠 Structures :
```rust
struct MemoryInfo {
    ram_used: u64,
    ram_total: u64,
    swap_used: u64,
    swap_total: u64,
    disk_used: u64,
    disk_total: u64,
}
```

### 🧩 Fonctions :
- `fn get_memory_info() -> MemoryInfo`

---

## 🧾 **6. `ProcessModule`**
### 🧭 Responsabilité :
Lister tous les processus avec tri/filtrage/sélection.

### 📄 Données :
- `/proc/[pid]/` → `status`, `stat`, `cmdline`

### 🧠 Structures :
```rust
struct ProcessInfo {
    pid: u32,
    name: String,
    state: String,
    cpu_percent: f32,
    mem_percent: f32,
}
```

### 🧩 Fonctions :
- `fn list_processes() -> Vec<ProcessInfo>`
- `fn filter_processes(query: &str)`
- `fn calculate_cpu_usage(pid: u32)`
- `fn calculate_mem_usage(pid: u32)`

---

## 🌐 **7. `NetworkModule`**
### 🧭 Responsabilité :
Afficher interfaces, RX/TX, progression graphique

### 📄 Données :
- `/proc/net/dev`

### 🧠 Structures :
```rust
struct InterfaceStats {
    name: String,
    ipv4: Option<String>,
    rx: NetData,
    tx: NetData,
}

struct NetData {
    bytes: u64,
    packets: u64,
    errs: u64,
    drop: u64,
    fifo: u64,
    frame_or_colls: u64, // selon RX ou TX
    compressed: u64,
    multicast_or_carrier: u64, // selon RX ou TX
}
```

### 🧩 Fonctions :
- `fn list_interfaces() -> Vec<InterfaceStats>`
- `fn format_bytes(bytes: u64) -> String` // ex : 431.78 MB
- `fn calculate_bandwidth(...)`

---

### 🔁 Commun à tous :
- Tous les modules doivent pouvoir :
  - Mettre à jour leurs données de manière périodique
  - Fournir les données à l’interface graphique
  - Être indépendants les uns des autres

---

