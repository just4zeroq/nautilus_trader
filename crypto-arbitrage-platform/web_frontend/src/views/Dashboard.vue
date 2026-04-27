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
          <div v-for="signal in recentSignals" :key="signal.id" class="signal-item">
            <div class="signal-time">{{ signal.time }}</div>
            <div class="signal-source">
              <span class="strategy-badge" :class="signal.strategyType">{{ signal.strategyName }}</span>
            </div>
            <div class="signal-symbol">{{ signal.symbol }}</div>
            <div class="signal-action" :class="signal.signal.toLowerCase()">{{ signal.signal }}</div>
          </div>
        </div>
      </div>

      <div class="panel">
        <div class="panel-header">
          <h2 class="panel-title">Real-time Orders</h2>
          <span class="panel-badge pulse">Live</span>
        </div>
        <div class="order-list">
          <div v-for="order in recentOrders" :key="order.id" class="order-item">
            <div class="order-left">
              <span class="order-symbol">{{ order.symbol }}</span>
              <span class="exchange-tag small" :class="order.exchange">{{ order.exchange === 'binance' ? 'BN' : 'OKX' }}</span>
            </div>
            <div class="order-details">
              <span class="order-side" :class="order.side.toLowerCase()">{{ order.side }}</span>
              <span class="order-price">{{ order.quantity }} @ ${{ formatNumber(order.price) }}</span>
            </div>
            <div class="order-status" :class="order.status.toLowerCase()">
              <span class="status-dot"></span>
              {{ order.status }}
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="panel full-width">
      <div class="panel-header">
        <h2 class="panel-title">Active Positions</h2>
        <span class="panel-count">{{ positions.length }}</span>
      </div>
      <table class="positions-table">
        <thead>
          <tr>
            <th>Symbol</th>
            <th>Exchange</th>
            <th>Strategy</th>
            <th>Side</th>
            <th>Quantity</th>
            <th>Avg Price</th>
            <th>P/L</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="pos in positions" :key="pos.id">
            <td class="symbol-cell">{{ pos.symbol }}</td>
            <td>
              <span class="exchange-tag" :class="pos.exchange.toLowerCase()">
                {{ pos.exchange === 'binance' ? 'BN' : 'OKX' }}
              </span>
            </td>
            <td>
              <span class="strategy-badge small" :class="pos.strategyType">{{ pos.strategyName }}</span>
            </td>
            <td>
              <span class="side-tag" :class="pos.side.toLowerCase()">{{ pos.side }}</span>
            </td>
            <td class="mono">{{ pos.quantity }}</td>
            <td class="mono">${{ formatNumber(pos.avgPrice) }}</td>
            <td class="pnl-cell" :class="pos.pnl >= 0 ? 'positive' : 'negative'">
              {{ pos.pnl >= 0 ? '+' : '' }}{{ formatNumber(pos.pnl) }}
            </td>
          </tr>
        </tbody>
      </table>
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
import { ref, onMounted, onUnmounted } from 'vue'

interface Signal {
  id: string
  time: string
  strategyName: string
  strategyType: string
  symbol: string
  signal: string
}

interface Order {
  id: string
  symbol: string
  exchange: string
  side: string
  quantity: number
  price: number
  status: string
}

interface Position {
  id: string
  symbol: string
  exchange: string
  strategyName: string
  strategyType: string
  side: string
  quantity: number
  avgPrice: number
  pnl: number
}

const recentSignals = ref<Signal[]>([
  { id: '1', time: '10:30:00', strategyName: 'Cross Exchange', strategyType: 'cross_exchange', symbol: 'BTCUSDT', signal: 'BUY' },
  { id: '2', time: '10:28:00', strategyName: 'Triangular', strategyType: 'triangular', symbol: 'ETH/USDT', signal: 'SELL' },
  { id: '3', time: '10:25:00', strategyName: 'Statistical', strategyType: 'statistical', symbol: 'ADAUSDT', signal: 'CLOSE' },
  { id: '4', time: '10:20:00', strategyName: 'Cross Exchange', strategyType: 'cross_exchange', symbol: 'ETHUSDT', signal: 'BUY' }
])

