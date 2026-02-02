import { invoke } from "@tauri-apps/api/core";
import type {
  TrackingStatus,
  WindowEvent,
  Category,
  DailySummary,
  AppUsageStats,
  CategoryStats,
  AppSettings,
  ActivityRecord,
} from "../types";

// Tracking commands
export const startTracking = () => invoke<void>("start_tracking");
export const stopTracking = () => invoke<void>("stop_tracking");
export const getTrackingStatus = () => invoke<TrackingStatus>("get_tracking_status");
export const getCurrentWindow = () => invoke<WindowEvent | null>("get_current_window");
export const getIdleTime = () => invoke<number>("get_idle_time");

// Category commands
export const getCategories = () => invoke<Category[]>("get_categories");
export const createCategory = (name: string, color: string, isProductive: boolean) =>
  invoke<Category>("create_category", { name, color, isProductive });
export const updateCategory = (
  id: number,
  name?: string,
  color?: string,
  isProductive?: boolean
) => invoke<Category>("update_category", { id, name, color, isProductive });
export const deleteCategory = (id: number) => invoke<void>("delete_category", { id });
export const assignAppToCategory = (appId: number, categoryId: number | null) =>
  invoke<void>("assign_app_to_category", { appId, categoryId });

// Report commands
export const getDailySummary = (date: string) =>
  invoke<DailySummary>("get_daily_summary", { date });
export const getTodaySummary = () => invoke<DailySummary>("get_today_summary");
export const getActivityRange = (startDate: string, endDate: string) =>
  invoke<ActivityRecord[]>("get_activity_range", { startDate, endDate });
export const getAppUsage = (startDate: string, endDate: string) =>
  invoke<AppUsageStats[]>("get_app_usage", { startDate, endDate });
export const getCategoryBreakdown = (startDate: string, endDate: string) =>
  invoke<CategoryStats[]>("get_category_breakdown", { startDate, endDate });

// Settings commands
export const getSettings = () => invoke<AppSettings>("get_settings");
export const updateSettings = (settings: AppSettings) =>
  invoke<void>("update_settings", { settings });
export const getIdleThreshold = () => invoke<number>("get_idle_threshold");
export const setIdleThreshold = (seconds: number) =>
  invoke<void>("set_idle_threshold", { seconds });
