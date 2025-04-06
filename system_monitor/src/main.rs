use slint::{SharedString, Timer};
use std::time::Duration;
use std::{cell::RefCell, error::Error, rc::Rc};
use system_monitor::models::{memory::MemoryInfo, system::SystemInfo};

slint::include_modules!();

#[derive(Default)]
struct AppState {
    system_info: SystemInfo,
    memory_info: MemoryInfo,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app_state = Rc::new(RefCell::new(AppState::default()));
    let ui = AppWindow::new()?;

    // Initialisation
    {
        let mut state = app_state.borrow_mut();
        state.system_info = SystemInfo::new();
        state.memory_info = MemoryInfo::new();
    }

    // Gestionnaire de rafraîchissement
    let ui_handle = ui.as_weak();
    let app_state_refresh = app_state.clone();
    ui.on_refresh(move || {
        let ui = ui_handle.unwrap();
        let mut state = app_state_refresh.borrow_mut();
        // Mise à jour des données
        state.memory_info.update();

        // Mise à jour de l'interface
        update_ui(&ui, &state);
    });

    // Timer pour les mises à jour automatiques
    let ui_handle = ui.as_weak();
    Timer::default().start(
        slint::TimerMode::Repeated,
        Duration::from_secs(1),
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

    // Formatage de l'utilisation de la mémoire
    let ram_usage = format!(
        "{:.1}/{:.1} GB",
        state.memory_info.ram_used as f64 / 1_048_576.0,
        state.memory_info.ram_total as f64 / 1_048_576.0
    );
    ui.set_ram_usage(SharedString::from(ram_usage));

    let swap_usage = format!(
        "{:.1}/{:.1} GB",
        state.memory_info.swap_used as f64 / 1_048_576.0,
        state.memory_info.swap_total as f64 / 1_048_576.0
    );
    ui.set_swap_usage(SharedString::from(swap_usage));
}
