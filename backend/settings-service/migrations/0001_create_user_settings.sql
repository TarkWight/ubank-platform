create table if not exists user_settings (
    user_id uuid not null,
    app_kind varchar(32) not null,
    theme varchar(16) not null,
    locale varchar(16) not null,
    version bigint not null default 0,
    created_at timestamptz not null,
    updated_at timestamptz not null,
    primary key (user_id, app_kind)
)