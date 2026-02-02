use crate::db::{TrackingStatus, WindowEvent};
use crate::state::AppState;
use crate::tracking::WindowDetector;
use tauri::State;

#[tauri::command]
pub fn start_tracking(state: State<'_, AppState>) -> Result<(), String> {
    state.set_tracking(true);
    let now = chrono::Utc::now().timestamp();
    state.set_session_start(Some(now));
    log::info!("Tracking started");
    Ok(())
}

#[tauri::command]
pub fn stop_tracking(state: State<'_, AppState>) -> Result<(), String> {
    state.set_tracking(false);
    state.set_session_start(None);
    log::info!("Tracking stopped");
    Ok(())
}

#[tauri::command]
pub fn get_tracking_status(state: State<'_, AppState>) -> Result<TrackingStatus, String> {
    Ok(state.get_tracking_status())
}

#[tauri::command]
pub fn get_current_window() -> Result<Option<WindowEvent>, String> {
    let detector = WindowDetector::new();
    Ok(detector.get_active_window())
}

#[tauri::command]
pub fn get_idle_time() -> Result<u64, String> {
    let detector = WindowDetector::new();
    Ok(detector.get_idle_time_seconds())
}
