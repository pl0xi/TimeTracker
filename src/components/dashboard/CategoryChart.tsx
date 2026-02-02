import { PieChart, Pie, Cell, ResponsiveContainer, Legend, Tooltip } from "recharts";
import { useTrackingStore } from "../../stores/trackingStore";
import { formatDuration } from "../../lib/utils";

export function CategoryChart() {
  const { categoryStats } = useTrackingStore();

  if (categoryStats.length === 0) {
    return (
      <div className="bg-[hsl(var(--card))] rounded-xl p-6 border border-[hsl(var(--border))]">
        <h2 className="text-lg font-semibold mb-4">Time by Category</h2>
        <div className="h-64 flex items-center justify-center text-[hsl(var(--muted-foreground))]">
          No data yet. Start tracking to see your time breakdown.
        </div>
      </div>
    );
  }

  const data = categoryStats.map((stat) => ({
    name: stat.category_name,
    value: stat.total_seconds,
    color: stat.color,
  }));

  return (
    <div className="bg-[hsl(var(--card))] rounded-xl p-6 border border-[hsl(var(--border))]">
      <h2 className="text-lg font-semibold mb-4">Time by Category</h2>
      <div className="h-64">
        <ResponsiveContainer width="100%" height="100%">
          <PieChart>
            <Pie
              data={data}
              cx="50%"
              cy="50%"
              innerRadius={60}
              outerRadius={80}
              paddingAngle={2}
              dataKey="value"
            >
              {data.map((entry, index) => (
                <Cell key={`cell-${index}`} fill={entry.color} />
              ))}
            </Pie>
            <Tooltip
              formatter={(value) => formatDuration(value as number)}
              contentStyle={{
                backgroundColor: "hsl(var(--card))",
                border: "1px solid hsl(var(--border))",
                borderRadius: "8px",
              }}
            />
            <Legend />
          </PieChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
