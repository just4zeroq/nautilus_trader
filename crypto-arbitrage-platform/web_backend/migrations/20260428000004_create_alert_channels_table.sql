CREATE TABLE IF NOT EXISTS alert_channels (
    id VARCHAR(64) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    channel_type VARCHAR(16) NOT NULL,
    config JSONB NOT NULL DEFAULT '{}',
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_alert_channels_type ON alert_channels(channel_type);
