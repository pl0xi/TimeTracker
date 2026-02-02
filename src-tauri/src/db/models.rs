use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub icon: Option<String>,
    pub is_productive: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub color: String,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: i64,
    pub name: String,
    pub executable_path: Option<String>,
    pub identifier: Option<String>,
    pub category_id: Option<i64>,
    pub project_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRecord {
    pub id: i64,
    pub application_id: Option<i64>,
    pub window_title: String,
    pub url: Option<String>,
    pub start_time: i64,
    pub end_time: i64,
    pub duration_seconds: i64,
    pub category_id: Option<i64>,
    pub project_id: Option<i64>,
    pub is_idle: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdlePeriod {
    pub id: i64,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub duration_seconds: Option<i64>,
    pub disposition: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySummary {
    pub id: i64,
    pub date: String,
    pub total_active_seconds: i64,
    pub total_idle_seconds: i64,
    pub productive_seconds: i64,
    pub category_breakdown: Option<String>,
    pub app_breakdown: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorizationRule {
    pub id: i64,
    pub name: String,
    pub rule_type: String,
    pub pattern: String,
    pub category_id: Option<i64>,
    pub project_id: Option<i64>,
    pub priority: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowEvent {
    pub app_name: String,
    pub window_title: String,
    pub executable_path: Option<String>,
    pub process_id: Option<u32>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingStatus {
    pub is_tracking: bool,
    pub is_idle: bool,
    pub current_app: Option<String>,
    pub current_window: Option<String>,
    pub today_total_seconds: i64,
    pub session_start_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUsageStats {
    pub app_name: String,
    pub app_id: i64,
    pub total_seconds: i64,
    pub category_name: Option<String>,
    pub category_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    pub category_id: i64,
    pub category_name: String,
    pub color: String,
    pub total_seconds: i64,
    pub is_productive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub idle_threshold_seconds: i64,
    pub tracking_enabled: bool,
    pub polling_interval_ms: i64,
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            idle_threshold_seconds: 300,
            tracking_enabled: true,
            polling_interval_ms: 1000,
            theme: "system".to_string(),
        }
    }
}
