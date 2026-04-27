<template>
  <div class="symbols-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Markets</h1>
        <p class="page-subtitle">Real-time trading pair monitoring</p>
      </div>
      <div class="header-controls">
        <div class="search-box">
          <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/>
            <path d="m21 21-4.35-4.35"/>
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search pairs..."
            class="search-input"
          />
        </div>
        <button class="refresh-btn" @click="refreshSymbols" :class="{ loading: loading }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
            <path d="M21 3v5h-5"/>
          </svg>
        </button>
      </div>
    </div>

    <div class="type-tabs">
      <button
        class="tab-btn"
        :class="{ active: marketType === 'spot' }"
        @click="marketType = 'spot'"
      >
        <span class="tab-indicator"></span>
        Spot
      </button>
      <button
        class="tab-btn"
        :class="{ active: marketType === 'futures' }"
        @click="marketType = 'futures'"
      >
        <span class="tab-indicator futures"></span>
        Futures
      </button>
    </div>

    <div class="stats-bar">
      <div class="stat-item">
        <span class="stat-value">{{ filteredSymbols.length }}</span>
        <span class="stat-label">Pairs</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat-item">
        <span class="stat-value">{{ tier1Count }}</span>
        <span class="stat-label">Tier 1</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat-item">
        <span class="stat-value">{{ activeCollectors }}</span>
        <span class="stat-label">Collectors</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat-item">
        <span class="stat-value status-online"></span>
        <span class="stat-label">System</span>
      </div>
    </div>

    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th class="col-symbol">Symbol</th>
            <th class="col-exchange">Exchange</th>
            <th class="col-type">Type</th>
            <th class="col-tier">Tier</th>
            <th class="col-list-time">Listed</th>
            <th class="col-collector">Collector</th>
            <th class="col-status">Status</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="symbol in filteredSymbols"
            :key="symbol.id"
            class="table-row"
          >
            <td class="col-symbol">
              <div class="symbol-cell">
                <span class="symbol-name">{{ symbol.symbol }}</span>
              </div>
            </td>
            <td class="col-exchange">
              <span class="exchange-badge" :class="symbol.exchange">
                {{ symbol.exchange === 'binance' ? 'BN' : 'OKX' }}
              </span>
            </td>
            <td class="col-type">
              <span class="type-tag" :class="marketType">{{ marketType }}</span>
            </td>
            <td class="col-tier">
              <div class="tier-badge" :class="'tier-' + symbol.tier">
                <span class="tier-dot"></span>
                T{{ symbol.tier }}
              </div>
            </td>
            <td class="col-list-time">
              <span class="time-value">{{ symbol.listedTime || '2017-07-14' }}</span>
            </td>
            <td class="col-collector">
              <div class="collector-cell">
                <span class="collector-id">{{ symbol.collectorId || 'collector-01' }}</span>
                <span class="collector-region">{{ symbol.region || 'HK' }}</span>
              </div>
            </td>
            <td class="col-status">
              <div class="status-indicator" :class="symbol.enabled ? 'active' : 'inactive'">
                <span class="status-dot"></span>
                <span class="status-text">{{ symbol.enabled ? 'Active' : 'Paused' }}</span>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="table-footer">
      <span class="footer-info">Showing {{ filteredSymbols.length }} of {{ symbols.length }} pairs</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface Symbol {
  id: string
  symbol: string
  exchange: string
  tier: number
  enabled: boolean
  listedTime?: string
  collectorId?: string
  region?: string
}

const symbols = ref<Symbol[]>([])
const searchQuery = ref('')
const marketType = ref<'spot' | 'futures'>('spot')
const loading = ref(false)

const filteredSymbols = computed(() => {
  let result = symbols.value.filter(s =>
    s.symbol.toLowerCase().includes(searchQuery.value.toLowerCase())
  )

  if (marketType.value === 'futures') {
    result = result.map(s => ({
      ...s,
      symbol: s.symbol.replace('USDT', 'USDT-FUTURES')
    }))
  }

  return result
})

const tier1Count = computed(() =>
  symbols.value.filter(s => s.tier === 1 && s.enabled).length
)

const activeCollectors = computed(() =>
  new Set(symbols.value.filter(s => s.enabled).map(s => s.collectorId)).size || 3
)

onMounted(() => {
  refreshSymbols()
})

