-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id VARCHAR(64) PRIMARY KEY,
    exchange VARCHAR(32) NOT NULL,
    asset VARCHAR(16) NOT NULL,
    balance DOUBLE PRECISION NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(exchange, asset)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_accounts_exchange ON accounts(exchange);
CREATE INDEX IF NOT EXISTS idx_accounts_asset ON accounts(asset);
