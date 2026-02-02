import { useEffect } from "react";
import { TrackingControl } from "../components/dashboard/TrackingControl";
import { CategoryChart } from "../components/dashboard/CategoryChart";
import { TopApps } from "../components/dashboard/TopApps";
import { ProductivityScore } from "../components/dashboard/ProductivityScore";
import { useTrackingStore } from "../stores/trackingStore";
import { getToday } from "../lib/utils";

export function Dashboard() {
  const { fetchStatus, fetchTodaySummary, fetchCategoryStats, fetchAppUsage } =
    useTrackingStore();

  useEffect(() => {
    // Initial fetch
    const loadData = async () => {
      const today = getToday();
      await Promise.all([
        fetchStatus(),
        fetchTodaySummary(),
        fetchCategoryStats(today),
        fetchAppUsage(today),
      ]);
    };

    loadData();

    // Refresh every 5 seconds
    const interval = setInterval(() => {
      const today = getToday();
      fetchStatus();
      fetchTodaySummary();
      fetchCategoryStats(today);
      fetchAppUsage(today);
    }, 5000);

    return () => clearInterval(interval);
  }, [fetchStatus, fetchTodaySummary, fetchCategoryStats, fetchAppUsage]);

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">Dashboard</h1>

      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
        <TrackingControl />
        <ProductivityScore />
        <TopApps />
        <div className="lg:col-span-2">
          <CategoryChart />
        </div>
      </div>
    </div>
  );
}
