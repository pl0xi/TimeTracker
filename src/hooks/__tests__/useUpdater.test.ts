import { describe, it, expect, vi, beforeEach } from "vitest";
import { checkForUpdates } from "../useUpdater";

// Mock the plugins
const mockCheck = vi.fn();
const mockAsk = vi.fn();
const mockRelaunch = vi.fn();
const mockDownloadAndInstall = vi.fn();

vi.mock("@tauri-apps/plugin-updater", () => ({
  check: () => mockCheck(),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  ask: (message: string, options: unknown) => mockAsk(message, options),
}));

vi.mock("@tauri-apps/plugin-process", () => ({
  relaunch: () => mockRelaunch(),
}));

describe("useUpdater", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockDownloadAndInstall.mockResolvedValue(undefined);
  });

  it("returns false when no update is available", async () => {
    mockCheck.mockResolvedValue(null);

    const result = await checkForUpdates(true);

    expect(result).toBe(false);
    expect(mockAsk).not.toHaveBeenCalled();
  });

  it("shows dialog when update is available", async () => {
    const mockUpdate = {
      version: "1.0.1",
      downloadAndInstall: mockDownloadAndInstall,
    };
    mockCheck.mockResolvedValue(mockUpdate);
    mockAsk.mockResolvedValue(true);

    await checkForUpdates();

    expect(mockAsk).toHaveBeenCalledWith(
      expect.stringContaining("1.0.1"),
      expect.objectContaining({ title: "Update Available" })
    );
  });

  it("installs update when user confirms", async () => {
    const mockUpdate = {
      version: "1.0.1",
      downloadAndInstall: mockDownloadAndInstall,
    };
    mockCheck.mockResolvedValue(mockUpdate);
    mockAsk.mockResolvedValue(true);

    await checkForUpdates();

    expect(mockDownloadAndInstall).toHaveBeenCalled();
    expect(mockRelaunch).toHaveBeenCalled();
  });

  it("does not install when user declines", async () => {
    const mockUpdate = {
      version: "1.0.1",
      downloadAndInstall: mockDownloadAndInstall,
    };
    mockCheck.mockResolvedValue(mockUpdate);
    mockAsk.mockResolvedValue(false);

    const result = await checkForUpdates();

    expect(result).toBe(false);
    expect(mockDownloadAndInstall).not.toHaveBeenCalled();
    expect(mockRelaunch).not.toHaveBeenCalled();
  });

  it("handles errors gracefully", async () => {
    mockCheck.mockRejectedValue(new Error("Network error"));

    const result = await checkForUpdates();

    expect(result).toBe(false);
  });
});
