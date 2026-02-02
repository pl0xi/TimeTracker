import { describe, it, expect } from "vitest";
import {
  formatDuration,
  formatDurationLong,
  formatTime,
  formatDate,
  getToday,
  getProductivityPercentage,
  cn,
} from "../utils";

describe("utils", () => {
  describe("formatDuration", () => {
    it("formats seconds only", () => {
      expect(formatDuration(30)).toBe("30s");
      expect(formatDuration(59)).toBe("59s");
    });

    it("formats minutes and seconds", () => {
      expect(formatDuration(60)).toBe("1m 0s");
      expect(formatDuration(90)).toBe("1m 30s");
      expect(formatDuration(125)).toBe("2m 5s");
    });

    it("formats hours and minutes", () => {
      expect(formatDuration(3600)).toBe("1h 0m");
      expect(formatDuration(3660)).toBe("1h 1m");
      expect(formatDuration(7200)).toBe("2h 0m");
      expect(formatDuration(7320)).toBe("2h 2m");
    });
  });

  describe("formatDurationLong", () => {
    it("formats minutes with proper pluralization", () => {
      expect(formatDurationLong(60)).toBe("1 minute");
      expect(formatDurationLong(120)).toBe("2 minutes");
    });

    it("formats hours and minutes with proper pluralization", () => {
      expect(formatDurationLong(3600)).toBe("1 hour 0 minutes");
      expect(formatDurationLong(3660)).toBe("1 hour 1 minute");
      expect(formatDurationLong(7320)).toBe("2 hours 2 minutes");
    });
  });

  describe("formatTime", () => {
    it("formats timestamp to time string", () => {
      // Create a specific timestamp (12:30 PM UTC)
      const timestamp = new Date("2024-01-01T12:30:00Z").getTime() / 1000;
      const result = formatTime(timestamp);
      // Result depends on locale - may use : or . as separator
      expect(result).toMatch(/\d{1,2}[:.]\d{2}/);
    });
  });

  describe("formatDate", () => {
    it("formats date to YYYY-MM-DD", () => {
      const date = new Date("2024-06-15T12:00:00Z");
      expect(formatDate(date)).toBe("2024-06-15");
    });
  });

  describe("getToday", () => {
    it("returns today in YYYY-MM-DD format", () => {
      const today = getToday();
      expect(today).toMatch(/^\d{4}-\d{2}-\d{2}$/);
    });
  });

  describe("getProductivityPercentage", () => {
    it("returns 0 when total is 0", () => {
      expect(getProductivityPercentage(0, 0)).toBe(0);
    });

    it("calculates percentage correctly", () => {
      expect(getProductivityPercentage(50, 100)).toBe(50);
      expect(getProductivityPercentage(75, 100)).toBe(75);
      expect(getProductivityPercentage(3600, 7200)).toBe(50);
    });

    it("rounds to nearest integer", () => {
      expect(getProductivityPercentage(33, 100)).toBe(33);
      expect(getProductivityPercentage(66, 100)).toBe(66);
    });
  });

  describe("cn", () => {
    it("merges class names", () => {
      expect(cn("foo", "bar")).toBe("foo bar");
    });

    it("handles conditional classes", () => {
      expect(cn("foo", false && "bar", "baz")).toBe("foo baz");
      expect(cn("foo", true && "bar", "baz")).toBe("foo bar baz");
    });

    it("merges tailwind classes correctly", () => {
      expect(cn("px-2", "px-4")).toBe("px-4");
      expect(cn("text-red-500", "text-blue-500")).toBe("text-blue-500");
    });
  });
});