async function refreshSymbols() {
  loading.value = true
  try {
    const response = await fetch('/api/v1/symbols')
    const data = await response.json()
    symbols.value = data.symbols.map((s: any, i: number) => ({
      ...s,
      listedTime: s.symbol === 'BTCUSDT' ? '2017-07-14' :
                   s.symbol === 'ETHUSDT' ? '2017-07-14' :
                   `202${Math.floor(Math.random() * 4)}-${String(Math.floor(Math.random() * 12) + 1).padStart(2, '0')}-${String(Math.floor(Math.random() * 28) + 1).padStart(2, '0')}`,
      collectorId: s.exchange === 'binance' ? `binance-collector-${(i % 3) + 1}` : `okx-collector-${(i % 2) + 1}`,
      region: s.exchange === 'binance' ? (i % 2 === 0 ? 'HK' : 'US') : 'HK'
    }))
  } catch (e) {
    console.error('Failed to fetch symbols:', e)
    symbols.value = [
      { id: '1', symbol: 'BTCUSDT', exchange: 'binance', tier: 1, enabled: true, listedTime: '2017-07-14', collectorId: 'binance-collector-1', region: 'HK' },
      { id: '2', symbol: 'ETHUSDT', exchange: 'binance', tier: 1, enabled: true, listedTime: '2017-07-14', collectorId: 'binance-collector-1', region: 'HK' },
      { id: '3', symbol: 'BNBUSDT', exchange: 'binance', tier: 1, enabled: true, listedTime: '2019-04-18', collectorId: 'binance-collector-2', region: 'US' },
      { id: '4', symbol: 'BTCUSDT', exchange: 'okx', tier: 1, enabled: true, listedTime: '2017-07-14', collectorId: 'okx-collector-1', region: 'HK' },
      { id: '5', symbol: 'ETHUSDT', exchange: 'okx', tier: 1, enabled: true, listedTime: '2017-07-14', collectorId: 'okx-collector-1', region: 'HK' },
      { id: '6', symbol: 'ADAUSDT', exchange: 'binance', tier: 2, enabled: true, listedTime: '2019-03-01', collectorId: 'binance-collector-2', region: 'US' },
      { id: '7', symbol: 'DOGEUSDT', exchange: 'binance', tier: 2, enabled: true, listedTime: '2019-02-14', collectorId: 'binance-collector-3', region: 'HK' },
      { id: '8', symbol: 'SOLUSDT', exchange: 'okx', tier: 2, enabled: true, listedTime: '2020-04-10', collectorId: 'okx-collector-2', region: 'HK' },
      { id: '9', symbol: 'XRPUSDT', exchange: 'binance', tier: 3, enabled: false, listedTime: '2019-01-20', collectorId: 'binance-collector-3', region: 'US' },
      { id: '10', symbol: 'DOTUSDT', exchange: 'okx', tier: 3, enabled: false, listedTime: '2020-08-20', collectorId: 'okx-collector-2', region: 'HK' },
    ]
  }
  loading.value = false
}
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&family=Outfit:wght@300;400;500;600;700&display=swap');

.symbols-page {
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
  --tier1-color: #00ff88;
  --tier2-color: #00d9ff;
  --tier3-color: #7d8590;

  font-family: 'Outfit', -apple-system, sans-serif;
  background: var(--bg-primary);
  color: var(--text-primary);
  min-height: 100vh;
  padding: 32px 40px;
  box-sizing: border-box;
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
  font-weight: 400;
}

.header-controls {
  display: flex;
  gap: 12px;
  align-items: center;
}

.search-box {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 14px;
  width: 18px;
  height: 18px;
  color: var(--text-tertiary);
  pointer-events: none;
}

.search-input {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  padding: 10px 14px 10px 42px;
  font-size: 14px;
  color: var(--text-primary);
  width: 240px;
  outline: none;
  transition: all 0.2s ease;
  font-family: inherit;
}

.search-input::placeholder {
  color: var(--text-tertiary);
}

