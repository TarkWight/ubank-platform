import { useEffect, useState } from "react";
import {
  Area,
  AreaChart,
  Bar,
  BarChart,
  CartesianGrid,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
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

function formatTime(value: string): string {
  return new Date(value).toLocaleTimeString();
}

export function OverviewPage() {
  const [overview, setOverview] = useState<OverviewMetricsResponse | null>(null);
  const [timeseries, setTimeseries] = useState<MetricsTimeseriesPoint[]>([]);
  const [error, setError] = useState<string | null>(null);

  const [autoRefresh, setAutoRefresh] = useState(false);
  const [refreshIntervalMs, setRefreshIntervalMs] = useState(10000);
  const [bucket, setBucket] = useState<"minute" | "hour">("minute");

  async function load() {
    try {
      setError(null);

      const [overviewData, timeseriesData] = await Promise.all([
        monitoringApi.getOverview(),
        monitoringApi.getTimeseries(bucket),
      ]);

      setOverview(overviewData);
      setTimeseries(timeseriesData.items);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    }
  }

  useEffect(() => {
    void load();
  }, [bucket]);

  useEffect(() => {
    if (!autoRefresh) {
      return;
    }

    const timerId = window.setInterval(() => {
      void load();
    }, refreshIntervalMs);

    return () => window.clearInterval(timerId);
  }, [autoRefresh, refreshIntervalMs, bucket]);

  if (error) {
    return <div className="error-box">{error}</div>;
  }

  if (!overview) {
    return <div>Loading...</div>;
  }

  return (
    <section>
      <div className="page-header">
        <h1>Overview</h1>

        <div className="actions">
          <select
            className="compact-select"
            value={bucket}
            onChange={(e) => setBucket(e.target.value as "minute" | "hour")}
          >
            <option value="minute">Minute</option>
            <option value="hour">Hour</option>
          </select>

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

      <div className="charts-grid">
        <div className="card chart-card">
          <h2>Error rate</h2>
          <ResponsiveContainer width="100%" height={280}>
            <LineChart data={timeseries}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="bucketStart" tickFormatter={formatTime} />
              <YAxis />
              <Tooltip labelFormatter={(value) => new Date(String(value)).toLocaleString()} />
              <Line
                type="monotone"
                dataKey="errorRatePercent"
                strokeWidth={2}
                dot={false}
                name="Error rate %"
              />
            </LineChart>
          </ResponsiveContainer>
        </div>

        <div className="card chart-card">
          <h2>Average duration</h2>
          <ResponsiveContainer width="100%" height={280}>
            <AreaChart data={timeseries}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="bucketStart" tickFormatter={formatTime} />
              <YAxis />
              <Tooltip labelFormatter={(value) => new Date(String(value)).toLocaleString()} />
              <Area
                type="monotone"
                dataKey="avgDurationMs"
                strokeWidth={2}
                fillOpacity={0.2}
                name="Avg duration ms"
              />
            </AreaChart>
          </ResponsiveContainer>
        </div>

        <div className="card chart-card wide-chart">
          <h2>Retries / Circuit breaker</h2>
          <ResponsiveContainer width="100%" height={300}>
            <BarChart data={timeseries}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="bucketStart" tickFormatter={formatTime} />
              <YAxis />
              <Tooltip labelFormatter={(value) => new Date(String(value)).toLocaleString()} />
              <Bar dataKey="totalRetries" name="Retries" />
              <Bar dataKey="totalCircuitBreakerOpen" name="Circuit breaker open" />
            </BarChart>
          </ResponsiveContainer>
        </div>
      </div>
    </section>
  );
}