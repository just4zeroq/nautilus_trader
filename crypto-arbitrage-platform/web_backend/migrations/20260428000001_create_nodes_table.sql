CREATE TABLE IF NOT EXISTS nodes (
    id VARCHAR(64) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    node_type VARCHAR(32) NOT NULL,
    exchange VARCHAR(32),
    status VARCHAR(16) NOT NULL DEFAULT 'offline',
    last_heartbeat TIMESTAMPTZ,
    latency BIGINT,
    version VARCHAR(32),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_nodes_type ON nodes(node_type);
CREATE INDEX IF NOT EXISTS idx_nodes_status ON nodes(status);
