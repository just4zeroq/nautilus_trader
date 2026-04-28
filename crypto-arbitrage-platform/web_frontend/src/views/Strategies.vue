<template>
  <div class="strategies-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Trading Strategies</h1>
        <p class="page-subtitle">Configure and manage arbitrage strategies</p>
      </div>
      <div class="header-controls">
        <button class="refresh-btn" @click="refreshStrategies" :class="{ loading: store.loading }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
            <path d="M21 3v5h-5"/>
          </svg>
        </button>
      </div>
    </div>

    <div class="strategies-grid">
      <div
        v-for="strategy in store.strategies"
        :key="strategy.id"
        class="strategy-card"
        :class="{ active: strategy.enabled }"
      >
        <div class="card-header-section">
          <div class="strategy-icon" :class="strategy.strategy_type">
            <svg v-if="strategy.strategy_type === 'cross_exchange'" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M8 3v3a2 2 0 0 1-2 2H3"/>
              <path d="M21 8h-3a2 2 0 0 1-2-2V3"/>
              <path d="M3 16h3a2 2 0 0 1 2 2v3"/>
              <path d="M16 21v-3a2 2 0 0 1 2-2h3"/>
            </svg>
            <svg v-else-if="strategy.strategy_type === 'triangular'" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 2L2 19h20L12 2z"/>
            </svg>
            <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M3 3v18h18"/>
              <path d="M18 17l-5-5-3 3-4-4"/>
            </svg>
          </div>
          <div class="status-indicator" :class="strategy.enabled ? 'running' : 'stopped'">
            <span class="status-dot"></span>
            <span class="status-text">{{ strategy.enabled ? 'Running' : 'Stopped' }}</span>
          </div>
        </div>

        <div class="card-body">
          <h3 class="strategy-name">{{ strategy.name }}</h3>
          <div class="strategy-type">{{ formatType(strategy.strategy_type) }}</div>
        </div>

        <div class="card-params">
          <div class="param" v-for="(value, key) in getDisplayParams(strategy.params)" :key="key">
            <span class="param-label">{{ key }}</span>
            <span class="param-value">{{ value }}</span>
          </div>
        </div>

        <div class="card-actions">
          <button
            class="action-btn"
            :class="strategy.enabled ? 'stop' : 'start'"
            @click="toggleStrategy(strategy)"
          >
            <svg v-if="!strategy.enabled" viewBox="0 0 24 24" fill="currentColor">
              <path d="M8 5v14l11-7z"/>
            </svg>
            <svg v-else viewBox="0 0 24 24" fill="currentColor">
              <rect x="6" y="4" width="4" height="16"/>
              <rect x="14" y="4" width="4" height="16"/>
            </svg>
            {{ strategy.enabled ? 'Stop' : 'Start' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useStrategyStore } from '@/stores/strategies'
import { ElMessage } from 'element-plus'

interface Strategy {
  id: string
  name: string
  strategy_type: string
  enabled: boolean
  params: Record<string, unknown>
}

const store = useStrategyStore()

onMounted(() => {
  store.fetchStrategies()
})

async function refreshStrategies() {
  await store.fetchStrategies()
}

async function toggleStrategy(strategy: Strategy) {
  try {
    if (strategy.enabled) {
      await store.stopStrategy(strategy.id)
      ElMessage.success(`${strategy.name} stopped`)
    } else {
      await store.startStrategy(strategy.id)
      ElMessage.success(`${strategy.name} started`)
    }
  } catch {
    ElMessage.error(`Failed to ${strategy.enabled ? 'stop' : 'start'} strategy`)
  }
}

function formatType(type: string): string {
  switch (type) {
    case 'cross_exchange': return 'Cross Exchange'
    case 'triangular': return 'Triangular'
    case 'statistical': return 'Statistical'
    default: return type
  }
}

function getDisplayParams(params: Record<string, unknown>): Record<string, string> {
  const display: Record<string, string> = {}
  for (const [key, value] of Object.entries(params)) {
    if (typeof value === 'number') {
      display[key] = value.toLocaleString()
    } else {
      display[key] = String(value)
    }
  }
  return display
}
</script>

<style scoped>
.strategies-page {
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

.strategies-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 20px;
}

.strategy-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 20px;
  transition: all 0.2s ease;
}

.strategy-card:hover {
  border-color: rgba(0, 217, 255, 0.3);
  transform: translateY(-2px);
}

.strategy-card.active {
  border-color: rgba(0, 255, 136, 0.3);
}

.card-header-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.strategy-icon {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.strategy-icon svg {
  width: 22px;
  height: 22px;
}

.strategy-icon.cross_exchange {
  background: rgba(0, 217, 255, 0.15);
  color: var(--accent-cyan);
}

.strategy-icon.triangular {
  background: rgba(255, 149, 0, 0.15);
  color: var(--accent-orange);
}

.strategy-icon.statistical {
  background: rgba(147, 51, 234, 0.15);
  color: #a855f7;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
}

.status-indicator.running {
  background: rgba(0, 255, 136, 0.12);
  color: var(--accent-green);
}

.status-indicator.stopped {
  background: rgba(125, 133, 144, 0.12);
  color: var(--text-tertiary);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.running .status-dot {
  background: var(--accent-green);
  box-shadow: 0 0 8px var(--accent-green);
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

.card-body {
  margin-bottom: 16px;
}

.strategy-name {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 4px 0;
}

.strategy-type {
  font-size: 13px;
  color: var(--text-secondary);
}

.card-params {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 10px;
  padding: 16px;
  background: var(--bg-tertiary);
  border-radius: 10px;
  margin-bottom: 16px;
}

.param {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.param-label {
  font-size: 11px;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.param-value {
  font-size: 14px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-primary);
}

.card-actions {
  display: flex;
  gap: 10px;
}

.action-btn {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 16px;
  border-radius: 10px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
  font-family: inherit;
}

.action-btn svg {
  width: 16px;
  height: 16px;
}

.action-btn.start {
  background: rgba(0, 255, 136, 0.15);
  color: var(--accent-green);
}

.action-btn.start:hover {
  background: rgba(0, 255, 136, 0.25);
}

.action-btn.stop {
  background: rgba(255, 71, 87, 0.15);
  color: var(--accent-red);
}

.action-btn.stop:hover {
  background: rgba(255, 71, 87, 0.25);
}
</style>
