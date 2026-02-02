use crate::db::WindowEvent;
use chrono::Utc;

pub struct WindowDetector;

impl WindowDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn get_active_window(&self) -> Option<WindowEvent> {
        match x_win::get_active_window() {
            Ok(window) => Some(WindowEvent {
                app_name: window.info.name.clone(),
                window_title: window.title.clone(),
                executable_path: Some(window.info.path.clone()),
                process_id: Some(window.info.process_id),
                timestamp: Utc::now().timestamp(),
            }),
            Err(e) => {
                log::warn!("Failed to get active window: {}", e);
                None
            }
        }
    }

    pub fn get_idle_time_seconds(&self) -> u64 {
        match user_idle::UserIdle::get_time() {
            Ok(idle) => idle.as_seconds(),
            Err(e) => {
                log::warn!("Failed to get idle time: {}", e);
                0
            }
        }
    }
}

impl Default for WindowDetector {
    fn default() -> Self {
        Self::new()
    }
}
