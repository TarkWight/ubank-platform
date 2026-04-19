alter table monitoring_events
    add column if not exists idempotency_key varchar(128) null;

create index if not exists idx_monitoring_events_trace_id
    on monitoring_events(trace_id);

create index if not exists idx_monitoring_events_idempotency_key
    on monitoring_events(idempotency_key);

create index if not exists idx_monitoring_events_service_event_timestamp
    on monitoring_events(service, event_timestamp desc);

create index if not exists idx_monitoring_events_trace_idem
    on monitoring_events(trace_id, idempotency_key);