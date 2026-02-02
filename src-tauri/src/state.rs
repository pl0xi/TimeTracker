use parking_lot::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::db::{Database, TrackingStatus, WindowEvent};

pub struct AppState {
    pub db: Database,
    pub tracking_enabled: AtomicBool,
    pub is_idle: AtomicBool,
    pub current_window: RwLock<Option<WindowEvent>>,
    pub session_start: RwLock<Option<i64>>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            tracking_enabled: AtomicBool::new(true),
            is_idle: AtomicBool::new(false),
            current_window: RwLock::new(None),
            session_start: RwLock::new(None),
        }
    }

    pub fn is_tracking(&self) -> bool {
        self.tracking_enabled.load(Ordering::SeqCst)
    }

    pub fn set_tracking(&self, enabled: bool) {
        self.tracking_enabled.store(enabled, Ordering::SeqCst);
    }

    pub fn is_idle(&self) -> bool {
        self.is_idle.load(Ordering::SeqCst)
    }

    pub fn set_idle(&self, idle: bool) {
        self.is_idle.store(idle, Ordering::SeqCst);
    }

    pub fn get_current_window(&self) -> Option<WindowEvent> {
        self.current_window.read().clone()
    }

    pub fn set_current_window(&self, window: Option<WindowEvent>) {
        *self.current_window.write() = window;
    }

    pub fn get_session_start(&self) -> Option<i64> {
        *self.session_start.read()
    }

    pub fn set_session_start(&self, start: Option<i64>) {
        *self.session_start.write() = start;
    }

    pub fn get_tracking_status(&self) -> TrackingStatus {
        let current = self.get_current_window();
        TrackingStatus {
            is_tracking: self.is_tracking(),
            is_idle: self.is_idle(),
            current_app: current.as_ref().map(|w| w.app_name.clone()),
            current_window: current.as_ref().map(|w| w.window_title.clone()),
            today_total_seconds: self.get_today_total_seconds(),
            session_start_time: self.get_session_start(),
        }
    }

    fn get_today_total_seconds(&self) -> i64 {
        let conn = self.db.conn();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        conn.query_row(
            "SELECT COALESCE(SUM(duration_seconds), 0) FROM activity_records
             WHERE date(start_time, 'unixepoch', 'localtime') = ?",
            [&today],
            |row| row.get(0),
        )
        .unwrap_or(0)
    }
}
