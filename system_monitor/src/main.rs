#[allow(warnings)]
use log::{info, warn};
use slint::{SharedString, Timer};
use system_monitor::models::sensors::{FanInfos, ThermalInfos};
use std::time::Duration;
use std::{cell::RefCell, error::Error, rc::Rc};
use system_monitor::models::network::InterfaceStats;
use system_monitor::models::process::ProcessList;
use system_monitor::models::{cpu::CpuInfo, memory::MemoryInfo, system::SystemInfo};
use system_monitor::utils::formater::format_memory_size;

slint::include_modules!();

#[derive(Default, Clone)]
struct AppState {
    system_info: SystemInfo,
    memory_info: MemoryInfo,
    network_info: InterfaceStats,
    processes: ProcessList,
    is_filtered: bool,
    cpu_usage: CpuInfo,
    thermal_usage: ThermalInfos,
    fan_usage: FanInfos,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app_state = Rc::new(RefCell::new(AppState::default()));
    let ui = AppWindow::new()?;
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Initialisation
    {
        let mut state = app_state.borrow_mut();
        state.system_info = SystemInfo::new();
        state.processes = ProcessList::new();
        let total_tasks = state.processes.total_tasks;
        state.system_info.set_tastks(total_tasks);
        state.memory_info = MemoryInfo::new();
        state.memory_info.update();
        state.network_info = InterfaceStats::new();
        state.network_info.initialize();
        let model = state.system_info.cpu_model.clone();
        state.cpu_usage = CpuInfo::new(model);
        state.cpu_usage.update_stats();
        state.thermal_usage = ThermalInfos::new();
        state.thermal_usage.update();
        state.fan_usage = FanInfos::new();
        state.fan_usage.update();
    }

    // Gestionnaire de rafraîchissement
    let _ui_handle = ui.as_weak();
    let _app_state_refresh = app_state.clone();
    // Initialisation du timer avec une valeur par défaut
    let timer = Rc::new(Timer::default());
    let refresh_interval = Rc::new(RefCell::new(Duration::from_secs(3)));

    ui.on_refresh({
        let ui_handle = ui.as_weak();
        let app_state_refresh = app_state.clone();

        move || {
            if let Some(ui) = ui_handle.upgrade() {
                // On met à jour les données pendant que le RefCell est mutable
                {
                    let state_copy = {
                        if let Ok(mut state) = app_state_refresh.try_borrow_mut() {
                            state.memory_info.update();
                            state.network_info.update();
                            state.processes.update();
                            state.cpu_usage.update_stats();
                            let total_tasks = state.processes.total_tasks;
                            state.system_info.set_tastks(total_tasks);

                            Some(AppState {
                                system_info: state.system_info.clone(),
                                memory_info: state.memory_info.clone(),
                                network_info: state.network_info.clone(),
                                processes: state.processes.clone(),
                                is_filtered: false,
                                cpu_usage: state.cpu_usage.clone(),
                                thermal_usage: state.thermal_usage.clone(),
                                fan_usage: state.fan_usage.clone(),
                            })
                        } else {
                            warn!("Conflit de borrow_mut lors du refresh UI");
                            None
                        }
                    };

                    // Mise à jour de l’UI (hors emprunt)
                    if let Some(state) = state_copy {
                        update_ui(&ui, &state);
                    }
                }
            }
        }
    });

     // Gestionnaire pour le changement d'intervalle
     let timer_clone = timer.clone();
     let refresh_interval_clone = refresh_interval.clone();
     {
        ui.on_refresh_interval_changed({
            let timer = timer_clone.clone();
            move |new_duration| {
                *refresh_interval_clone.borrow_mut() = Duration::from_millis(new_duration as u64);
                timer.stop();
                timer.start(
                    slint::TimerMode::Repeated,
                    *refresh_interval_clone.borrow(),
                    {
                        let ui = _ui_handle.clone();
                        let app_state = _app_state_refresh.clone();
                        move || {
                            if let Some(ui) = ui.upgrade() {
                                if let Ok(mut state) = app_state.try_borrow_mut() {
                                    state.cpu_usage.update_stats();
                                    state.fan_usage.update();
                                    state.thermal_usage.update();
    
                                    let state_copy = AppState {
                                        system_info: state.system_info.clone(),
                                        memory_info: state.memory_info.clone(),
                                        network_info: state.network_info.clone(),
                                        processes: state.processes.clone(),
                                        is_filtered: state.is_filtered,
                                        cpu_usage: state.cpu_usage.clone(),
                                        thermal_usage: state.thermal_usage.clone(),
                                        fan_usage: state.fan_usage.clone(),
                                    };
                                    update_ui(&ui, &state_copy);
                                }
                            }
                        }
                    }
                );
            }
        });
     }

