use crate::db::AppSettings;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let conn = state.db.conn();

    let idle_threshold: i64 = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'idle_threshold_seconds'",
            [],
            |row| row.get::<_, String>(0),
        )
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300);

    let tracking_enabled: bool = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'tracking_enabled'",
            [],
            |row| row.get::<_, String>(0),
        )
        .ok()
        .map(|v| v == "true")
        .unwrap_or(true);

    let polling_interval: i64 = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'polling_interval_ms'",
            [],
            |row| row.get::<_, String>(0),
        )
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1000);

    let theme: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'theme'",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "system".to_string());

    Ok(AppSettings {
        idle_threshold_seconds: idle_threshold,
        tracking_enabled,
        polling_interval_ms: polling_interval,
        theme,
    })
}

#[tauri::command]
pub fn update_settings(state: State<'_, AppState>, settings: AppSettings) -> Result<(), String> {
    let conn = state.db.conn();

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at)
         VALUES ('idle_threshold_seconds', ?, strftime('%s', 'now'))",
        [settings.idle_threshold_seconds.to_string()],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at)
         VALUES ('tracking_enabled', ?, strftime('%s', 'now'))",
        [settings.tracking_enabled.to_string()],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at)
         VALUES ('polling_interval_ms', ?, strftime('%s', 'now'))",
        [settings.polling_interval_ms.to_string()],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at)
         VALUES ('theme', ?, strftime('%s', 'now'))",
        [&settings.theme],
    )
    .map_err(|e| e.to_string())?;

    // Update app state
    state.set_tracking(settings.tracking_enabled);

    Ok(())
}

#[tauri::command]
pub fn get_idle_threshold(state: State<'_, AppState>) -> Result<u64, String> {
    let conn = state.db.conn();

    let threshold: u64 = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'idle_threshold_seconds'",
            [],
            |row| row.get::<_, String>(0),
        )
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300);

    Ok(threshold)
}

#[tauri::command]
pub fn set_idle_threshold(state: State<'_, AppState>, seconds: u64) -> Result<(), String> {
    let conn = state.db.conn();

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at)
         VALUES ('idle_threshold_seconds', ?, strftime('%s', 'now'))",
        [seconds.to_string()],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
