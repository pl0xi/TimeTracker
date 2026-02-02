export interface Category {
  id: number;
  name: string;
  color: string;
  icon: string | null;
  is_productive: boolean;
  created_at: number;
  updated_at: number;
}

export interface Project {
  id: number;
  name: string;
  description: string | null;
  category_id: number | null;
  color: string;
  is_active: boolean;
  created_at: number;
  updated_at: number;
}

export interface Application {
  id: number;
  name: string;
  executable_path: string | null;
  identifier: string | null;
  category_id: number | null;
  project_id: number | null;
  created_at: number;
  updated_at: number;
}

export interface ActivityRecord {
  id: number;
  application_id: number | null;
  window_title: string;
  url: string | null;
  start_time: number;
  end_time: number;
  duration_seconds: number;
  category_id: number | null;
  project_id: number | null;
  is_idle: boolean;
}

export interface WindowEvent {
  app_name: string;
  window_title: string;
  executable_path: string | null;
  process_id: number | null;
  timestamp: number;
}

export interface TrackingStatus {
  is_tracking: boolean;
  is_idle: boolean;
  current_app: string | null;
  current_window: string | null;
  today_total_seconds: number;
  session_start_time: number | null;
}

export interface DailySummary {
  id: number;
  date: string;
  total_active_seconds: number;
  total_idle_seconds: number;
  productive_seconds: number;
  category_breakdown: string | null;
  app_breakdown: string | null;
}

export interface AppUsageStats {
  app_name: string;
  app_id: number;
  total_seconds: number;
  category_name: string | null;
  category_color: string | null;
}

export interface CategoryStats {
  category_id: number;
  category_name: string;
  color: string;
  total_seconds: number;
  is_productive: boolean;
}

export interface AppSettings {
  idle_threshold_seconds: number;
  tracking_enabled: boolean;
  polling_interval_ms: number;
  theme: string;
}
