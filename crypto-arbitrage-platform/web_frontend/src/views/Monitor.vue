<template>
  <div class="monitor-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">System Monitor</h1>
        <p class="page-subtitle">Real-time node status and performance</p>
      </div>
      <div class="header-controls">
        <button class="refresh-btn" @click="store.fetchNodes()" :class="{ loading: store.loading }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
            <path d="M21 3v5h-5"/>
          </svg>
        </button>
      </div>
    </div>

    <div class="type-tabs">
      <button
        v-for="t in types"
        :key="t.key"
        class="tab-btn"
        :class="{ active: filterType === t.key }"
        @click="filterType = t.key"
      >
        <span class="tab-indicator" :class="t.key !== 'all' ? t.key : ''"></span>
        {{ t.label }}
      </button>
    </div>

    <div class="stats-bar">
      <div class="stat-item">
        <span class="stat-value">{{ store.nodes.length }}</span>
        <span class="stat-label">Total Nodes</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat-item">
        <span class="stat-value">{{ onlineCount }}</span>
        <span class="stat-label">Online</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat-item">
        <span class="stat-value">{{ offlineCount }}</span>
        <span class="stat-label">Offline</span>
      </div>
    </div>

    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th class="col-name">Name</th>
            <th class="col-type">Type</th>
            <th class="col-exchange">Exchange</th>
            <th class="col-status">Status</th>
            <th class="col-latency">Latency</th>
            <th class="col-heartbeat">Heartbeat</th>
            <th class="col-version">Version</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="node in filteredNodes"
            :key="node.id"
            class="table-row"
          >
            <td class="col-name">
              <span class="node-name">{{ node.name }}</span>
            </td>
            <td class="col-type">
              <span class="type-badge" :class="node.node_type">
                {{ node.node_type }}
              </span>
            </td>
            <td class="col-exchange">
              <span v-if="node.exchange" class="exchange-badge" :class="node.exchange">
                {{ node.exchange === 'binance' ? 'BN' : node.exchange === 'okx' ? 'OKX' : node.exchange }}
              </span>
              <span v-else class="null-value">-</span>
            </td>
            <td class="col-status">
              <div class="status-indicator" :class="node.status === 'online' ? 'active' : 'inactive'">
                <span class="status-dot"></span>
                <span class="status-text">{{ node.status === 'online' ? 'Online' : 'Offline' }}</span>
              </div>
            </td>
            <td class="col-latency">
              <span class="mono-value">{{ node.latency !== null ? node.latency + 'ms' : '-' }}</span>
            </td>
            <td class="col-heartbeat">
              <span class="mono-value">{{ node.last_heartbeat ? formatTime(node.last_heartbeat) : '-' }}</span>
            </td>
            <td class="col-version">
              <span class="version-text">{{ node.version || '-' }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="table-footer">
      <span class="footer-info">Showing {{ filteredNodes.length }} of {{ store.nodes.length }} nodes</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useNodeStore } from '@/stores/nodes'

const store = useNodeStore()
const filterType = ref('all')

const types = [
  { key: 'all', label: 'All' },
  { key: 'collector', label: 'Collector' },
  { key: 'trader', label: 'Trader' },
  { key: 'alert', label: 'Alert' },
  { key: 'redis', label: 'Redis' }
]

const filteredNodes = computed(() =>
  filterType.value === 'all'
    ? store.nodes
    : store.nodes.filter(n => n.node_type === filterType.value)
)

const onlineCount = computed(() =>
  store.nodes.filter(n => n.status === 'online').length
)

const offlineCount = computed(() =>
  store.nodes.filter(n => n.status !== 'online').length
)

function formatTime(ts: string | null): string {
  if (!ts) return '-'
  return new Date(ts).toLocaleTimeString('en-US', { hour12: false })
}

onMounted(() => {
  store.fetchNodes()
})
</script>

<style scoped>
.monitor-page {
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

.tab-indicator.collector {
  background: var(--accent-cyan);
}

.tab-indicator.trader {
  background: var(--accent-green);
}

.tab-indicator.alert {
  background: var(--accent-orange);
}

.tab-indicator.redis {
  background: var(--accent-red);
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

.node-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  font-family: 'JetBrains Mono', monospace;
}

.type-badge {
  display: inline-block;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.type-badge.collector {
  background: rgba(0, 217, 255, 0.12);
  color: var(--accent-cyan);
}

.type-badge.trader {
  background: rgba(0, 255, 136, 0.12);
  color: var(--accent-green);
}

.type-badge.alert {
  background: rgba(255, 149, 0, 0.12);
  color: var(--accent-orange);
}

.type-badge.redis {
  background: rgba(255, 71, 87, 0.12);
  color: var(--accent-red);
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

.null-value {
  font-size: 13px;
  color: var(--text-tertiary);
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

.mono-value {
  font-size: 13px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-secondary);
}

.version-text {
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
}

.table-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
}

.footer-info {
  font-size: 12px;
  color: var(--text-tertiary);
}

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
