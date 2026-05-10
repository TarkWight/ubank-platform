import { useEffect, useState } from "react";
import { monitoringApi } from "../api/monitoringApi";
import type { EventItem } from "../api/types";

export function EventsPage() {
  const [items, setItems] = useState<EventItem[]>([]);
  const [service, setService] = useState("");
  const [eventType, setEventType] = useState("");
  const [transport, setTransport] = useState("");
  const [traceId, setTraceId] = useState("");
  const [idempotencyKey, setIdempotencyKey] = useState("");
  const [operation, setOperation] = useState("");
  const [error, setError] = useState<string | null>(null);

  async function load() {
    try {
      setError(null);
      const response = await monitoringApi.getEvents({
        service,
        eventType,
        transport,
        traceId,
        idempotencyKey,
        operation,
        limit: 50,
        offset: 0,
      });
      setItems(response.items);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    }
  }

  useEffect(() => {
    void load();
  }, []);

  return (
    <section>
      <div className="page-header">
        <h1>Events</h1>
        <button onClick={load}>Search</button>
      </div>

      <div className="card filters">
        <input placeholder="service" value={service} onChange={(e) => setService(e.target.value)} />
        <input placeholder="eventType" value={eventType} onChange={(e) => setEventType(e.target.value)} />
        <select value={transport} onChange={(e) => setTransport(e.target.value)}>
          <option value="">transport</option>
          <option value="HTTP">HTTP</option>
          <option value="WS">WS</option>
        </select>
        <input placeholder="traceId" value={traceId} onChange={(e) => setTraceId(e.target.value)} />
        <input placeholder="idempotencyKey" value={idempotencyKey} onChange={(e) => setIdempotencyKey(e.target.value)} />
        <input placeholder="operation" value={operation} onChange={(e) => setOperation(e.target.value)} />
      </div>

      {error && <div className="error-box">{error}</div>}

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
              <th>Trace</th>
            </tr>
          </thead>
          <tbody>
            {items.map((event) => (
              <tr key={event.id}>
                <td>{new Date(event.timestamp).toLocaleString()}</td>
                <td>{event.service}</td>
                <td>{event.transport ?? "—"}</td>
                <td>{event.operation ?? "—"}</td>
                <td><span className={`badge ${event.eventType.toLowerCase()}`}>{event.eventType}</span></td>
                <td>{event.status ?? "—"}</td>
                <td>{event.durationMs ?? "—"}</td>
                <td className="mono">{event.traceId}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}