create table if not exists hidden_accounts (
    user_id uuid not null,
    app_kind varchar(32) not null,
    account_id varchar(128) not null,
    created_at timestamptz not null,
    primary key (user_id, app_kind, account_id)
);