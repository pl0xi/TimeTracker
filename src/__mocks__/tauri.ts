import { vi } from "vitest";

// Mock data
export const mockTrackingStatus = {
  is_tracking: true,
  is_idle: false,
  current_app: "Visual Studio Code",
  current_window: "time-tracker - Visual Studio Code",
  today_total_seconds: 3600,
  session_start_time: Date.now() / 1000,
};

export const mockCategories = [
  { id: 1, name: "Work", color: "#22C55E", icon: null, is_productive: true, created_at: 0, updated_at: 0 },
  { id: 2, name: "Development", color: "#3B82F6", icon: null, is_productive: true, created_at: 0, updated_at: 0 },
  { id: 3, name: "Entertainment", color: "#F59E0B", icon: null, is_productive: false, created_at: 0, updated_at: 0 },
];

export const mockDailySummary = {
  id: 1,
  date: new Date().toISOString().split("T")[0],
  total_active_seconds: 7200,
  total_idle_seconds: 300,
  productive_seconds: 6000,
  category_breakdown: null,
  app_breakdown: null,
};

export const mockAppUsage = [
  { app_name: "Visual Studio Code", app_id: 1, total_seconds: 3600, category_name: "Development", category_color: "#3B82F6" },
  { app_name: "Chrome", app_id: 2, total_seconds: 1800, category_name: "Work", category_color: "#22C55E" },
  { app_name: "Slack", app_id: 3, total_seconds: 900, category_name: "Communication", category_color: "#8B5CF6" },
];

export const mockCategoryStats = [
  { category_id: 1, category_name: "Development", color: "#3B82F6", total_seconds: 3600, is_productive: true },
  { category_id: 2, category_name: "Work", color: "#22C55E", total_seconds: 1800, is_productive: true },
];

export const mockSettings = {
  idle_threshold_seconds: 300,
  tracking_enabled: true,
  polling_interval_ms: 1000,
  theme: "system",
};

// Create mock invoke function
export const createMockInvoke = () => {
  return vi.fn().mockImplementation((command: string, _args?: Record<string, unknown>) => {
    switch (command) {
      case "get_tracking_status":
        return Promise.resolve(mockTrackingStatus);
      case "start_tracking":
        return Promise.resolve();
      case "stop_tracking":
        return Promise.resolve();
      case "get_categories":
        return Promise.resolve(mockCategories);
      case "get_today_summary":
        return Promise.resolve(mockDailySummary);
      case "get_daily_summary":
        return Promise.resolve(mockDailySummary);
      case "get_app_usage":
        return Promise.resolve(mockAppUsage);
      case "get_category_breakdown":
        return Promise.resolve(mockCategoryStats);
      case "get_settings":
        return Promise.resolve(mockSettings);
      case "update_settings":
        return Promise.resolve();
      default:
        return Promise.reject(new Error(`Unknown command: ${command}`));
    }
  });
};

// Mock the @tauri-apps/api/core module
vi.mock("@tauri-apps/api/core", () => ({
  invoke: createMockInvoke(),
}));

// Mock the updater plugin
vi.mock("@tauri-apps/plugin-updater", () => ({
  check: vi.fn().mockResolvedValue(null),
}));

// Mock the process plugin
vi.mock("@tauri-apps/plugin-process", () => ({
  relaunch: vi.fn().mockResolvedValue(undefined),
}));

// Mock the dialog plugin
vi.mock("@tauri-apps/plugin-dialog", () => ({
  ask: vi.fn().mockResolvedValue(true),
}));
