import { describe, it, expect, beforeEach, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useTrackingStore } from "../trackingStore";

// Define mock data inline to avoid hoisting issues
const mockTrackingStatus = {
  is_tracking: true,
  is_idle: false,
  current_app: "Visual Studio Code",
  current_window: "time-tracker - Visual Studio Code",
  today_total_seconds: 3600,
  session_start_time: Date.now() / 1000,
};

const mockDailySummary = {
  id: 1,
  date: new Date().toISOString().split("T")[0],
  total_active_seconds: 7200,
  total_idle_seconds: 300,
  productive_seconds: 6000,
  category_breakdown: null,
  app_breakdown: null,
};

const mockCategoryStats = [
  { category_id: 1, category_name: "Development", color: "#3B82F6", total_seconds: 3600, is_productive: true },
  { category_id: 2, category_name: "Work", color: "#22C55E", total_seconds: 1800, is_productive: true },
];

const mockAppUsage = [
  { app_name: "Visual Studio Code", app_id: 1, total_seconds: 3600, category_name: "Development", category_color: "#3B82F6" },
  { app_name: "Chrome", app_id: 2, total_seconds: 1800, category_name: "Work", category_color: "#22C55E" },
];

// Mock the API module with inline functions
vi.mock("../../services/api", () => ({
  getTrackingStatus: vi.fn(),
  getTodaySummary: vi.fn(),
  getCategoryBreakdown: vi.fn(),
  getAppUsage: vi.fn(),
  startTracking: vi.fn(),
  stopTracking: vi.fn(),
}));

import * as api from "../../services/api";

describe("trackingStore", () => {
  beforeEach(() => {
    // Reset store state before each test
    useTrackingStore.setState({
      status: null,
      todaySummary: null,
      categoryStats: [],
      appUsage: [],
      isLoading: false,
      error: null,
    });
    vi.clearAllMocks();

    // Setup default mock implementations
    vi.mocked(api.getTrackingStatus).mockResolvedValue(mockTrackingStatus);
    vi.mocked(api.getTodaySummary).mockResolvedValue(mockDailySummary);
    vi.mocked(api.getCategoryBreakdown).mockResolvedValue(mockCategoryStats);
    vi.mocked(api.getAppUsage).mockResolvedValue(mockAppUsage);
    vi.mocked(api.startTracking).mockResolvedValue(undefined);
    vi.mocked(api.stopTracking).mockResolvedValue(undefined);
  });

  it("initializes with default state", () => {
    const { result } = renderHook(() => useTrackingStore());

    expect(result.current.status).toBeNull();
    expect(result.current.todaySummary).toBeNull();
    expect(result.current.categoryStats).toEqual([]);
    expect(result.current.appUsage).toEqual([]);
    expect(result.current.isLoading).toBe(false);
    expect(result.current.error).toBeNull();
  });

  it("fetches tracking status", async () => {
    const { result } = renderHook(() => useTrackingStore());

    await act(async () => {
      await result.current.fetchStatus();
    });

    expect(api.getTrackingStatus).toHaveBeenCalled();
    expect(result.current.status).toEqual(mockTrackingStatus);
  });

  it("fetches today summary", async () => {
    const { result } = renderHook(() => useTrackingStore());

    await act(async () => {
      await result.current.fetchTodaySummary();
    });

    expect(api.getTodaySummary).toHaveBeenCalled();
    expect(result.current.todaySummary).toEqual(mockDailySummary);
  });

  it("fetches category stats", async () => {
    const { result } = renderHook(() => useTrackingStore());
    const today = new Date().toISOString().split("T")[0];

    await act(async () => {
      await result.current.fetchCategoryStats(today);
    });

    expect(api.getCategoryBreakdown).toHaveBeenCalledWith(today, today);
    expect(result.current.categoryStats).toEqual(mockCategoryStats);
  });

  it("fetches app usage", async () => {
    const { result } = renderHook(() => useTrackingStore());
    const today = new Date().toISOString().split("T")[0];

    await act(async () => {
      await result.current.fetchAppUsage(today);
    });

    expect(api.getAppUsage).toHaveBeenCalledWith(today, today);
    expect(result.current.appUsage).toEqual(mockAppUsage);
  });

  it("starts tracking", async () => {
    const { result } = renderHook(() => useTrackingStore());

    await act(async () => {
      await result.current.startTracking();
    });

    expect(api.startTracking).toHaveBeenCalled();
    expect(api.getTrackingStatus).toHaveBeenCalled();
  });

  it("stops tracking", async () => {
    const { result } = renderHook(() => useTrackingStore());

    await act(async () => {
      await result.current.stopTracking();
    });

    expect(api.stopTracking).toHaveBeenCalled();
    expect(api.getTrackingStatus).toHaveBeenCalled();
  });

  it("sets error on API failure", async () => {
    const errorMessage = "Network error";
    vi.mocked(api.getTrackingStatus).mockRejectedValueOnce(new Error(errorMessage));

    const { result } = renderHook(() => useTrackingStore());

    await act(async () => {
      await result.current.fetchStatus();
    });

    expect(result.current.error).toContain(errorMessage);
  });

  it("clears error with setError", () => {
    const { result } = renderHook(() => useTrackingStore());

    act(() => {
      result.current.setError("Some error");
    });
    expect(result.current.error).toBe("Some error");

    act(() => {
      result.current.setError(null);
    });
    expect(result.current.error).toBeNull();
  });
});