      // Démarrage initial du timer
    timer.start(
        slint::TimerMode::Repeated,
        *refresh_interval.borrow(),
        {
            let ui_handle = ui.as_weak();
            let app_state_timer = app_state.clone();
            move || {
                if let Some(ui) = ui_handle.upgrade() {
                    if let Ok(mut state) = app_state_timer.try_borrow_mut() {
                        if state.is_filtered {
                            return;
                        }
                        state.cpu_usage.update_stats();

                        let state_copy = AppState {
                            system_info: state.system_info.clone(),
                            memory_info: state.memory_info.clone(),
                            network_info: state.network_info.clone(),
                            processes: state.processes.clone(),
                            is_filtered: state.is_filtered,
                            cpu_usage: state.cpu_usage.clone(),
                            thermal_usage: state.thermal_usage.clone(),
                            fan_usage: state.fan_usage.clone(),
                        };
                        update_ui(&ui, &state_copy);
                    }
                }
            }
        }
    );

    // Gestionnaire pour la recherche de processus
    ui.on_search({
        let app_state_filter = app_state.clone();
        let ui_handle = ui.as_weak();

        move |query: slint::SharedString| -> slint::ModelRc<ProcessData> {
            if let Ok(mut state) = app_state_filter.try_borrow_mut() {
                if !query.trim().is_empty() {
                    state.is_filtered = true;
                }
                let filtered = state.processes.filter_by_name(&query.to_string());

                let process_rows: Vec<ProcessData> = filtered
                    .iter()
                    .map(|process| ProcessData {
                        pid: process.pid.to_string().into(),
                        name: process.name.clone().into(),
                        state: process.state.to_string().into(),
                        cpu: format!("{:.1}%", process.cpu_usage).into(),
                        memory: format!("{:.1}%", process.mem_usage).into(),
                    })
                    .collect();

                let model = slint::ModelRc::new(slint::VecModel::from(process_rows));

                // Important: mettre à jour l'interface avec le nouveau modèle
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_process_list(model.clone());
                }

                model
            } else {
                // Retourner une liste vide en cas d'erreur
                slint::ModelRc::new(slint::VecModel::from(Vec::<ProcessData>::new()))
            }
        }
    });

    // ✅ Rafraîchissement automatique avec Timer persistant
    let timer = Rc::new(Timer::default());
    {
        let ui_handle = ui.as_weak();
        let app_state_timer = app_state.clone();
        let timer_clone = timer.clone();

        timer.start(
            slint::TimerMode::Repeated,
            Duration::from_secs(3),
            move || {
                if let Some(ui) = ui_handle.upgrade() {
                    if let Ok(mut state) = app_state_timer.try_borrow_mut() {
                        if state.is_filtered {
                            return;
                        }
                        state.memory_info.update();
                        state.network_info.update();
                        state.processes.update();
                        // state.cpu_usage.update_stats();
                        let total_tasks = state.processes.total_tasks;
                        state.system_info.set_tastks(total_tasks);

                        let state_copy = AppState {
                            system_info: state.system_info.clone(),
                            memory_info: state.memory_info.clone(),
                            network_info: state.network_info.clone(),
                            processes: state.processes.clone(),
                            is_filtered: false,
                            cpu_usage: state.cpu_usage.clone(),
                            thermal_usage: state.thermal_usage.clone(),
                            fan_usage: state.fan_usage.clone(),
                        };
                        update_ui(&ui, &state_copy);
                    } else {
                        warn!("Conflit de borrow_mut lors du timer");
                    }
                }
            },
        );

        // empêche le timer d’être drop immédiatement
        std::mem::forget(timer_clone);
    }

    // Mise à jour initiale
    {
        // Bloc pour emprunt immuable initial
        let state = app_state.borrow();
        update_ui(&ui, &state);
    } // Emprunt immuable libéré
    ui.run()?;
    Ok(())
}

