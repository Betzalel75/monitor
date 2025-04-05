
---

### 🗂️ **Structure de projet**

```sh
system_monitor/
├── Cargo.toml
├── src/
│   ├── main.rs                   # Point d'entrée, initialise l’UI
│   ├── ui/                       # Tout ce qui est lié à Slint
│   │   ├── mod.rs
│   │   ├── main_window.slint     # Interface principale
│   │   ├── cpu_tab.slint
│   │   ├── memory_tab.slint
│   │   └── network_tab.slint
│   ├── app/                      # Logique métier (backend de l’app)
│   │   ├── mod.rs
│   │   ├── cpu.rs                # Lecture /proc/cpuinfo, calcul %, etc.
│   │   ├── memory.rs             # RAM, SWAP via /proc/meminfo ou sysinfo
│   │   ├── network.rs            # RX/TX, interfaces, etc.
│   │   ├── process.rs            # Liste des processus
│   │   └── system.rs             # Infos globales (OS, hostname, etc.)
│   ├── models/                   # Structures de données partagées
│   │   ├── mod.rs
│   │   ├── cpu.rs
│   │   ├── memory.rs
│   │   ├── network.rs
│   │   └── process.rs
│   └── utils.rs                 # Fonctions utilitaires (formatage, conversion, etc.)
```

---

### 📦 **Crates principales à utiliser**

| Crate        | Rôle |
|--------------|------|
| `slint`      | UI frontend |
| `sysinfo`    | Infos système rapides (CPU %, RAM, etc.) |
| `procfs`     | Accès détaillé à `/proc/` |
| `chrono`     | Gestion des timestamps |
| `once_cell`  | Données partagées / lazy init |
| `log`, `env_logger` | Logs de debug |
| `tokio` ou `async-std` | Pour le scheduling si tu veux du rafraîchissement asynchrone |
| `plotters` ou `conrod_core` | (optionnel) graphes custom si Slint seul ne suffit pas |

---

### 🧠 **Séparation des responsabilités**

- **UI** = `.slint` + Rust glue (`ui/mod.rs`)
- **Collecte données** = modules `cpu.rs`, `memory.rs`, etc.
- **Mise à jour périodique** = boucle asynchrone (`tokio::interval`)
- **Communication UI ↔ Backend** = propriétés exportées, callbacks, modèles partagés (`SharedData`, `Rc`, `RefCell`, etc.)

---
