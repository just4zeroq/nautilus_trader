CREATE TABLE IF NOT EXISTS symbol_blacklist (
    id VARCHAR(64) PRIMARY KEY,
    symbol VARCHAR(32) NOT NULL,
    exchange VARCHAR(32) NOT NULL,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by VARCHAR(64) NOT NULL DEFAULT 'system'
);
CREATE INDEX IF NOT EXISTS idx_blacklist_symbol ON symbol_blacklist(symbol);
CREATE INDEX IF NOT EXISTS idx_blacklist_exchange ON symbol_blacklist(exchange);
