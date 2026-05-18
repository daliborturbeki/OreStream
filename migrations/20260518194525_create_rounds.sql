-- Add migration script here
CREATE TABLE rounds (
    round_id     BIGINT PRIMARY KEY,
    round_address TEXT,
    motherlode   DOUBLE PRECISION,
    total_deployed DOUBLE PRECISION,
    deploy_count BIGINT,
    recorded_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);