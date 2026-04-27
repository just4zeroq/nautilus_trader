-- Seed initial strategies
INSERT INTO strategies (id, name, strategy_type, enabled, params) VALUES
    ('cross-exchange-1', 'Cross Exchange Arbitrage', 'cross_exchange', true, 
     '{"spread_threshold": 10.0, "max_position": 1.0, "order_size": 0.1}'::jsonb),
    ('triangular-1', 'Triangular Arbitrage', 'triangular', false, 
     '{"spread_threshold": 0.001, "min_profit": 0.0005}'::jsonb),
    ('statistical-1', 'Statistical Arbitrage', 'statistical', false, 
     '{"lookback_period": 100, "entry_threshold": 2.0, "exit_threshold": 0.5}'::jsonb)
ON CONFLICT (id) DO NOTHING;

-- Seed initial symbols (Tier 1 pairs)
INSERT INTO symbols (id, symbol, exchange, tier, enabled) VALUES
    ('binance-btcusdt', 'BTCUSDT', 'binance', 1, true),
    ('binance-ethusdt', 'ETHUSDT', 'binance', 1, true),
    ('binance-bnbusdt', 'BNBUSDT', 'binance', 1, true),
    ('okx-btcusdt', 'BTCUSDT', 'okx', 1, true),
    ('okx-ethusdt', 'ETHUSDT', 'okx', 1, true),
    ('binance-adausdt', 'ADAUSDT', 'binance', 2, true),
    ('binance-dogeusdt', 'DOGEUSDT', 'binance', 2, true),
    ('okx-solusdt', 'SOLUSDT', 'okx', 3, false)
ON CONFLICT (symbol, exchange) DO NOTHING;

-- Seed initial accounts
INSERT INTO accounts (id, exchange, asset, balance) VALUES
    ('binance-usdt', 'binance', 'USDT', 50000.0),
    ('binance-btc', 'binance', 'BTC', 1.5),
    ('binance-eth', 'binance', 'ETH', 25.0),
    ('okx-usdt', 'okx', 'USDT', 50000.0),
    ('okx-btc', 'okx', 'BTC', 1.2)
ON CONFLICT (exchange, asset) DO NOTHING;

-- Seed alert rules
INSERT INTO alert_rules (id, name, level, condition, channels, enabled, phone_interval_secs) VALUES
    ('alert-loss-threshold', '账户亏损超限', 'P0', 'daily_loss > 1000', '{"phone", "sms", "feishu"}', true, 300),
    ('alert-strategy-crash', '策略崩溃', 'P1', 'strategy_status == ''crashed''', '{"sms", "feishu"}', true, NULL),
    ('alert-position-exceed', '持仓超限', 'P2', 'position_exceed_limit', '{"feishu"}', true, NULL),
    ('alert-strategy-status', '策略启停', 'P3', "strategy_status in ['started', 'stopped']", '{"feishu"}', true, NULL)
ON CONFLICT (id) DO NOTHING;
