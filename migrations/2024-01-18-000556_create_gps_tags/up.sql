CREATE TABLE gps_real_time (
    ID SERIAL PRIMARY KEY,
    time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    lat  FLOAT8 NOT NULL,
    lon  FLOAT8 NOT NULL,
    alt  FLOAT8 NOT NULL,
    speed  FLOAT8 NOT NULL,
    heading  FLOAT8 NOT NULL,
    time_gps  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Run this separately for timescale DB
-- SELECT create_hypertable('gps_real_time', by_range('time'));

CREATE INDEX ix_gps_time ON gps_real_time (time_gps, time DESC);