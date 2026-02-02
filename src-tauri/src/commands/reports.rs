use crate::db::{ActivityRecord, AppUsageStats, CategoryStats, DailySummary};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_daily_summary(state: State<'_, AppState>, date: String) -> Result<DailySummary, String> {
    let conn = state.db.conn();

    // Try to get existing summary
    let summary: Option<DailySummary> = conn
        .query_row(
            "SELECT id, date, total_active_seconds, total_idle_seconds, productive_seconds,
                    category_breakdown, app_breakdown
             FROM daily_summaries WHERE date = ?",
            [&date],
            |row| {
                Ok(DailySummary {
                    id: row.get(0)?,
                    date: row.get(1)?,
                    total_active_seconds: row.get(2)?,
                    total_idle_seconds: row.get(3)?,
                    productive_seconds: row.get(4)?,
                    category_breakdown: row.get(5)?,
                    app_breakdown: row.get(6)?,
                })
            },
        )
        .ok();

    if let Some(s) = summary {
        return Ok(s);
    }

    // Calculate from activity records
    let total_active: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(duration_seconds), 0) FROM activity_records
             WHERE date(start_time, 'unixepoch', 'localtime') = ? AND is_idle = 0",
            [&date],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let total_idle: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(duration_seconds), 0) FROM idle_periods
             WHERE date(start_time, 'unixepoch', 'localtime') = ?",
            [&date],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let productive: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(ar.duration_seconds), 0)
             FROM activity_records ar
             LEFT JOIN categories c ON ar.category_id = c.id
             WHERE date(ar.start_time, 'unixepoch', 'localtime') = ?
               AND ar.is_idle = 0
               AND (c.is_productive = 1 OR ar.category_id IS NULL)",
            [&date],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok(DailySummary {
        id: 0,
        date,
        total_active_seconds: total_active,
        total_idle_seconds: total_idle,
        productive_seconds: productive,
        category_breakdown: None,
        app_breakdown: None,
    })
}

#[tauri::command]
pub fn get_activity_range(
    state: State<'_, AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<ActivityRecord>, String> {
    let conn = state.db.conn();

    let mut stmt = conn
        .prepare(
            "SELECT id, application_id, window_title, url, start_time, end_time,
                    duration_seconds, category_id, project_id, is_idle
             FROM activity_records
             WHERE date(start_time, 'unixepoch', 'localtime') >= ?
               AND date(start_time, 'unixepoch', 'localtime') <= ?
             ORDER BY start_time DESC",
        )
        .map_err(|e| e.to_string())?;

    let records = stmt
        .query_map([&start_date, &end_date], |row| {
            Ok(ActivityRecord {
                id: row.get(0)?,
                application_id: row.get(1)?,
                window_title: row.get(2)?,
                url: row.get(3)?,
                start_time: row.get(4)?,
                end_time: row.get(5)?,
                duration_seconds: row.get(6)?,
                category_id: row.get(7)?,
                project_id: row.get(8)?,
                is_idle: row.get::<_, i32>(9)? == 1,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(records)
}

#[tauri::command]
pub fn get_app_usage(
    state: State<'_, AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<AppUsageStats>, String> {
    let conn = state.db.conn();

    let mut stmt = conn
        .prepare(
            "SELECT a.name, a.id, COALESCE(SUM(ar.duration_seconds), 0) as total,
                    c.name as cat_name, c.color as cat_color
             FROM applications a
             LEFT JOIN activity_records ar ON a.id = ar.application_id
                AND date(ar.start_time, 'unixepoch', 'localtime') >= ?
                AND date(ar.start_time, 'unixepoch', 'localtime') <= ?
             LEFT JOIN categories c ON a.category_id = c.id
             GROUP BY a.id
             HAVING total > 0
             ORDER BY total DESC",
        )
        .map_err(|e| e.to_string())?;

    let stats = stmt
        .query_map([&start_date, &end_date], |row| {
            Ok(AppUsageStats {
                app_name: row.get(0)?,
                app_id: row.get(1)?,
                total_seconds: row.get(2)?,
                category_name: row.get(3)?,
                category_color: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(stats)
}

#[tauri::command]
pub fn get_category_breakdown(
    state: State<'_, AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<CategoryStats>, String> {
    let conn = state.db.conn();

    let mut stmt = conn
        .prepare(
            "SELECT c.id, c.name, c.color, COALESCE(SUM(ar.duration_seconds), 0) as total,
                    c.is_productive
             FROM categories c
             LEFT JOIN activity_records ar ON c.id = ar.category_id
                AND date(ar.start_time, 'unixepoch', 'localtime') >= ?
                AND date(ar.start_time, 'unixepoch', 'localtime') <= ?
             GROUP BY c.id
             HAVING total > 0
             ORDER BY total DESC",
        )
        .map_err(|e| e.to_string())?;

    let stats = stmt
        .query_map([&start_date, &end_date], |row| {
            Ok(CategoryStats {
                category_id: row.get(0)?,
                category_name: row.get(1)?,
                color: row.get(2)?,
                total_seconds: row.get(3)?,
                is_productive: row.get::<_, i32>(4)? == 1,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(stats)
}

#[tauri::command]
pub fn get_today_summary(state: State<'_, AppState>) -> Result<DailySummary, String> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    get_daily_summary(state, today)
}
