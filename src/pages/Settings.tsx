import { useEffect, useState } from "react";
import type { AppSettings } from "../types";
import * as api from "../services/api";

export function Settings() {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

  const loadSettings = async () => {
    try {
      const data = await api.getSettings();
      setSettings(data);
    } catch (error) {
      console.error("Failed to load settings:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const saveSettings = async () => {
    if (!settings) return;

    setIsSaving(true);
    try {
      await api.updateSettings(settings);
    } catch (error) {
      console.error("Failed to save settings:", error);
    } finally {
      setIsSaving(false);
    }
  };

  useEffect(() => {
    loadSettings();
  }, []);

  if (isLoading || !settings) {
    return (
      <div className="p-6">
        <h1 className="text-2xl font-bold mb-6">Settings</h1>
        <p className="text-[hsl(var(--muted-foreground))]">Loading...</p>
      </div>
    );
  }

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">Settings</h1>

      <div className="space-y-6 max-w-2xl">
        <div className="bg-[hsl(var(--card))] rounded-xl p-6 border border-[hsl(var(--border))]">
          <h2 className="text-lg font-semibold mb-4">Tracking</h2>

          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium">Enable Tracking</p>
                <p className="text-sm text-[hsl(var(--muted-foreground))]">
                  Automatically track active windows
                </p>
              </div>
              <button
                onClick={() =>
                  setSettings((s) =>
                    s ? { ...s, tracking_enabled: !s.tracking_enabled } : s
                  )
                }
                className={`w-12 h-6 rounded-full transition-colors ${
                  settings.tracking_enabled
                    ? "bg-green-500"
                    : "bg-[hsl(var(--muted))]"
                }`}
              >
                <div
                  className={`w-5 h-5 rounded-full bg-white shadow transition-transform ${
                    settings.tracking_enabled ? "translate-x-6" : "translate-x-0.5"
                  }`}
                />
              </button>
            </div>

            <div>
              <label className="block font-medium mb-2">
                Idle Detection Threshold
              </label>
              <select
                value={settings.idle_threshold_seconds}
                onChange={(e) =>
                  setSettings((s) =>
                    s
                      ? { ...s, idle_threshold_seconds: parseInt(e.target.value) }
                      : s
                  )
                }
                className="w-full px-3 py-2 rounded-lg border border-[hsl(var(--border))] bg-[hsl(var(--background))]"
              >
                <option value={60}>1 minute</option>
                <option value={120}>2 minutes</option>
                <option value={300}>5 minutes</option>
                <option value={600}>10 minutes</option>
                <option value={900}>15 minutes</option>
              </select>
              <p className="text-sm text-[hsl(var(--muted-foreground))] mt-1">
                Time of inactivity before marking as idle
              </p>
            </div>
          </div>
        </div>

        <div className="bg-[hsl(var(--card))] rounded-xl p-6 border border-[hsl(var(--border))]">
          <h2 className="text-lg font-semibold mb-4">Appearance</h2>

          <div>
            <label className="block font-medium mb-2">Theme</label>
            <select
              value={settings.theme}
              onChange={(e) =>
                setSettings((s) => (s ? { ...s, theme: e.target.value } : s))
              }
              className="w-full px-3 py-2 rounded-lg border border-[hsl(var(--border))] bg-[hsl(var(--background))]"
            >
              <option value="system">System</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </div>
        </div>

        <button
          onClick={saveSettings}
          disabled={isSaving}
          className="px-4 py-2 bg-[hsl(var(--primary))] text-[hsl(var(--primary-foreground))] rounded-lg font-medium hover:opacity-90 transition-opacity disabled:opacity-50"
        >
          {isSaving ? "Saving..." : "Save Settings"}
        </button>
      </div>
    </div>
  );
}
