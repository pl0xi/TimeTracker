pub mod connection;
pub mod migrations;
pub mod models;

pub use connection::Database;
pub use models::*;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_db() -> (Database, TempDir) {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = Database::new(temp_dir.path().to_path_buf()).expect("Failed to create database");
        (db, temp_dir)
    }

    #[test]
    fn test_database_creation() {
        let (db, _temp_dir) = create_test_db();
        let conn = db.conn();

        // Verify tables were created
        let table_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
                [],
                |row| row.get(0),
            )
            .expect("Failed to query tables");

        // Should have created several tables
        assert!(table_count >= 7, "Expected at least 7 tables, got {}", table_count);
    }

    #[test]
    fn test_default_categories_created() {
        let (db, _temp_dir) = create_test_db();
        let conn = db.conn();

        let category_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM categories", [], |row| row.get(0))
            .expect("Failed to count categories");

        // Should have default categories
        assert!(category_count > 0, "Expected default categories to be created");
    }

    #[test]
    fn test_insert_and_retrieve_category() {
        let (db, _temp_dir) = create_test_db();
        let conn = db.conn();

        // Insert a new category
        conn.execute(
            "INSERT INTO categories (name, color, is_productive) VALUES (?1, ?2, ?3)",
            ["Test Category", "#FF0000", "1"],
        ).expect("Failed to insert category");

        // Retrieve it
        let name: String = conn
            .query_row(
                "SELECT name FROM categories WHERE name = ?1",
                ["Test Category"],
                |row| row.get(0),
            )
            .expect("Failed to retrieve category");

        assert_eq!(name, "Test Category");
    }

    #[test]
    fn test_insert_activity_record() {
        let (db, _temp_dir) = create_test_db();
        let conn = db.conn();

        // Insert an application first
        conn.execute(
            "INSERT INTO applications (name) VALUES (?1)",
            ["Test App"],
        ).expect("Failed to insert application");

        let app_id: i64 = conn.last_insert_rowid();

        // Insert activity record (duration_seconds is a generated column)
        conn.execute(
            "INSERT INTO activity_records (application_id, window_title, start_time, end_time, is_idle)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![app_id, "Test Window", 1000, 2000, false],
        ).expect("Failed to insert activity record");

        // Verify count and auto-generated duration
        let (count, duration): (i64, i64) = conn
            .query_row(
                "SELECT COUNT(*), duration_seconds FROM activity_records",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("Failed to query records");

        assert_eq!(count, 1);
        assert_eq!(duration, 1000); // end_time (2000) - start_time (1000)
    }

    #[test]
    fn test_database_clone() {
        let (db, _temp_dir) = create_test_db();
        let db_clone = db.clone();

        // Both should work and access the same data
        {
            let conn = db.conn();
            conn.execute(
                "INSERT INTO applications (name) VALUES (?1)",
                ["Clone Test App"],
            ).expect("Failed to insert");
        }

        {
            let conn = db_clone.conn();
            let name: String = conn
                .query_row(
                    "SELECT name FROM applications WHERE name = ?1",
                    ["Clone Test App"],
                    |row| row.get(0),
                )
                .expect("Failed to retrieve");

            assert_eq!(name, "Clone Test App");
        }
    }

    #[test]
    fn test_app_settings_default() {
        let settings = AppSettings::default();

        assert_eq!(settings.idle_threshold_seconds, 300);
        assert!(settings.tracking_enabled);
        assert_eq!(settings.polling_interval_ms, 1000);
        assert_eq!(settings.theme, "system");
    }

    #[test]
    fn test_daily_summary_table() {
        let (db, _temp_dir) = create_test_db();
        let conn = db.conn();

        // Insert a daily summary
        conn.execute(
            "INSERT INTO daily_summaries (date, total_active_seconds, total_idle_seconds, productive_seconds)
             VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["2024-01-15", 7200, 300, 6000],
        ).expect("Failed to insert daily summary");

        // Retrieve it
        let (total_active, productive): (i64, i64) = conn
            .query_row(
                "SELECT total_active_seconds, productive_seconds FROM daily_summaries WHERE date = ?1",
                ["2024-01-15"],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("Failed to retrieve summary");

        assert_eq!(total_active, 7200);
        assert_eq!(productive, 6000);
    }
}
