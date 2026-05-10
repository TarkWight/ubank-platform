create table if not exists idempotency_records (
    idempotency_key varchar(128) not null,
    user_id uuid not null,
    method varchar(16) not null,
    path varchar(256) not null,
    request_hash varchar(128) not null,
    status varchar(32) not null,
    response_status integer,
    response_body jsonb,
    error_code varchar(64),
    created_at timestamptz not null,
    updated_at timestamptz not null,
    primary key (idempotency_key, user_id)
);