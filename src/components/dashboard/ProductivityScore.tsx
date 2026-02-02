import { useTrackingStore } from "../../stores/trackingStore";
import { getProductivityPercentage } from "../../lib/utils";

export function ProductivityScore() {
  const { todaySummary } = useTrackingStore();

  const totalSeconds = todaySummary?.total_active_seconds ?? 0;
  const productiveSeconds = todaySummary?.productive_seconds ?? 0;
  const score = getProductivityPercentage(productiveSeconds, totalSeconds);

  const getScoreColor = (score: number) => {
    if (score >= 80) return "text-green-500";
    if (score >= 60) return "text-yellow-500";
    if (score >= 40) return "text-orange-500";
    return "text-red-500";
  };

  const getScoreLabel = (score: number) => {
    if (score >= 80) return "Excellent";
    if (score >= 60) return "Good";
    if (score >= 40) return "Fair";
    return "Needs improvement";
  };

  return (
    <div className="bg-[hsl(var(--card))] rounded-xl p-6 border border-[hsl(var(--border))]">
      <h2 className="text-lg font-semibold mb-4">Productivity Score</h2>

      <div className="flex items-center justify-center">
        <div className="relative w-32 h-32">
          <svg className="w-full h-full transform -rotate-90">
            <circle
              cx="64"
              cy="64"
              r="56"
              stroke="hsl(var(--muted))"
              strokeWidth="12"
              fill="none"
            />
            <circle
              cx="64"
              cy="64"
              r="56"
              stroke="currentColor"
              strokeWidth="12"
              fill="none"
              strokeLinecap="round"
              strokeDasharray={`${score * 3.52} 352`}
              className={getScoreColor(score)}
            />
          </svg>
          <div className="absolute inset-0 flex flex-col items-center justify-center">
            <span className={`text-3xl font-bold ${getScoreColor(score)}`}>{score}%</span>
          </div>
        </div>
      </div>

      <p className="text-center mt-4 text-[hsl(var(--muted-foreground))]">
        {totalSeconds === 0 ? "Start tracking to see your score" : getScoreLabel(score)}
      </p>
    </div>
  );
}
