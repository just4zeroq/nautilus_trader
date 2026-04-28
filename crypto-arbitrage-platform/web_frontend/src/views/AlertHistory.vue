<template>
  <div class="history-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Alert History</h1>
        <p class="page-subtitle">View triggered alert events</p>
      </div>
      <div class="header-controls">
        <button class="refresh-btn" @click="store.fetchHistory()" :class="{ loading: store.loading }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
            <path d="M21 3v5h-5"/>
          </svg>
        </button>
      </div>
    </div>

    <div class="filters-bar">
      <div class="filter-group">
        <label class="filter-label">Level</label>
        <el-select v-model="levelFilter" class="filter-select" placeholder="All Levels">
          <el-option label="All Levels" value="all" />
          <el-option label="Info" value="info" />
          <el-option label="Warning" value="warn" />
          <el-option label="Critical" value="critical" />
        </el-select>
      </div>
      <div class="filter-group">
        <label class="filter-label">Status</label>
        <el-select v-model="statusFilter" class="filter-select" placeholder="All Statuses">
          <el-option label="All Statuses" value="all" />
          <el-option label="Sent" value="sent" />
          <el-option label="Failed" value="failed" />
          <el-option label="Pending" value="pending" />
        </el-select>
      </div>
      <div class="filter-group">
        <label class="filter-label">Date Range</label>
        <el-date-picker
          v-model="dateRange"
          type="daterange"
          range-separator="to"
          start-placeholder="Start"
          end-placeholder="End"
          class="filter-datepicker"
          value-format="YYYY-MM-DD"
        />
      </div>
    </div>

    <div class="stats-bar">
      <div class="stat-item">
        <span class="stat-value">{{ filteredHistory.length }}</span>
        <span class="stat-label">Events</span>
      </div>
    </div>

    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th>Time</th>
            <th>Rule</th>
            <th>Level</th>
            <th>Conditions</th>
            <th>Triggered Value</th>
            <th>Channels</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in filteredHistory" :key="item.id" class="table-row">
            <td><span class="mono-value">{{ formatTime(item.created_at) }}</span></td>
            <td><span class="rule-name">{{ item.rule_name }}</span></td>
            <td>
              <span class="level-badge" :class="item.level">{{ item.level }}</span>
            </td>
            <td><span class="mono-value">{{ formatObject(item.conditions) }}</span></td>
            <td><span class="mono-value">{{ formatObject(item.triggered_value) }}</span></td>
            <td><span class="config-text">{{ formatObject(item.channels) }}</span></td>
            <td>
              <span class="status-tag" :class="item.status">{{ item.status }}</span>
            </td>
          </tr>
          <tr v-if="filteredHistory.length === 0">
            <td colspan="7" class="empty-cell">
              <div class="empty-state">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="empty-icon">
                  <path d="M12 22c5.523 0 10-4.477 10-10S17.523 2 12 2 2 6.477 2 12s4.477 10 10 10z"/>
                  <path d="M12 6v6l4 2"/>
                </svg>
                <span class="empty-text">No alert history found</span>
                <span class="empty-hint">Alert events will appear here when triggered</span>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAlertStore } from '@/stores/alerts'

const store = useAlertStore()
const levelFilter = ref('all')
const statusFilter = ref('all')
const dateRange = ref<[string, string] | null>(null)

const filteredHistory = computed(() => {
  let items = store.history

  if (levelFilter.value !== 'all') {
    items = items.filter(i => i.level === levelFilter.value)
  }

  if (statusFilter.value !== 'all') {
    items = items.filter(i => i.status === statusFilter.value)
  }

  if (dateRange.value) {
    const [start, end] = dateRange.value
    const startMs = new Date(start).getTime()
    const endMs = new Date(end).getTime() + 86400000
    items = items.filter(i => {
      const t = new Date(i.created_at).getTime()
      return t >= startMs && t <= endMs
    })
  }

  return items
})

function formatTime(ts: string): string {
  return new Date(ts).toLocaleString('en-US', { hour12: false })
}

function formatObject(val: any): string {
  if (val === null || val === undefined) return '-'
  if (typeof val === 'object') {
    try { return JSON.stringify(val) } catch { return String(val) }
  }
  return String(val)
}

onMounted(() => {
  store.fetchHistory()
})
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&family=Outfit:wght@300;400;500;600;700&display=swap');

.history-page {
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
  --accent-purple: #a855f7;

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
  margin-bottom: 24px;
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

.filters-bar {
  display: flex;
  gap: 16px;
  align-items: flex-end;
  margin-bottom: 20px;
  padding: 16px 24px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.filter-label {
  font-size: 11px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-tertiary);
}

.filter-select {
  width: 160px;
}

.filter-select :deep(.el-input__wrapper) {
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: none;
  padding: 4px 12px;
}

.filter-select :deep(.el-input__wrapper:hover) {
  border-color: var(--accent-cyan);
}

.filter-select :deep(.el-input__inner) {
  font-family: 'Outfit', -apple-system, sans-serif;
  font-size: 13px;
  color: var(--text-primary);
}

.filter-select :deep(.el-input__suffix) {
  color: var(--text-tertiary);
}

.filter-datepicker {
  width: 260px;
}

.filter-datepicker :deep(.el-input__wrapper) {
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: none;
  padding: 4px 12px;
}

.filter-datepicker :deep(.el-input__wrapper:hover) {
  border-color: var(--accent-cyan);
}

.filter-datepicker :deep(.el-input__inner) {
  font-family: 'Outfit', -apple-system, sans-serif;
  font-size: 13px;
  color: var(--text-primary);
}

.filter-datepicker :deep(.el-input__icon) {
  color: var(--text-tertiary);
}

.stats-bar {
  display: flex;
  align-items: center;
  gap: 24px;
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

.rule-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.level-badge {
  display: inline-block;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.level-badge.info {
  background: rgba(0, 217, 255, 0.12);
  color: var(--accent-cyan);
}

.level-badge.warn {
  background: rgba(255, 149, 0, 0.12);
  color: var(--accent-orange);
}

.level-badge.critical {
  background: rgba(255, 71, 87, 0.12);
  color: var(--accent-red);
}

.status-tag {
  display: inline-block;
  padding: 4px 10px;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.status-tag.sent {
  background: rgba(0, 255, 136, 0.12);
  color: var(--accent-green);
}

.status-tag.failed {
  background: rgba(255, 71, 87, 0.12);
  color: var(--accent-red);
}

.status-tag.pending {
  background: rgba(255, 149, 0, 0.12);
  color: var(--accent-orange);
}

.mono-value {
  font-size: 13px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-secondary);
  max-width: 160px;
  display: inline-block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.config-text {
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
  max-width: 120px;
  display: inline-block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-cell {
  text-align: center;
  padding: 64px 20px !important;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.empty-icon {
  width: 48px;
  height: 48px;
  color: var(--text-tertiary);
  opacity: 0.5;
}

.empty-text {
  font-size: 16px;
  font-weight: 500;
  color: var(--text-secondary);
}

.empty-hint {
  font-size: 13px;
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
