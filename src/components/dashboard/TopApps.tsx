import { useTrackingStore } from "../../stores/trackingStore";
import { formatDuration } from "../../lib/utils";

export function TopApps() {
  const { appUsage } = useTrackingStore();

  const topApps = appUsage.slice(0, 5);

  return (
    <div className="bg-[hsl(var(--card))] rounded-xl p-6 border border-[hsl(var(--border))]">
      <h2 className="text-lg font-semibold mb-4">Top Applications</h2>

      {topApps.length === 0 ? (
        <div className="text-center py-8 text-[hsl(var(--muted-foreground))]">
          No apps tracked yet today.
        </div>
      ) : (
        <div className="space-y-3">
          {topApps.map((app) => {
            const maxSeconds = topApps[0]?.total_seconds ?? 1;
            const percentage = (app.total_seconds / maxSeconds) * 100;

            return (
              <div key={app.app_id} className="space-y-1">
                <div className="flex items-center justify-between text-sm">
                  <span className="font-medium truncate flex-1">{app.app_name}</span>
                  <span className="text-[hsl(var(--muted-foreground))] ml-2">
                    {formatDuration(app.total_seconds)}
                  </span>
                </div>
                <div className="h-2 bg-[hsl(var(--muted))] rounded-full overflow-hidden">
                  <div
                    className="h-full rounded-full transition-all"
                    style={{
                      width: `${percentage}%`,
                      backgroundColor: app.category_color ?? "hsl(var(--primary))",
                    }}
                  />
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
