import { useEffect, useState } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  CartesianGrid,
} from "recharts";
import { monitoringApi } from "../api/monitoringApi";
import type { MetricsTimeseriesPoint, OverviewMetricsResponse } from "../api/types";

function MetricCard(props: { title: string; value: string | number }) {
  return (
    <div className="card metric-card">
      <div className="metric-title">{props.title}</div>
      <div className="metric-value">{props.value}</div>
    </div>
  );
}

export function OverviewPage() {
  const [overview, setOverview] = useState<OverviewMetricsResponse | null>(null);
  const [timeseries, setTimeseries] = useState<MetricsTimeseriesPoint[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [autoRefresh, setAutoRefresh] = useState(false);
  const [refreshIntervalMs, setRefreshIntervalMs] = useState(10000);

  async function load() {
    try {
      setError(null);
      const [overviewData, timeseriesData] = await Promise.all([
        monitoringApi.getOverview(),
        monitoringApi.getTimeseries("minute"),
      ]);

      setOverview(overviewData);
      setTimeseries(timeseriesData.items);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    }
  }

  useEffect(() => {
    void load();
  }, []);

  useEffect(() => {
  if (!autoRefresh) {
    return;
  }

  const timerId = window.setInterval(() => {
    void load();
  }, refreshIntervalMs);

  return () => window.clearInterval(timerId);
}, [autoRefresh, refreshIntervalMs]);

  if (error) {
    return <div className="error-box">{error}</div>;
  }

  if (!overview) {
    return <div>Loading...</div>;
  }

  return (
    <section>
      <div className="actions">
  <label className="checkbox-control">
    <input
      type="checkbox"
      checked={autoRefresh}
      onChange={(e) => setAutoRefresh(e.target.checked)}
    />
    Auto refresh
  </label>

  <select
    className="compact-select"
    value={refreshIntervalMs}
    onChange={(e) => setRefreshIntervalMs(Number(e.target.value))}
    disabled={!autoRefresh}
  >
    <option value={5000}>5s</option>
    <option value={10000}>10s</option>
  </select>

  <button onClick={load}>Refresh</button>
</div>

      <div className="metrics-grid">
        <MetricCard title="Total events" value={overview.totalEvents} />
        <MetricCard title="Requests" value={overview.totalRequests} />
        <MetricCard title="Errors" value={overview.totalErrors} />
        <MetricCard title="Error rate" value={`${overview.errorRatePercent.toFixed(2)}%`} />
        <MetricCard title="Avg duration" value={overview.avgDurationMs?.toFixed(1) ?? "—"} />
        <MetricCard title="Retries" value={overview.totalRetries} />
        <MetricCard title="Circuit breaker open" value={overview.totalCircuitBreakerOpen} />
        <MetricCard title="Idempotency conflicts" value={overview.totalIdempotencyConflicts} />
      </div>

      <div className="card chart-card">
        <h2>Error rate over time</h2>
        <ResponsiveContainer width="100%" height={320}>
          <LineChart data={timeseries}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="bucketStart" hide />
            <YAxis />
            <Tooltip />
            <Line type="monotone" dataKey="errorRatePercent" strokeWidth={2} dot={false} />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </section>
  );
}