<template>
  <div class="dashboard">
    <div class="page-header-simple">
      <h1 class="page-title">Dashboard</h1>
      <p class="page-subtitle">Portfolio overview and performance metrics</p>
    </div>

    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon balance">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 6v12M8 10h8M8 14h8"/>
          </svg>
        </div>
        <div class="stat-content">
          <span class="stat-label">Total Balance</span>
          <span class="stat-value">$100,000.00</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon pl">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v20M2 12l10-10 10 10"/>
          </svg>
        </div>
        <div class="stat-content">
          <span class="stat-label">Daily P/L</span>
          <span class="stat-value positive">+$1,234.56</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon strategies">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
          </svg>
        </div>
        <div class="stat-content">
          <span class="stat-label">Active Strategies</span>
          <span class="stat-value">2</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon positions">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12V7a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h7"/>
            <path d="M16 19l2 2 4-4"/>
          </svg>
        </div>
        <div class="stat-content">
          <span class="stat-label">Open Positions</span>
          <span class="stat-value">5</span>
        </div>
      </div>
    </div>

    <div class="content-grid">
      <div class="panel">
        <div class="panel-header">
          <h2 class="panel-title">Recent Signals</h2>
          <span class="panel-badge">Live</span>
        </div>
        <div class="signal-list">
          <div v-for="signal in recentSignals" :key="signal.time" class="signal-item">
            <div class="signal-time">{{ signal.time }}</div>
            <div class="signal-strategy">{{ signal.strategy }}</div>
            <div class="signal-symbol">{{ signal.symbol }}</div>
            <div class="signal-action" :class="signal.signal.toLowerCase()">{{ signal.signal }}</div>
          </div>
        </div>
      </div>

      <div class="panel">
        <div class="panel-header">
          <h2 class="panel-title">Active Positions</h2>
          <span class="panel-count">{{ positions.length }}</span>
        </div>
        <table class="positions-table">
          <thead>
            <tr>
              <th>Symbol</th>
              <th>Exchange</th>
              <th>Side</th>
              <th>Quantity</th>
              <th>P/L</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="pos in positions" :key="pos.symbol">
              <td class="symbol-cell">{{ pos.symbol }}</td>
              <td>
                <span class="exchange-tag" :class="pos.exchange.toLowerCase()">
                  {{ pos.exchange === 'binance' ? 'BN' : 'OKX' }}
                </span>
              </td>
              <td>
                <span class="side-tag" :class="pos.side.toLowerCase()">{{ pos.side }}</span>
              </td>
              <td class="mono">{{ pos.quantity }}</td>
              <td class="pnl-cell" :class="parseFloat(pos.pnl) >= 0 ? 'positive' : 'negative'">
                {{ pos.pnl }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div class="arbitrage-flow">
      <div class="flow-header">
        <h2 class="panel-title">Arbitrage Flow</h2>
      </div>
      <div class="flow-diagram">
        <div class="flow-node binance">
          <div class="node-icon">BN</div>
          <div class="node-label">Binance</div>
          <div class="node-price">$65,420.50</div>
        </div>
        <div class="flow-arrow">
          <svg viewBox="0 0 100 24" fill="none">
            <path d="M0 12h90M90 6l8 6-8 6" stroke="currentColor" stroke-width="2"/>
          </svg>
          <span class="spread">+5.20</span>
        </div>
        <div class="flow-node okx">
          <div class="node-icon">OK</div>
          <div class="node-label">OKX</div>
          <div class="node-price">$65,425.70</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const recentSignals = ref([
  { time: '10:30:00', strategy: 'Cross Exchange', symbol: 'BTCUSDT', signal: 'BUY' },
  { time: '10:25:00', strategy: 'Triangular', symbol: 'ETH/USDT', signal: 'SELL' },
  { time: '10:20:00', strategy: 'Statistical', symbol: 'ADAUSDT', signal: 'CLOSE' },
  { time: '10:15:00', strategy: 'Cross Exchange', symbol: 'ETHUSDT', signal: 'BUY' }
])

const positions = ref([
  { symbol: 'BTCUSDT', exchange: 'binance', side: 'LONG', quantity: '0.5', pnl: '+200.00' },
  { symbol: 'ETHUSDT', exchange: 'okx', side: 'LONG', quantity: '5.0', pnl: '+50.00' },
  { symbol: 'ADAUSDT', exchange: 'binance', side: 'SHORT', quantity: '1000', pnl: '-25.50' }
])
</script>

<style scoped>
.dashboard {
  --bg-primary: #0a0e14;
  --bg-secondary: #111822;
  --bg-tertiary: #1a2232;
  --border-color: rgba(56, 139, 253, 0.15);
  --text-primary: #e6edf3;
  --text-secondary: #7d8590;
  --text-tertiary: #484f58;
  --accent-cyan: #00d9ff;
  --accent-green: #00ff88;
  --accent-orange: #ff9500;
  --accent-red: #ff4757;

  padding: 32px 40px;
  background: var(--bg-primary);
  min-height: 100vh;
}

.page-header-simple {
  margin-bottom: 32px;
}

.page-title {
  font-size: 32px;
  font-weight: 600;
  margin: 0 0 4px 0;
  background: linear-gradient(135deg, var(--text-primary) 0%, var(--accent-cyan) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.page-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 32px;
}

.stat-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  transition: all 0.2s ease;
}

.stat-card:hover {
  border-color: var(--accent-cyan);
  transform: translateY(-2px);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.stat-icon svg {
  width: 24px;
  height: 24px;
}

.stat-icon.balance {
  background: rgba(0, 217, 255, 0.15);
  color: var(--accent-cyan);
}

.stat-icon.pl {
  background: rgba(0, 255, 136, 0.15);
  color: var(--accent-green);
}

.stat-icon.strategies {
  background: rgba(255, 149, 0, 0.15);
  color: var(--accent-orange);
}

.stat-icon.positions {
  background: rgba(147, 51, 234, 0.15);
  color: #a855f7;
}

.stat-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.stat-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-primary);
}

