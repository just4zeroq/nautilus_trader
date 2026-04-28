CREATE TABLE IF NOT EXISTS alert_history (
    id VARCHAR(64) PRIMARY KEY,
    rule_id VARCHAR(64) NOT NULL,
    rule_name VARCHAR(255) NOT NULL,
    level VARCHAR(16) NOT NULL,
    conditions JSONB NOT NULL,
    triggered_value JSONB NOT NULL,
    channels JSONB NOT NULL,
    status VARCHAR(16) NOT NULL DEFAULT 'pending',
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_alert_history_created ON alert_history(created_at DESC);
