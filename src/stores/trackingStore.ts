import { create } from "zustand";
import type { TrackingStatus, DailySummary, CategoryStats, AppUsageStats } from "../types";
import * as api from "../services/api";

interface TrackingState {
  status: TrackingStatus | null;
  todaySummary: DailySummary | null;
  categoryStats: CategoryStats[];
  appUsage: AppUsageStats[];
  isLoading: boolean;
  error: string | null;

  // Actions
  fetchStatus: () => Promise<void>;
  fetchTodaySummary: () => Promise<void>;
  fetchCategoryStats: (date: string) => Promise<void>;
  fetchAppUsage: (date: string) => Promise<void>;
  startTracking: () => Promise<void>;
  stopTracking: () => Promise<void>;
  setError: (error: string | null) => void;
}

export const useTrackingStore = create<TrackingState>((set, get) => ({
  status: null,
  todaySummary: null,
  categoryStats: [],
  appUsage: [],
  isLoading: false,
  error: null,

  fetchStatus: async () => {
    try {
      const status = await api.getTrackingStatus();
      set({ status });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  fetchTodaySummary: async () => {
    try {
      const todaySummary = await api.getTodaySummary();
      set({ todaySummary });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  fetchCategoryStats: async (date: string) => {
    try {
      const categoryStats = await api.getCategoryBreakdown(date, date);
      set({ categoryStats });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  fetchAppUsage: async (date: string) => {
    try {
      const appUsage = await api.getAppUsage(date, date);
      set({ appUsage });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  startTracking: async () => {
    try {
      await api.startTracking();
      await get().fetchStatus();
    } catch (error) {
      set({ error: String(error) });
    }
  },

  stopTracking: async () => {
    try {
      await api.stopTracking();
      await get().fetchStatus();
    } catch (error) {
      set({ error: String(error) });
    }
  },

  setError: (error) => set({ error }),
}));
