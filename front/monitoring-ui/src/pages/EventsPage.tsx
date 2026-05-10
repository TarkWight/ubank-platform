import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { monitoringApi } from "../api/monitoringApi";
import type { EventItem } from "../api/types";

const DEFAULT_LIMIT = 50;

export function EventsPage() {
  const [items, setItems] = useState<EventItem[]>([]);
  const [service, setService] = useState("");
  const [eventType, setEventType] = useState("");
  const [transport, setTransport] = useState("");
  const [traceId, setTraceId] = useState("");
  const [idempotencyKey, setIdempotencyKey] = useState("");
  const [operation, setOperation] = useState("");

  const [limit, setLimit] = useState(DEFAULT_LIMIT);
  const [offset, setOffset] = useState(0);
  const [lastLoadedCount, setLastLoadedCount] = useState(0);

  const [error, setError] = useState<string | null>(null);

  async function load(nextOffset = offset) {
    try {
      setError(null);

      const response = await monitoringApi.getEvents({
        service,
        eventType,
        transport,
        traceId,
        idempotencyKey,
        operation,
        limit,
        offset: nextOffset,
      });

      setItems(response.items);
      setOffset(nextOffset);
      setLastLoadedCount(response.items.length);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    }
  }

  function searchFromStart() {
    void load(0);
  }

  function goPrevious() {
    const nextOffset = Math.max(offset - limit, 0);
    void load(nextOffset);
  }

  function goNext() {
    void load(offset + limit);
  }

  useEffect(() => {
    void load(0);
  }, []);

  const canGoPrevious = offset > 0;
  const canGoNext = lastLoadedCount === limit;

  return (
    <section>
      <div className="page-header">
        <div>
          <h1>Events</h1>
          <div className="muted">
            Showing {items.length} events · offset {offset} · limit {limit}
          </div>
        </div>

        <button onClick={searchFromStart}>Search</button>
      </div>

      <div className="card filters">
        <input
          placeholder="service"
          value={service}
          onChange={(e) => setService(e.target.value)}
        />

        <input
          placeholder="eventType"
          value={eventType}
          onChange={(e) => setEventType(e.target.value)}
        />

        <select value={transport} onChange={(e) => setTransport(e.target.value)}>
          <option value="">transport</option>
          <option value="HTTP">HTTP</option>
          <option value="WS">WS</option>
        </select>

        <input
          placeholder="traceId"
          value={traceId}
          onChange={(e) => setTraceId(e.target.value)}
        />

        <input
          placeholder="idempotencyKey"
          value={idempotencyKey}
          onChange={(e) => setIdempotencyKey(e.target.value)}
        />

        <input
          placeholder="operation"
          value={operation}
          onChange={(e) => setOperation(e.target.value)}
        />

        <select
          value={limit}
          onChange={(e) => {
            setLimit(Number(e.target.value));
            setOffset(0);
          }}
        >
          <option value={25}>25 per page</option>
          <option value={50}>50 per page</option>
          <option value={100}>100 per page</option>
          <option value={200}>200 per page</option>
        </select>
      </div>

      <div className="pagination-bar">
        <button disabled={!canGoPrevious} onClick={goPrevious}>
          Previous
        </button>

        <span className="muted">
          Page {Math.floor(offset / limit) + 1}
        </span>

        <button disabled={!canGoNext} onClick={goNext}>
          Next
        </button>
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
              <th>Idempotency</th>
            </tr>
          </thead>

          <tbody>
            {items.map((event) => (
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
                  <Link to={`/traces/${encodeURIComponent(event.traceId)}`}>
                    {event.traceId}
                  </Link>
                </td>
                <td className="mono">
                  {event.idempotencyKey ? (
                    <Link to={`/idempotency/${encodeURIComponent(event.idempotencyKey)}`}>
                      {event.idempotencyKey}
                    </Link>
                  ) : (
                    "—"
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="pagination-bar bottom">
        <button disabled={!canGoPrevious} onClick={goPrevious}>
          Previous
        </button>

        <span className="muted">
          Page {Math.floor(offset / limit) + 1}
        </span>

        <button disabled={!canGoNext} onClick={goNext}>
          Next
        </button>
      </div>
    </section>
  );
}