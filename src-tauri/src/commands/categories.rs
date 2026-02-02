use crate::db::Category;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn get_categories(state: State<'_, AppState>) -> Result<Vec<Category>, String> {
    let conn = state.db.conn();

    let mut stmt = conn
        .prepare(
            "SELECT id, name, color, icon, is_productive, created_at, updated_at
             FROM categories ORDER BY name",
        )
        .map_err(|e| e.to_string())?;

    let categories = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
                is_productive: row.get::<_, i32>(4)? == 1,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(categories)
}

#[tauri::command]
pub fn create_category(
    state: State<'_, AppState>,
    name: String,
    color: String,
    is_productive: bool,
) -> Result<Category, String> {
    let conn = state.db.conn();

    conn.execute(
        "INSERT INTO categories (name, color, is_productive) VALUES (?, ?, ?)",
        rusqlite::params![name, color, is_productive as i32],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    conn.query_row(
        "SELECT id, name, color, icon, is_productive, created_at, updated_at
         FROM categories WHERE id = ?",
        [id],
        |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
                is_productive: row.get::<_, i32>(4)? == 1,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_category(
    state: State<'_, AppState>,
    id: i64,
    name: Option<String>,
    color: Option<String>,
    is_productive: Option<bool>,
) -> Result<Category, String> {
    let conn = state.db.conn();

    if let Some(name) = &name {
        conn.execute(
            "UPDATE categories SET name = ?, updated_at = strftime('%s', 'now') WHERE id = ?",
            rusqlite::params![name, id],
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(color) = &color {
        conn.execute(
            "UPDATE categories SET color = ?, updated_at = strftime('%s', 'now') WHERE id = ?",
            rusqlite::params![color, id],
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(is_productive) = is_productive {
        conn.execute(
            "UPDATE categories SET is_productive = ?, updated_at = strftime('%s', 'now') WHERE id = ?",
            rusqlite::params![is_productive as i32, id],
        )
        .map_err(|e| e.to_string())?;
    }

    conn.query_row(
        "SELECT id, name, color, icon, is_productive, created_at, updated_at
         FROM categories WHERE id = ?",
        [id],
        |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                icon: row.get(3)?,
                is_productive: row.get::<_, i32>(4)? == 1,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_category(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.conn();

    conn.execute("DELETE FROM categories WHERE id = ?", [id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn assign_app_to_category(
    state: State<'_, AppState>,
    app_id: i64,
    category_id: Option<i64>,
) -> Result<(), String> {
    let conn = state.db.conn();

    conn.execute(
        "UPDATE applications SET category_id = ?, updated_at = strftime('%s', 'now') WHERE id = ?",
        rusqlite::params![category_id, app_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
