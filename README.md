# Ublox dump example

## Procedure

### Initial database setup

We start with the initial database setup.

```bash
cargo new ublox_pg_dump_demo
cd ublox_pg_dump_demo

# edit the setup for Cargo.toml based on: https://diesel.rs/guides/getting-started

echo DATABASE_URL=postgres://postgres:postgres@localhost/bp_dump_demo > .env
diesel setup

cargo add ublox
```

### Planning the relational data structure

Let's see what we can dump into Postgres database using GPS receiver.

```bash
diesel migration generate gps_logs
```

For `up.sql`:

```sql
CREATE TABLE gps_real_time (
    time TIMESTAMPTZ NOT NULL,
    lat  FLOAT8 NOT NULL,
    lon  FLOAT8 NOT NULL,
    alt  FLOAT8 NOT NULL,
    speed  FLOAT8 NOT NULL,
    heading  FLOAT8 NOT NULL,
    time_gps  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT create_hypertable('gps_real_time', by_range('time'));

CREATE INDEX ix_gps_time ON stocks_real_time (time_gps, time DESC);
```

For `down.sql`:

```sql
DROP TABLE IF EXISTS gps_real_time CASCADE;
```
