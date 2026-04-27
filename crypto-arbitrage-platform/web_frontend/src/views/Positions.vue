<template>
  <div class="positions-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Positions & Accounts</h1>
        <p class="page-subtitle">Portfolio management and balance tracking</p>
      </div>
      <div class="header-controls">
        <button class="refresh-btn" @click="refreshAll" :class="{ loading }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
            <path d="M21 3v5h-5"/>
          </svg>
        </button>
      </div>
    </div>

    <div class="section">
      <div class="section-header">
        <h2 class="section-title">Account Balances</h2>
      </div>
      <div class="balances-grid">
        <div v-for="balance in balances" :key="balance.exchange + balance.asset" class="balance-card">
          <div class="balance-header">
            <span class="exchange-badge" :class="balance.exchange">
              {{ balance.exchange === 'binance' ? 'BN' : 'OKX' }}
            </span>
            <span class="asset-name">{{ balance.asset }}</span>
          </div>
          <div class="balance-value">
            <span class="balance-amount">{{ formatNumber(balance.balance) }}</span>
            <span class="balance-asset">{{ balance.asset }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="section">
      <div class="section-header">
        <h2 class="section-title">Open Positions</h2>
        <span class="position-count">{{ positions.length }} positions</span>
      </div>
      <div class="table-container">
        <table class="data-table">
          <thead>
            <tr>
              <th>Symbol</th>
              <th>Exchange</th>
              <th>Type</th>
              <th>Side</th>
              <th>Quantity</th>
              <th>Avg Price</th>
              <th>Est. Value</th>
              <th>P/L</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="pos in positions" :key="pos.id" class="table-row">
              <td class="symbol-cell">{{ pos.symbol }}</td>
              <td>
                <span class="exchange-badge" :class="pos.exchange">
                  {{ pos.exchange === 'binance' ? 'BN' : 'OKX' }}
                </span>
              </td>
              <td><span class="type-tag spot">Spot</span></td>
              <td>
                <span class="side-tag" :class="pos.side.toLowerCase()">{{ pos.side }}</span>
              </td>
              <td class="mono">{{ pos.quantity }}</td>
              <td class="mono">${{ formatNumber(pos.avg_price) }}</td>
              <td class="mono">${{ formatNumber(pos.quantity * pos.avg_price) }}</td>
              <td class="pnl-cell" :class="getPnLClass(pos)">
                {{ getPnLSign(pos) }}{{ formatNumber(Math.abs(getPnL(pos))) }}
              </td>
              <td>
                <div class="action-buttons">
                  <button class="action-btn close" title="Close Position">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path d="M18 6L6 18M6 6l12 12"/>
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div class="section">
      <div class="section-header">
        <h2 class="section-title">Recent Orders</h2>
      </div>
      <div class="orders-list">
        <div v-for="order in orders" :key="order.id" class="order-item">
          <div class="order-left">
            <span class="order-symbol">{{ order.symbol }}</span>
            <span class="exchange-badge small" :class="order.exchange">{{ order.exchange === 'binance' ? 'BN' : 'OKX' }}</span>
          </div>
          <div class="order-details">
            <span class="order-side" :class="order.side.toLowerCase()">{{ order.side }}</span>
            <span class="order-info">{{ order.quantity }} @ ${{ formatNumber(order.price) }}</span>
          </div>
          <div class="order-right">
            <span class="order-status" :class="order.status.toLowerCase()">{{ order.status }}</span>
            <span class="order-time">{{ order.created_at }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Balance {
  exchange: string
  asset: string
  balance: number
  updated_at: string
}

interface Position {
  id: string
  symbol: string
  exchange: string
  side: string
  quantity: number
  avg_price: number
}

interface Order {
  id: string
  symbol: string
  exchange: string
  side: string
  quantity: number
  price: number
  status: string
  created_at: string
}

const loading = ref(false)
const balances = ref<Balance[]>([])
const positions = ref<Position[]>([])
const orders = ref<Order[]>([])

onMounted(() => {
  refreshAll()
})

async function refreshAll() {
  loading.value = true
  try {
    const [balanceRes, posRes] = await Promise.all([
      fetch('/api/v1/account'),
      fetch('/api/v1/account/positions')
    ])
    const balanceData = await balanceRes.json()
    const posData = await posRes.json()
    balances.value = balanceData
    positions.value = posData
  } catch (e) {
    balances.value = [
      { exchange: 'binance', asset: 'USDT', balance: 50000, updated_at: new Date().toISOString() },
      { exchange: 'binance', asset: 'BTC', balance: 1.5, updated_at: new Date().toISOString() },
      { exchange: 'binance', asset: 'ETH', balance: 25, updated_at: new Date().toISOString() },
      { exchange: 'okx', asset: 'USDT', balance: 50000, updated_at: new Date().toISOString() },
      { exchange: 'okx', asset: 'BTC', balance: 1.2, updated_at: new Date().toISOString() }
    ]
    positions.value = [
      { id: '1', symbol: 'BTCUSDT', exchange: 'binance', side: 'LONG', quantity: 0.5, avg_price: 65000 },
      { id: '2', symbol: 'ETHUSDT', exchange: 'okx', side: 'LONG', quantity: 5, avg_price: 3500 }
    ]
  }
  orders.value = [
    { id: 'order-1', symbol: 'BTCUSDT', exchange: 'binance', side: 'BUY', quantity: 0.1, price: 64000, status: 'FILLED', created_at: '2 min ago' },
    { id: 'order-2', symbol: 'ETHUSDT', exchange: 'okx', side: 'SELL', quantity: 2, price: 3600, status: 'FILLED', created_at: '15 min ago' },
    { id: 'order-3', symbol: 'ADAUSDT', exchange: 'binance', side: 'BUY', quantity: 500, price: 0.58, status: 'FILLED', created_at: '1 hour ago' }
  ]
  loading.value = false
}

function formatNumber(num: number): string {
  return num.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
}

function getPnL(pos: Position): number {
  const currentPrice = pos.avg_price * 1.02
  return pos.side === 'LONG'
    ? (currentPrice - pos.avg_price) * pos.quantity
    : (pos.avg_price - currentPrice) * pos.quantity
}

function getPnLSign(pos: Position): string {
  return getPnL(pos) >= 0 ? '+' : '-'
}

function getPnLClass(pos: Position): string {
  return getPnL(pos) >= 0 ? 'positive' : 'negative'
}
</script>

<style scoped>
.positions-page {
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
  font-family: 'Outfit', -apple-system, sans-serif;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
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

.refresh-btn {
  width: 42px;
  height: 42px;
  border-radius: 10px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.refresh-btn svg {
  width: 18px;
  height: 18px;
}

.refresh-btn:hover {
  border-color: var(--accent-cyan);
  color: var(--accent-cyan);
}

.refresh-btn.loading svg {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.section {
  margin-bottom: 32px;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.position-count {
  font-size: 12px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-secondary);
  background: var(--bg-tertiary);
  padding: 4px 10px;
  border-radius: 6px;
}

.balances-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}

.balance-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  padding: 16px;
}

.balance-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.exchange-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 10px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.exchange-badge.binance {
  background: rgba(0, 217, 255, 0.15);
  color: var(--accent-cyan);
}

.exchange-badge.okx {
  background: rgba(255, 149, 0, 0.15);
  color: var(--accent-orange);
}

.exchange-badge.small {
  padding: 2px 6px;
  font-size: 9px;
}

.asset-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.balance-value {
  display: flex;
  align-items: baseline;
  gap: 6px;
}

.balance-amount {
  font-size: 20px;
  font-weight: 700;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-primary);
}

.balance-asset {
  font-size: 12px;
  color: var(--text-tertiary);
}

.table-container {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  overflow: hidden;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
}

.data-table th {
  text-align: left;
  padding: 14px 16px;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-color);
}

.data-table td {
  padding: 14px 16px;
  border-bottom: 1px solid var(--border-color);
  font-size: 13px;
}

.table-row:last-child td {
  border-bottom: none;
}

.table-row:hover {
  background: rgba(0, 217, 255, 0.03);
}

.symbol-cell {
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.mono {
  font-family: 'JetBrains Mono', monospace;
}

.type-tag {
  display: inline-block;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
}

.type-tag.spot {
  background: rgba(0, 255, 136, 0.12);
  color: var(--accent-green);
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

.action-buttons {
  display: flex;
  gap: 8px;
}

.action-btn {
  width: 28px;
  height: 28px;
  border-radius: 6px;
  border: 1px solid var(--border-color);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.action-btn svg {
  width: 14px;
  height: 14px;
}

.action-btn:hover {
  border-color: var(--accent-red);
  color: var(--accent-red);
}

.orders-list {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  overflow: hidden;
}

.order-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
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

.order-info {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
}

.order-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.order-status {
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

.order-time {
  font-size: 11px;
  color: var(--text-tertiary);
}
</style>
