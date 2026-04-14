create table if not exists monitoring_events (
    id bigserial primary key,
    trace_id text not null,
    event_type text not null,
    event_timestamp timestamptz not null,
    service text not null,
    operation text null,
    span_id text null,
    parent_span_id text null,
    method text null,
    path text null,
    status integer null,
    duration_ms bigint null,
    success boolean null,
    attempt integer null,
    error_code text null,
    error_type text null,
    error_message text null,
    raw_payload jsonb not null,
    created_at timestamptz not null default now()
);

create index if not exists idx_monitoring_events_trace_id
    on monitoring_events(trace_id);

create index if not exists idx_monitoring_events_service
    on monitoring_events(service);

create index if not exists idx_monitoring_events_event_timestamp
    on monitoring_events(event_timestamp desc);

create index if not exists idx_monitoring_events_event_type
    on monitoring_events(event_type);

create index if not exists idx_monitoring_events_service_event_timestamp
    on monitoring_events(service, event_timestamp desc);

create index if not exists idx_monitoring_events_trace_span
    on monitoring_events(trace_id, span_id);