.stat-value.positive {
  color: var(--accent-green);
}

.stat-value.negative {
  color: var(--accent-red);
}

.content-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  margin-bottom: 32px;
}

.panel {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  overflow: hidden;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.panel-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.panel-badge {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: var(--accent-green);
  background: rgba(0, 255, 136, 0.1);
  padding: 4px 10px;
  border-radius: 20px;
}

.panel-badge::before {
  content: '';
  width: 6px;
  height: 6px;
  background: var(--accent-green);
  border-radius: 50%;
  animation: blink 1.5s infinite;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

.panel-count {
  font-size: 12px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-secondary);
  background: var(--bg-tertiary);
  padding: 4px 10px;
  border-radius: 6px;
}

.signal-list {
  padding: 8px 0;
}

.signal-item {
  display: grid;
  grid-template-columns: 80px 1fr 100px 70px;
  align-items: center;
  padding: 12px 20px;
  transition: background 0.15s ease;
}

.signal-item:hover {
  background: rgba(0, 217, 255, 0.03);
}

.signal-time {
  font-size: 12px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-tertiary);
}

.signal-strategy {
  font-size: 13px;
  color: var(--text-secondary);
}

.signal-symbol {
  font-size: 13px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-primary);
}

.signal-action {
  font-size: 11px;
  font-weight: 600;
  padding: 4px 10px;
  border-radius: 4px;
  text-align: center;
}

.signal-action.buy {
  background: rgba(0, 255, 136, 0.15);
  color: var(--accent-green);
}

.signal-action.sell {
  background: rgba(255, 71, 87, 0.15);
  color: var(--accent-red);
}

.signal-action.close {
  background: rgba(125, 133, 144, 0.15);
  color: var(--text-secondary);
}

.positions-table {
  width: 100%;
  border-collapse: collapse;
}

.positions-table th {
  text-align: left;
  padding: 12px 20px;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
}

.positions-table td {
  padding: 14px 20px;
  border-bottom: 1px solid var(--border-color);
  font-size: 13px;
}

.symbol-cell {
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.mono {
  font-family: 'JetBrains Mono', monospace;
}

.exchange-tag {
  display: inline-block;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.exchange-tag.binance {
  background: rgba(0, 217, 255, 0.15);
  color: var(--accent-cyan);
}

.exchange-tag.okx {
  background: rgba(255, 149, 0, 0.15);
  color: var(--accent-orange);
}

.side-tag {
  display: inline-block;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
}

.side-tag.long {
  background: rgba(0, 255, 136, 0.15);
  color: var(--accent-green);
}

.side-tag.short {
  background: rgba(255, 71, 87, 0.15);
  color: var(--accent-red);
}

.pnl-cell {
  font-family: 'JetBrains Mono', monospace;
  font-weight: 600;
}

.pnl-cell.positive {
  color: var(--accent-green);
}

.pnl-cell.negative {
  color: var(--accent-red);
}

.arbitrage-flow {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  overflow: hidden;
}

.flow-header {
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.flow-diagram {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px;
  gap: 24px;
}

.flow-node {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 20px 32px;
  border-radius: 16px;
  border: 1px solid;
}

.flow-node.binance {
  background: rgba(0, 217, 255, 0.08);
  border-color: rgba(0, 217, 255, 0.3);
}

.flow-node.okx {
  background: rgba(255, 149, 0, 0.08);
  border-color: rgba(255, 149, 0, 0.3);
}

.node-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
  font-weight: 700;
  font-family: 'JetBrains Mono', monospace;
}

.binance .node-icon {
  background: rgba(0, 217, 255, 0.2);
  color: var(--accent-cyan);
}

.okx .node-icon {
  background: rgba(255, 149, 0, 0.2);
  color: var(--accent-orange);
}

.node-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.node-price {
  font-size: 16px;
  font-weight: 700;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-primary);
}

.flow-arrow {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  color: var(--accent-green);
}

.flow-arrow svg {
  width: 100px;
  height: 24px;
}

.spread {
  font-size: 14px;
  font-weight: 700;
  font-family: 'JetBrains Mono', monospace;
}
</style>