const recentOrders = ref<Order[]>([
  { id: 'order-1', symbol: 'BTCUSDT', exchange: 'binance', side: 'BUY', quantity: 0.1, price: 65420.50, status: 'FILLED' },
  { id: 'order-2', symbol: 'BTCUSDT', exchange: 'okx', side: 'SELL', quantity: 0.1, price: 65425.70, status: 'FILLED' },
  { id: 'order-3', symbol: 'ETHUSDT', exchange: 'binance', side: 'BUY', quantity: 2.0, price: 3520.00, status: 'FILLED' },
  { id: 'order-4', symbol: 'ETHUSDT', exchange: 'okx', side: 'SELL', quantity: 2.0, price: 3522.50, status: 'PENDING' },
  { id: 'order-5', symbol: 'ADAUSDT', exchange: 'binance', side: 'BUY', quantity: 500, price: 0.5820, status: 'FILLED' }
])

const positions = ref<Position[]>([
  { id: '1', symbol: 'BTCUSDT', exchange: 'binance', strategyName: 'Cross Exchange', strategyType: 'cross_exchange', side: 'LONG', quantity: 0.5, avgPrice: 65000, pnl: 210.25 },
  { id: '2', symbol: 'ETHUSDT', exchange: 'okx', strategyName: 'Triangular', strategyType: 'triangular', side: 'LONG', quantity: 5.0, avgPrice: 3500, pnl: 112.50 },
  { id: '3', symbol: 'ADAUSDT', exchange: 'binance', strategyName: 'Statistical', strategyType: 'statistical', side: 'SHORT', quantity: 1000, avgPrice: 0.58, pnl: -15.00 }
])

let orderTimer: number

function formatNumber(num: number): string {
  return num.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
}

onMounted(() => {
  orderTimer = window.setInterval(() => {
    const symbols = ['BTCUSDT', 'ETHUSDT', 'SOLUSDT', 'XRPUSDT']
    const exchanges = ['binance', 'okx']
    const statuses = ['FILLED', 'PENDING', 'FILLED']
    const sides = ['BUY', 'SELL']

    const newOrder: Order = {
      id: 'order-' + Date.now(),
      symbol: symbols[Math.floor(Math.random() * symbols.length)],
      exchange: exchanges[Math.floor(Math.random() * exchanges.length)],
      side: sides[Math.floor(Math.random() * sides.length)],
      quantity: Math.random() * 2,
      price: Math.random() * 10000 + 1000,
      status: statuses[Math.floor(Math.random() * statuses.length)]
    }

    recentOrders.value = [newOrder, ...recentOrders.value.slice(0, 9)]
  }, 5000)
})

onUnmounted(() => {
  clearInterval(orderTimer)
})
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

.signal-source {
  display: flex;
  align-items: center;
}

.strategy-badge {
  display: inline-block;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.strategy-badge.cross_exchange {
  background: rgba(0, 217, 255, 0.15);
  color: var(--accent-cyan);
}

.strategy-badge.triangular {
  background: rgba(255, 149, 0, 0.15);
  color: var(--accent-orange);
}

.strategy-badge.statistical {
  background: rgba(147, 51, 234, 0.15);
  color: #a855f7;
}

.strategy-badge.small {
  padding: 2px 6px;
  font-size: 9px;
}

.signal-action.close {
  background: rgba(125, 133, 144, 0.15);
  color: var(--text-secondary);
}

.order-list {
  padding: 8px 0;
}

.order-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  border-bottom: 1px solid var(--border-color);
  transition: background 0.15s ease;
}

.order-item:last-child {
  border-bottom: none;
}

.order-item:hover {
  background: rgba(0, 217, 255, 0.03);
}

.order-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.order-symbol {
  font-size: 13px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-primary);
}

.exchange-tag.small {
  padding: 2px 6px;
  font-size: 9px;
}

.order-details {
  display: flex;
  align-items: center;
  gap: 12px;
}

.order-side {
  font-size: 11px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 4px;
}

.order-side.buy {
  background: rgba(0, 255, 136, 0.15);
  color: var(--accent-green);
}

.order-side.sell {
  background: rgba(255, 71, 87, 0.15);
  color: var(--accent-red);
}

.order-price {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
}

.order-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  font-weight: 500;
  padding: 2px 8px;
  border-radius: 4px;
}

.order-status.filled {
  background: rgba(0, 255, 136, 0.12);
  color: var(--accent-green);
}

.order-status.pending {
  background: rgba(255, 149, 0, 0.12);
  color: var(--accent-orange);
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.panel-badge.pulse::before {
  animation: blink 1s infinite;
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