fn update_ui(ui: &AppWindow, state: &AppState) {
    ui.set_os_type(SharedString::from(state.system_info.os_type.clone()));
    ui.set_user(SharedString::from(state.system_info.user.clone()));
    ui.set_hostname(SharedString::from(state.system_info.hostname.clone()));
    ui.set_cpu_model(SharedString::from(state.system_info.cpu_model.clone()));
    ui.set_total_tasks(state.system_info.total_tasks as i32);

    let ram_usage = format!(
        "{}/{}",
        format_memory_size(state.memory_info.ram_used),
        format_memory_size(state.memory_info.ram_total)
    );

    ui.set_ram_usage(SharedString::from(ram_usage));
    ui.set_ram_usagel(usage_memory(
        state.memory_info.ram_total,
        state.memory_info.ram_used,
    ));

    let swap_usage = format!(
        "{}/{}",
        format_memory_size(state.memory_info.swap_used),
        format_memory_size(state.memory_info.swap_total)
    );
    ui.set_swap_usage(SharedString::from(swap_usage));
    ui.set_swap_usagel(usage_memory(
        state.memory_info.swap_total,
        state.memory_info.swap_used,
    ));

    let sizing = 1024.0;
    let disk_usage = format!(
        "{}/{}",
        format_memory_size(state.memory_info.disk_used / sizing),
        format_memory_size(state.memory_info.disk_total / sizing)
    );
    ui.set_disk_usage(SharedString::from(disk_usage));
    ui.set_disk_usagel(usage_memory(
        state.memory_info.disk_total,
        state.memory_info.disk_used,
    ));

    use slint::ModelRc;
    use slint::VecModel;

    let mut rx_rows = Vec::new();
    let mut tx_rows = Vec::new();
    let mut interface_list = Vec::new();
    let cpu_usage: Vec<Cores> = state
        .cpu_usage
        .stats
        .iter()
        .map(|core| Cores {
            id: core.id.clone().into(),
            values: Points {
                a: core.history[0],
                b: core.history[1],
                c: core.history[2],
                d: core.history[3],
                e: core.history[4],
                f: core.history[5],
                g: core.history[6],
                h: core.history[7],
                i: core.history[8],
            },
        })
        .collect();
    let fan_usage:Vec<Cores> = state
        .fan_usage
        .fans
        .iter()
        .map(|core| Cores {
            id: core.id.clone().into(),
            values: Points {
                a: core.history[0],
                b: core.history[1],
                c: core.history[2],
                d: core.history[3],
                e: core.history[4],
                f: core.history[5],
                g: core.history[6],
                h: core.history[7],
                i: core.history[8],
            },
        })
        .collect();
    
    let thermal_usage:Vec<Cores> = state
        .thermal_usage
        .thermals
        .iter()
        .map(|core| Cores {
            id: core.id.clone().into(),
            values: Points {
                a: core.history[0],
                b: core.history[1],
                c: core.history[2],
                d: core.history[3],
                e: core.history[4],
                f: core.history[5],
                g: core.history[6],
                h: core.history[7],
                i: core.history[8],
            },
        })
        .collect();

    for interface in &state.network_info.interfaces {
        // Ajouter l'interface à la liste
        interface_list.push(InterfaceInfo {
            name: interface.name.clone().into(),
            ip: interface.ip.to_string().into(),
        });

        if let (Some(rx), Some(tx)) = (&interface.rx_stats, &interface.tx_stats) {
            let name: SharedString = interface.name.clone().into();

            rx_rows.push(RxInfo {
                name: name.clone(),
                rx_bytes: rx.bytes.into(),
                rx_packets: rx.packets.to_string().into(),
                rx_errors: rx.errs.to_string().into(),
                rx_drops: rx.drop.to_string().into(),
                rx_fifo: rx.fifo.to_string().into(),
                rx_frame: rx.frame.to_string().into(),
                rx_compressed: rx.compressed.to_string().into(),
                rx_multicast: rx.multicast.to_string().into(),
            });

            tx_rows.push(TxInfo {
                name,
                tx_bytes: tx.bytes.into(),
                tx_packets: tx.packets.to_string().into(),
                tx_errors: tx.errs.to_string().into(),
                tx_drops: tx.drop.to_string().into(),
                tx_fifo: tx.fifo.to_string().into(),
                tx_colls: tx.colls.to_string().into(),
                tx_compressed: tx.compressed.to_string().into(),
                tx_carrier: tx.carrier.to_string().into(),
            });
        }
    }
    
    // Mettre à jour l'interface avec les modèles
    ui.set_rx_table(ModelRc::new(VecModel::from(rx_rows)));
    ui.set_tx_table(ModelRc::new(VecModel::from(tx_rows)));
    ui.set_interface_list(ModelRc::new(VecModel::from(interface_list)));
    let process_rows: Vec<ProcessData> = state
        .processes
        .tasks
        .iter()
        .map(|process| ProcessData {
            pid: process.pid.to_string().into(),
            name: process.name.clone().into(),
            state: process.state.to_string().into(),
            cpu: format!("{:.1}%", process.cpu_usage).into(),
            memory: format!("{:.1}%", process.mem_usage).into(),
        })
        .collect();

    ui.set_process_list(ModelRc::new(VecModel::from(process_rows)));
    ui.set_cpu_usage(ModelRc::new(VecModel::from(cpu_usage)));
    ui.set_fan_usage(ModelRc::new(VecModel::from(fan_usage)));
    ui.set_thermal_usage(ModelRc::new(VecModel::from(thermal_usage)));
}

fn usage_memory(total: f32, usage: f32) -> i32 {
    let result = ((usage / total) * 100.0) as i32;
    result
}
