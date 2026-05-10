alter table monitoring_events
    add column if not exists transport varchar(16) null;

create index if not exists idx_monitoring_events_transport
    on monitoring_events(transport);

create index if not exists idx_monitoring_events_service_transport
    on monitoring_events(service, transport);