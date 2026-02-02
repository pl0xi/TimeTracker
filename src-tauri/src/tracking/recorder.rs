use crate::db::{Database, WindowEvent};
use chrono::Utc;

pub struct ActivityRecorder {
    db: Database,
    current_activity: Option<CurrentActivity>,
}

struct CurrentActivity {
    app_name: String,
    window_title: String,
    executable_path: Option<String>,
    start_time: i64,
    application_id: Option<i64>,
}

impl ActivityRecorder {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            current_activity: None,
        }
    }

    pub fn record_window_change(&mut self, window: &WindowEvent) {
        let now = Utc::now().timestamp();

        // Check if this is a different window
        let is_different = match &self.current_activity {
            Some(current) => {
                current.app_name != window.app_name || current.window_title != window.window_title
            }
            None => true,
        };

        if is_different {
            // Save the previous activity if there was one
            if let Some(prev) = self.current_activity.take() {
                self.save_activity(&prev, now);
            }

            // Get or create application record
            let app_id = self.get_or_create_application(window);

            // Start tracking new activity
            self.current_activity = Some(CurrentActivity {
                app_name: window.app_name.clone(),
                window_title: window.window_title.clone(),
                executable_path: window.executable_path.clone(),
                start_time: now,
                application_id: app_id,
            });
        }
    }

    pub fn flush_current(&mut self) {
        if let Some(activity) = self.current_activity.take() {
            let now = Utc::now().timestamp();
            self.save_activity(&activity, now);
        }
    }

    fn save_activity(&self, activity: &CurrentActivity, end_time: i64) {
        // Don't save very short activities (less than 1 second)
        if end_time - activity.start_time < 1 {
            return;
        }

        let conn = self.db.conn();

        // Get category from application
        let category_id: Option<i64> = activity.application_id.and_then(|app_id| {
            conn.query_row(
                "SELECT category_id FROM applications WHERE id = ?",
                [app_id],
                |row| row.get(0),
            )
            .ok()
            .flatten()
        });

        let result = conn.execute(
            "INSERT INTO activity_records (application_id, window_title, start_time, end_time, category_id, is_idle)
             VALUES (?, ?, ?, ?, ?, 0)",
            rusqlite::params![
                activity.application_id,
                activity.window_title,
                activity.start_time,
                end_time,
                category_id,
            ],
        );

        if let Err(e) = result {
            log::error!("Failed to save activity record: {}", e);
        }
    }

    fn get_or_create_application(&self, window: &WindowEvent) -> Option<i64> {
        let conn = self.db.conn();

        // Try to find existing application by executable path or name
        let identifier = window.executable_path.as_ref().unwrap_or(&window.app_name);

        let existing: Option<i64> = conn
            .query_row(
                "SELECT id FROM applications WHERE identifier = ? OR name = ?",
                [identifier, &window.app_name],
                |row| row.get(0),
            )
            .ok();

        if let Some(id) = existing {
            return Some(id);
        }

        // Create new application record
        let result = conn.execute(
            "INSERT INTO applications (name, executable_path, identifier)
             VALUES (?, ?, ?)",
            rusqlite::params![window.app_name, window.executable_path, identifier],
        );

        match result {
            Ok(_) => Some(conn.last_insert_rowid()),
            Err(e) => {
                log::error!("Failed to create application record: {}", e);
                None
            }
        }
    }
}
