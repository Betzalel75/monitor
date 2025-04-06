use slint::{SharedString, Timer};
use std::time::Duration;
use std::{cell::RefCell, error::Error, rc::Rc};
use system_monitor::models::network::InterfaceStats;
use system_monitor::models::{memory::MemoryInfo, system::SystemInfo};
use system_monitor::utils::formater::{convert_memory_size, format_memory_size};
use system_monitor::utils::get_tasks::get_total_tasks;

slint::include_modules!();

#[derive(Default)]
struct AppState {
    system_info: SystemInfo,
    memory_info: MemoryInfo,
    network_info: InterfaceStats,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app_state = Rc::new(RefCell::new(AppState::default()));
    let ui = AppWindow::new()?;

    // Initialisation
    {
        let mut state = app_state.borrow_mut();
        state.system_info = SystemInfo::new();
        state.system_info.set_tastks(get_total_tasks());
        state.memory_info = MemoryInfo::new();
        state.memory_info.update();
        state.network_info = InterfaceStats::new();
        state.network_info.update();
    }

    // Gestionnaire de rafraîchissement
    let ui_handle = ui.as_weak();
    let app_state_refresh = app_state.clone();
    ui.on_refresh(move || {
        let ui = ui_handle.unwrap();
        let mut state = app_state_refresh.borrow_mut();
        // Mise à jour des données
        state.memory_info.update();
        state.network_info.update();

        // Mise à jour de l'interface
        update_ui(&ui, &state);
    });

    // Timer pour les mises à jour automatiques
    let ui_handle = ui.as_weak();
    Timer::default().start(
        slint::TimerMode::Repeated,
        Duration::from_secs(3),
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                ui.invoke_refresh();
            }
        },
    );

    // Mise à jour initiale
    let state = app_state.borrow();
    update_ui(&ui, &state);

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
        "{:.2}/{}",
        convert_memory_size(state.memory_info.ram_used),
        format_memory_size(state.memory_info.ram_total)
    );
    ui.set_ram_usage(SharedString::from(ram_usage));

    let swap_usage = format!(
        "{:.2}/{}",
        convert_memory_size(state.memory_info.swap_used),
        format_memory_size(state.memory_info.swap_total)
    );
    ui.set_swap_usage(SharedString::from(swap_usage));

    if let Some(interface) = state.network_info.interfaces.first() {
        if let (Some(rx), Some(tx)) = (&interface.rx_stats, &interface.tx_stats) {
            // Update interface info
            ui.set_interface_name(interface.name.clone().into());
            ui.set_ipv4(interface.ip.to_string().into());

            // Update RX stats
            ui.set_rx_bytes(rx.bytes.to_string().into());
            ui.set_rx_packets(rx.packets.to_string().into());
            ui.set_rx_errors(rx.errs.to_string().into());
            ui.set_rx_drops(rx.drop.to_string().into());
            ui.set_rx_fifo(rx.fifo.to_string().into());
            ui.set_rx_frame(rx.frame.to_string().into());
            ui.set_rx_compressed(rx.compressed.to_string().into());
            ui.set_rx_multicast(rx.multicast.to_string().into());

            // Update TX stats
            ui.set_tx_bytes(tx.bytes.to_string().into());
            ui.set_tx_packets(tx.packets.to_string().into());
            ui.set_tx_errors(tx.errs.to_string().into());
            ui.set_tx_drops(tx.drop.to_string().into());
            ui.set_tx_fifo(tx.fifo.to_string().into());
            ui.set_tx_colls(tx.colls.to_string().into());
            ui.set_tx_carrier(tx.carrier.to_string().into());
            ui.set_tx_compressed(tx.compressed.to_string().into());
        }
    }
}
