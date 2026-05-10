import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import { monitoringApi } from "../api/monitoringApi";
import type { TraceResponse } from "../api/types";

export function TracePage() {
  const { traceId } = useParams<{ traceId: string }>();
  const [trace, setTrace] = useState<TraceResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    if (!traceId) {
      setError("traceId is required");
      return;
    }

    try {
      setError(null);
      const response = await monitoringApi.getTrace(traceId);
      setTrace(response);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    }
  }

  useEffect(() => {
    void load();
  }, [traceId]);

  if (error) {
    return <div className="error-box">{error}</div>;
  }

  if (!trace) {
    return <div>Loading...</div>;
  }

  return (
    <section>
      <div className="page-header">
        <div>
          <h1>Trace details</h1>
          <div className="mono">{trace.traceId}</div>
        </div>
        <button onClick={load}>Refresh</button>
      </div>

      <div className="metrics-grid">
        <div className="card metric-card">
          <div className="metric-title">Events</div>
          <div className="metric-value">{trace.eventCount}</div>
        </div>
        <div className="card metric-card">
          <div className="metric-title">Duration</div>
          <div className="metric-value">{trace.durationMs ?? "—"}</div>
        </div>
        <div className="card metric-card">
          <div className="metric-title">Started</div>
          <div className="metric-value small-metric">
            {trace.startedAt ? new Date(trace.startedAt).toLocaleString() : "—"}
          </div>
        </div>
        <div className="card metric-card">
          <div className="metric-title">Finished</div>
          <div className="metric-value small-metric">
            {trace.finishedAt ? new Date(trace.finishedAt).toLocaleString() : "—"}
          </div>
        </div>
      </div>

      <div className="card table-card">
        <table>
          <thead>
            <tr>
              <th>Time</th>
              <th>Service</th>
              <th>Transport</th>
              <th>Operation</th>
              <th>Type</th>
              <th>Status</th>
              <th>Duration</th>
              <th>Idempotency</th>
              <th>Error</th>
            </tr>
          </thead>
          <tbody>
            {trace.events.map((event) => (
              <tr key={event.id}>
                <td>{new Date(event.timestamp).toLocaleString()}</td>
                <td>{event.service}</td>
                <td>{event.transport ?? "—"}</td>
                <td>{event.operation ?? "—"}</td>
                <td>
                  <span className={`badge ${event.eventType.toLowerCase()}`}>
                    {event.eventType}
                  </span>
                </td>
                <td>{event.status ?? "—"}</td>
                <td>{event.durationMs ?? "—"}</td>
                <td className="mono">
                  {event.idempotencyKey ? (
                    <Link to={`/idempotency/${encodeURIComponent(event.idempotencyKey)}`}>
                      {event.idempotencyKey}
                    </Link>
                  ) : (
                    "—"
                  )}
                </td>
                <td>{event.error?.message ?? "—"}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}