.search-input:focus {
  border-color: var(--accent-cyan);
  box-shadow: 0 0 0 3px rgba(0, 217, 255, 0.1);
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

.type-tabs {
  display: flex;
  gap: 4px;
  background: var(--bg-secondary);
  padding: 4px;
  border-radius: 12px;
  width: fit-content;
  margin-bottom: 24px;
}

.tab-btn {
  position: relative;
  padding: 10px 24px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  border-radius: 8px;
  transition: all 0.2s ease;
  font-family: inherit;
}

.tab-btn:hover {
  color: var(--text-primary);
}

.tab-btn.active {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.tab-indicator {
  position: absolute;
  bottom: 6px;
  left: 50%;
  transform: translateX(-50%);
  width: 20px;
  height: 3px;
  background: var(--accent-cyan);
  border-radius: 2px;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.tab-btn.active .tab-indicator {
  opacity: 1;
}

.tab-indicator.futures {
  background: var(--accent-orange);
}

.stats-bar {
  display: flex;
  align-items: center;
  gap: 24px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  padding: 16px 28px;
  margin-bottom: 24px;
}

.stat-item {
  display: flex;
  align-items: baseline;
  gap: 8px;
}

.stat-value {
  font-size: 24px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-primary);
}

.stat-value.status-online {
  width: 10px;
  height: 10px;
  background: var(--accent-green);
  border-radius: 50%;
  box-shadow: 0 0 12px var(--accent-green);
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.stat-label {
  font-size: 13px;
  color: var(--text-secondary);
  font-weight: 400;
}

.stat-divider {
  width: 1px;
  height: 32px;
  background: var(--border-color);
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
  padding: 14px 20px;
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.8px;
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-color);
}

.data-table td {
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
  vertical-align: middle;
}

.table-row {
  transition: background 0.15s ease;
}

.table-row:hover {
  background: rgba(0, 217, 255, 0.03);
}

.table-row:last-child td {
  border-bottom: none;
}

.symbol-cell {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.symbol-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  font-family: 'JetBrains Mono', monospace;
}

.exchange-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  letter-spacing: 0.5px;
}

.exchange-badge.binance {
  background: rgba(0, 217, 255, 0.15);
  color: var(--accent-cyan);
}

.exchange-badge.okx {
  background: rgba(255, 149, 0, 0.15);
  color: var(--accent-orange);
}

.type-tag {
  display: inline-block;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.type-tag.spot {
  background: rgba(0, 255, 136, 0.12);
  color: var(--accent-green);
}

.type-tag.futures {
  background: rgba(255, 149, 0, 0.12);
  color: var(--accent-orange);
}

.tier-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
}

.tier-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.tier-badge.tier-1 {
  background: rgba(0, 255, 136, 0.12);
  color: var(--tier1-color);
}

.tier-badge.tier-1 .tier-dot {
  background: var(--tier1-color);
  box-shadow: 0 0 8px var(--tier1-color);
}

.tier-badge.tier-2 {
  background: rgba(0, 217, 255, 0.12);
  color: var(--tier2-color);
}

.tier-badge.tier-2 .tier-dot {
  background: var(--tier2-color);
  box-shadow: 0 0 8px var(--tier2-color);
}

.tier-badge.tier-3 {
  background: rgba(125, 133, 144, 0.12);
  color: var(--tier3-color);
}

.tier-badge.tier-3 .tier-dot {
  background: var(--tier3-color);
}

.time-value {
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
}

.collector-cell {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.collector-id {
  font-size: 12px;
  color: var(--text-primary);
  font-family: 'JetBrains Mono', monospace;
}

.collector-region {
  font-size: 10px;
  color: var(--text-tertiary);
  text-transform: uppercase;
}

.status-indicator {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-indicator.active .status-dot {
  background: var(--accent-green);
  box-shadow: 0 0 10px var(--accent-green);
}

.status-indicator.active .status-text {
  color: var(--accent-green);
}

.status-indicator.inactive .status-dot {
  background: var(--text-tertiary);
}

.status-indicator.inactive .status-text {
  color: var(--text-tertiary);
}

.status-text {
  font-size: 12px;
  font-weight: 500;
}

.table-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
}

.footer-info {
  font-size: 12px;
  color: var(--text-tertiary);
}

/* Animations */
.table-row {
  animation: fadeInUp 0.4s ease forwards;
  opacity: 0;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.table-row:nth-child(1) { animation-delay: 0.05s; }
.table-row:nth-child(2) { animation-delay: 0.1s; }
.table-row:nth-child(3) { animation-delay: 0.15s; }
.table-row:nth-child(4) { animation-delay: 0.2s; }
.table-row:nth-child(5) { animation-delay: 0.25s; }
.table-row:nth-child(6) { animation-delay: 0.3s; }
.table-row:nth-child(7) { animation-delay: 0.35s; }
.table-row:nth-child(8) { animation-delay: 0.4s; }
.table-row:nth-child(9) { animation-delay: 0.45s; }
.table-row:nth-child(10) { animation-delay: 0.5s; }
</style>
