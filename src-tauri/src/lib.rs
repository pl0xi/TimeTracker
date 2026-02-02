mod commands;
mod db;
mod state;
mod tracking;
mod tray;

use db::Database;
use state::AppState;
use std::time::Duration;
use tauri::{Emitter, Manager};
use tracking::{ActivityRecorder, WindowDetector};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize logging
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Get app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            // Initialize database
            let db = Database::new(app_data_dir).expect("Failed to initialize database");

            // Create app state
            let state = AppState::new(db.clone());
            app.manage(state);

            // Setup system tray
            tray::setup_tray(app.handle())?;

            // Start background tracking loop
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                run_tracking_loop(app_handle, db);
            });

            log::info!("Time Tracker initialized successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Tracking
            commands::start_tracking,
            commands::stop_tracking,
            commands::get_tracking_status,
            commands::get_current_window,
            commands::get_idle_time,
            // Categories
            commands::get_categories,
            commands::create_category,
            commands::update_category,
            commands::delete_category,
            commands::assign_app_to_category,
            // Reports
            commands::get_daily_summary,
            commands::get_activity_range,
            commands::get_app_usage,
            commands::get_category_breakdown,
            commands::get_today_summary,
            // Settings
            commands::get_settings,
            commands::update_settings,
            commands::get_idle_threshold,
            commands::set_idle_threshold,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_tracking_loop(app_handle: tauri::AppHandle, db: Database) {
    let detector = WindowDetector::new();
    let mut recorder = ActivityRecorder::new(db.clone());
    let mut last_idle_check = std::time::Instant::now();

    // Get idle threshold from settings
    let idle_threshold: u64 = {
        let conn = db.conn();
        conn.query_row(
            "SELECT value FROM settings WHERE key = 'idle_threshold_seconds'",
            [],
            |row| row.get::<_, String>(0),
        )
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300)
    };

    loop {
        std::thread::sleep(Duration::from_secs(1));

        // Get app state
        let state = match app_handle.try_state::<AppState>() {
            Some(s) => s,
            None => continue,
        };

        // Check if tracking is enabled
        if !state.is_tracking() {
            continue;
        }

        // Check idle status periodically
        if last_idle_check.elapsed() >= Duration::from_secs(5) {
            last_idle_check = std::time::Instant::now();
            let idle_seconds = detector.get_idle_time_seconds();
            let is_idle = idle_seconds >= idle_threshold;

            if is_idle != state.is_idle() {
                state.set_idle(is_idle);
                if is_idle {
                    log::info!("User went idle after {} seconds", idle_seconds);
                    recorder.flush_current();
                } else {
                    log::info!("User returned from idle");
                }

                // Emit event to frontend
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.emit("idle:changed", is_idle);
                }
            }
        }

        // Skip window detection if idle
        if state.is_idle() {
            continue;
        }

        // Get active window
        if let Some(window) = detector.get_active_window() {
            state.set_current_window(Some(window.clone()));
            recorder.record_window_change(&window);
        }
    }
}
