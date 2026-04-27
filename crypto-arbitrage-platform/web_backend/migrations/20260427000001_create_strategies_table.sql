-- Create strategies table
CREATE TABLE IF NOT EXISTS strategies (
    id VARCHAR(64) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    strategy_type VARCHAR(64) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT false,
    params JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on strategy_type for filtering
CREATE INDEX IF NOT EXISTS idx_strategies_type ON strategies(strategy_type);

-- Create index on enabled for filtering active strategies
CREATE INDEX IF NOT EXISTS idx_strategies_enabled ON strategies(enabled);
