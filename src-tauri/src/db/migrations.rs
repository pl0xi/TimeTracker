use rusqlite::Connection;

pub fn run_all(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Create migrations table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _migrations (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            applied_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        )",
        [],
    )?;

    // Run each migration
    run_migration(conn, "001_initial_schema", migration_001_initial_schema)?;

    Ok(())
}

fn run_migration(
    conn: &Connection,
    name: &str,
    migration_fn: fn(&Connection) -> Result<(), rusqlite::Error>,
) -> Result<(), rusqlite::Error> {
    // Check if migration has been applied
    let applied: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM _migrations WHERE name = ?)",
            [name],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !applied {
        log::info!("Running migration: {}", name);
        migration_fn(conn)?;
        conn.execute("INSERT INTO _migrations (name) VALUES (?)", [name])?;
        log::info!("Migration {} completed", name);
    }

    Ok(())
}

fn migration_001_initial_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        r#"
        -- Application settings
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Categories for grouping applications
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            color TEXT NOT NULL DEFAULT '#6B7280',
            icon TEXT,
            is_productive INTEGER NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Projects for more granular tracking
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
            color TEXT NOT NULL DEFAULT '#3B82F6',
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Known applications with their categorization
        CREATE TABLE IF NOT EXISTS applications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            executable_path TEXT,
            identifier TEXT UNIQUE,
            category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
            project_id INTEGER REFERENCES projects(id) ON DELETE SET NULL,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Auto-categorization rules
        CREATE TABLE IF NOT EXISTS categorization_rules (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            rule_type TEXT NOT NULL CHECK (rule_type IN ('app_name', 'window_title', 'url')),
            pattern TEXT NOT NULL,
            category_id INTEGER REFERENCES categories(id) ON DELETE CASCADE,
            project_id INTEGER REFERENCES projects(id) ON DELETE SET NULL,
            priority INTEGER NOT NULL DEFAULT 0,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Activity records (the core time tracking data)
        CREATE TABLE IF NOT EXISTS activity_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            application_id INTEGER REFERENCES applications(id) ON DELETE SET NULL,
            window_title TEXT NOT NULL,
            url TEXT,
            start_time INTEGER NOT NULL,
            end_time INTEGER NOT NULL,
            duration_seconds INTEGER GENERATED ALWAYS AS (end_time - start_time) STORED,
            category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
            project_id INTEGER REFERENCES projects(id) ON DELETE SET NULL,
            is_idle INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Idle periods
        CREATE TABLE IF NOT EXISTS idle_periods (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            start_time INTEGER NOT NULL,
            end_time INTEGER,
            duration_seconds INTEGER,
            disposition TEXT CHECK (disposition IN ('discarded', 'break', 'meeting', 'other')),
            notes TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Daily summaries (pre-aggregated for performance)
        CREATE TABLE IF NOT EXISTS daily_summaries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL UNIQUE,
            total_active_seconds INTEGER NOT NULL DEFAULT 0,
            total_idle_seconds INTEGER NOT NULL DEFAULT 0,
            productive_seconds INTEGER NOT NULL DEFAULT 0,
            category_breakdown TEXT,
            app_breakdown TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );

        -- Indexes for query performance
        CREATE INDEX IF NOT EXISTS idx_activity_start_time ON activity_records(start_time);
        CREATE INDEX IF NOT EXISTS idx_activity_end_time ON activity_records(end_time);
        CREATE INDEX IF NOT EXISTS idx_activity_application ON activity_records(application_id);
        CREATE INDEX IF NOT EXISTS idx_activity_category ON activity_records(category_id);
        CREATE INDEX IF NOT EXISTS idx_activity_project ON activity_records(project_id);
        CREATE INDEX IF NOT EXISTS idx_idle_start ON idle_periods(start_time);
        CREATE INDEX IF NOT EXISTS idx_rules_priority ON categorization_rules(priority DESC);
        CREATE INDEX IF NOT EXISTS idx_daily_date ON daily_summaries(date);
        CREATE INDEX IF NOT EXISTS idx_apps_identifier ON applications(identifier);

        -- Insert default categories
        INSERT OR IGNORE INTO categories (name, color, is_productive) VALUES
            ('Work', '#22C55E', 1),
            ('Development', '#3B82F6', 1),
            ('Communication', '#8B5CF6', 1),
            ('Entertainment', '#F59E0B', 0),
            ('Social Media', '#EF4444', 0),
            ('Utilities', '#6B7280', 1),
            ('Uncategorized', '#9CA3AF', 1);

        -- Insert default settings
        INSERT OR IGNORE INTO settings (key, value) VALUES
            ('idle_threshold_seconds', '300'),
            ('tracking_enabled', 'true'),
            ('polling_interval_ms', '1000'),
            ('theme', 'system');
        "#,
    )?;

    Ok(())
}
