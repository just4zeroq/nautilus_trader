INSERT INTO nodes (id, name, node_type, exchange, status, version) VALUES
('collector-binance-1', 'Binance Collector 1', 'collector', 'binance', 'offline', '0.1.0'),
('collector-binance-2', 'Binance Collector 2', 'collector', 'binance', 'offline', '0.1.0'),
('collector-okx-1', 'OKX Collector 1', 'collector', 'okx', 'offline', '0.1.0'),
('collector-okx-2', 'OKX Collector 2', 'collector', 'okx', 'offline', '0.1.0'),
('trader-rust-1', 'Rust Trader Engine', 'trader', NULL, 'offline', '0.1.0'),
('trader-python-1', 'Python Trader Engine', 'trader', NULL, 'offline', '0.1.0'),
('alert-manager-1', 'Alert Manager', 'alert', NULL, 'offline', '0.1.0'),
('redis-master', 'Redis Cache', 'redis', NULL, 'offline', '7.2.0')
ON CONFLICT (id) DO NOTHING;
