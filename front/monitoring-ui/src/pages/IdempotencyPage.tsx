import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import { monitoringApi } from "../api/monitoringApi";
import type { IdempotencyResponse } from "../api/types";

export function IdempotencyPage() {
  const { idempotencyKey } = useParams<{ idempotencyKey: string }>();
  const [data, setData] = useState<IdempotencyResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function load() {
    if (!idempotencyKey) {
      setError("idempotencyKey is required");
      return;
    }

    try {
      setError(null);
      const response = await monitoringApi.getIdempotency(idempotencyKey);
      setData(response);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    }
  }

  useEffect(() => {
    void load();
  }, [idempotencyKey]);

  if (error) {
    return <div className="error-box">{error}</div>;
  }

  if (!data) {
    return <div>Loading...</div>;
  }

  return (
    <section>
      <div className="page-header">
        <div>
          <h1>Idempotency details</h1>
          <div className="mono">{data.idempotencyKey}</div>
        </div>
        <button onClick={load}>Refresh</button>
      </div>

      <div className="metrics-grid">
        <div className="card metric-card">
          <div className="metric-title">Events</div>
          <div className="metric-value">{data.eventCount}</div>
        </div>

        <div className="card metric-card">
          <div className="metric-title">Traces</div>
          <div className="metric-value">{data.traceIds.length}</div>
        </div>
      </div>

      <div className="card table-card">
        <h2>Related traces</h2>
        <div className="chips">
          {data.traceIds.map((traceId) => (
            <Link key={traceId} className="chip mono" to={`/traces/${encodeURIComponent(traceId)}`}>
              {traceId}
            </Link>
          ))}
        </div>
      </div>

      <div className="card table-card">
        <h2>Events</h2>
        <table>
          <thead>
            <tr>
              <th>Time</th>
              <th>Trace</th>
              <th>Service</th>
              <th>Transport</th>
              <th>Operation</th>
              <th>Type</th>
              <th>Status</th>
              <th>Duration</th>
              <th>Error</th>
            </tr>
          </thead>
          <tbody>
            {data.events.map((event) => (
              <tr key={event.id}>
                <td>{new Date(event.timestamp).toLocaleString()}</td>
                <td className="mono">
                  <Link to={`/traces/${encodeURIComponent(event.traceId)}`}>
                    {event.traceId}
                  </Link>
                </td>
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
                <td>{event.error?.message ?? "—"}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}