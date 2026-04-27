-- Create alert_rules table
CREATE TABLE IF NOT EXISTS alert_rules (
    id VARCHAR(64) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    level VARCHAR(8) NOT NULL,
    condition TEXT NOT NULL,
    channels TEXT[] NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    phone_interval_secs BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_alert_rules_level ON alert_rules(level);
CREATE INDEX IF NOT EXISTS idx_alert_rules_enabled ON alert_rules(enabled);
