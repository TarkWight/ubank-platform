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
  const [from, setFrom] = useState("");
  const [to, setTo] = useState("");

  const [limit, setLimit] = useState(DEFAULT_LIMIT);
  const [offset, setOffset] = useState(0);
  const [lastLoadedCount, setLastLoadedCount] = useState(0);

  const [expandedEventId, setExpandedEventId] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);

  const [autoRefresh, setAutoRefresh] = useState(false);
  const [refreshIntervalMs, setRefreshIntervalMs] = useState(10000);

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
        from: toRfc3339OrUndefined(from),
        to: toRfc3339OrUndefined(to),
        limit,
        offset: nextOffset,
      });

      setItems(response.items);
      setOffset(nextOffset);
      setLastLoadedCount(response.items.length);

      if (!response.items.some((item) => item.id === expandedEventId)) {
        setExpandedEventId(null);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unknown error");
    }
  }

  function searchFromStart() {
    void load(0);
  }

  function clearFilters() {
    setService("");
    setEventType("");
    setTransport("");
    setTraceId("");
    setIdempotencyKey("");
    setOperation("");
    setFrom("");
    setTo("");
    setOffset(0);
    setExpandedEventId(null);
  }

  function goPrevious() {
    const nextOffset = Math.max(offset - limit, 0);
    void load(nextOffset);
  }

  function goNext() {
    void load(offset + limit);
  }

  function toggleEvent(eventId: number) {
    setExpandedEventId((current) => (current === eventId ? null : eventId));
  }

  useEffect(() => {
    void load(0);
  }, []);

  useEffect(() => {
    if (!autoRefresh) {
      return;
    }

    const timerId = window.setInterval(() => {
      void load(offset);
    }, refreshIntervalMs);

    return () => window.clearInterval(timerId);
  }, [
    autoRefresh,
    refreshIntervalMs,
    offset,
    limit,
    service,
    eventType,
    transport,
    traceId,
    idempotencyKey,
    operation,
    from,
    to,
  ]);

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

          <button className="secondary-button" onClick={clearFilters}>
            Clear
          </button>
          <button onClick={searchFromStart}>Search</button>
        </div>
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

        <label className="field-label">
          <span>From</span>
          <input type="datetime-local" value={from} onChange={(e) => setFrom(e.target.value)} />
        </label>

        <label className="field-label">
          <span>To</span>
          <input type="datetime-local" value={to} onChange={(e) => setTo(e.target.value)} />
        </label>

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

        <span className="muted">Page {Math.floor(offset / limit) + 1}</span>

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
            {items.map((event) => {
              const isExpanded = expandedEventId === event.id;

              return (
                <>
                  <tr
                    key={event.id}
                    className={isExpanded ? "event-row expanded" : "event-row"}
                    onClick={() => toggleEvent(event.id)}
                  >
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
                    <td className="mono" onClick={(e) => e.stopPropagation()}>
                      <Link to={`/traces/${encodeURIComponent(event.traceId)}`}>
                        {event.traceId}
                      </Link>
                    </td>
                    <td className="mono" onClick={(e) => e.stopPropagation()}>
                      {event.idempotencyKey ? (
                        <Link to={`/idempotency/${encodeURIComponent(event.idempotencyKey)}`}>
                          {event.idempotencyKey}
                        </Link>
                      ) : (
                        "—"
                      )}
                    </td>
                  </tr>

                  {isExpanded && (
                    <tr key={`${event.id}-details`} className="event-details-row">
                      <td colSpan={9}>
                        <EventDetails event={event} />
                      </td>
                    </tr>
                  )}
                </>
              );
            })}
          </tbody>
        </table>
      </div>

      <div className="pagination-bar bottom">
        <button disabled={!canGoPrevious} onClick={goPrevious}>
          Previous
        </button>

        <span className="muted">Page {Math.floor(offset / limit) + 1}</span>

        <button disabled={!canGoNext} onClick={goNext}>
          Next
        </button>
      </div>
    </section>
  );
}

function EventDetails(props: { event: EventItem }) {
  const { event } = props;

  return (
    <div className="event-details">
      <div className="details-grid">
        <Detail label="Event ID" value={event.id} />
        <Detail label="Trace ID" value={event.traceId} />
        <Detail label="Span ID" value={event.spanId} />
        <Detail label="Parent span ID" value={event.parentSpanId} />
        <Detail label="Idempotency key" value={event.idempotencyKey} />
        <Detail label="Transport" value={event.transport} />
        <Detail label="Service" value={event.service} />
        <Detail label="Operation" value={event.operation} />
        <Detail label="Event type" value={event.eventType} />
        <Detail label="Method" value={event.method} />
        <Detail label="Path" value={event.path} />
        <Detail label="Status" value={event.status} />
        <Detail label="Duration ms" value={event.durationMs} />
        <Detail label="Success" value={event.success === null ? null : String(event.success)} />
        <Detail label="Attempt" value={event.attempt} />
      </div>

      {event.error && (
        <div className="details-block">
          <h3>Error</h3>
          <div className="details-grid">
            <Detail label="Code" value={event.error.code} />
            <Detail label="Type" value={event.error.type} />
            <Detail label="Message" value={event.error.message} />
          </div>
        </div>
      )}

      <div className="details-block">
        <h3>Raw JSON</h3>
        <pre className="json-block">{JSON.stringify(event, null, 2)}</pre>
      </div>
    </div>
  );
}

function Detail(props: { label: string; value: string | number | boolean | null | undefined }) {
  return (
    <div className="detail-item">
      <div className="detail-label">{props.label}</div>
      <div className="detail-value mono">{props.value ?? "—"}</div>
    </div>
  );
}

function toRfc3339OrUndefined(value: string): string | undefined {
  if (!value) {
    return undefined;
  }

  return new Date(value).toISOString();
}