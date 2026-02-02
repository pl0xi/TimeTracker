import { Play, Pause, Coffee } from "lucide-react";
import { useTrackingStore } from "../../stores/trackingStore";
import { formatDuration } from "../../lib/utils";
import { cn } from "../../lib/utils";

export function TrackingControl() {
  const { status, startTracking, stopTracking } = useTrackingStore();

  const isTracking = status?.is_tracking ?? false;
  const isIdle = status?.is_idle ?? false;
  const currentApp = status?.current_app ?? "No app";
  const currentWindow = status?.current_window ?? "";
  const todayTotal = status?.today_total_seconds ?? 0;

  return (
    <div className="bg-[hsl(var(--card))] rounded-xl p-6 border border-[hsl(var(--border))]">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold">Tracking Status</h2>
        <button
          onClick={isTracking ? stopTracking : startTracking}
          className={cn(
            "flex items-center gap-2 px-4 py-2 rounded-lg font-medium transition-colors",
            isTracking
              ? "bg-red-500/10 text-red-500 hover:bg-red-500/20"
              : "bg-green-500/10 text-green-500 hover:bg-green-500/20"
          )}
        >
          {isTracking ? (
            <>
              <Pause className="h-4 w-4" />
              Pause
            </>
          ) : (
            <>
              <Play className="h-4 w-4" />
              Start
            </>
          )}
        </button>
      </div>

      <div className="space-y-4">
        <div className="flex items-center gap-3">
          <div
            className={cn(
              "h-3 w-3 rounded-full",
              isTracking ? (isIdle ? "bg-yellow-500" : "bg-green-500") : "bg-gray-400"
            )}
          />
          <span className="text-sm text-[hsl(var(--muted-foreground))]">
            {isTracking ? (isIdle ? "Idle" : "Tracking") : "Paused"}
          </span>
        </div>

        {isIdle && (
          <div className="flex items-center gap-2 text-yellow-500 text-sm">
            <Coffee className="h-4 w-4" />
            <span>You appear to be away</span>
          </div>
        )}

        {isTracking && !isIdle && currentApp && (
          <div className="space-y-1">
            <p className="font-medium truncate">{currentApp}</p>
            <p className="text-sm text-[hsl(var(--muted-foreground))] truncate">
              {currentWindow}
            </p>
          </div>
        )}

        <div className="pt-4 border-t border-[hsl(var(--border))]">
          <p className="text-sm text-[hsl(var(--muted-foreground))]">Today's total</p>
          <p className="text-2xl font-bold">{formatDuration(todayTotal)}</p>
        </div>
      </div>
    </div>
  );
}
