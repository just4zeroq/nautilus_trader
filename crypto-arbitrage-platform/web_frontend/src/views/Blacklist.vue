<template>
  <div class="blacklist-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Blacklist</h1>
        <p class="page-subtitle">Manage restricted trading pairs</p>
      </div>
      <div class="header-controls">
        <div class="search-box">
          <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/>
            <path d="m21 21-4.35-4.35"/>
          </svg>
          <input
            v-model="exchangeFilter"
            type="text"
            placeholder="Filter by exchange..."
            class="search-input"
          />
        </div>
        <button class="primary-btn" @click="showAddDialog = true">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 5v14M5 12h14"/>
          </svg>
          Add to Blacklist
        </button>
      </div>
    </div>

    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th>Symbol</th>
            <th>Exchange</th>
            <th>Reason</th>
            <th>Created At</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="item in filteredItems"
            :key="item.id"
            class="table-row"
          >
            <td>
              <span class="symbol-name">{{ item.symbol }}</span>
            </td>
            <td>
              <span class="exchange-badge" :class="item.exchange">
                {{ item.exchange === 'binance' ? 'BN' : 'OKX' }}
              </span>
            </td>
            <td>
              <span class="reason-text">{{ item.reason || '-' }}</span>
            </td>
            <td>
              <span class="time-value">{{ formatDate(item.created_at) }}</span>
            </td>
            <td>
              <button class="action-btn delete" @click="confirmRemove(item)">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                </svg>
              </button>
            </td>
          </tr>
          <tr v-if="filteredItems.length === 0">
            <td colspan="5" class="empty-cell">
              <span class="empty-text">No blacklisted pairs found</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="table-footer">
      <span class="footer-info">{{ filteredItems.length }} item{{ filteredItems.length !== 1 ? 's' : '' }}</span>
    </div>

    <!-- Add Dialog -->
    <el-dialog v-model="showAddDialog" title="Add to Blacklist" width="480px" class="custom-dialog">
      <div class="dialog-body">
        <div class="form-group">
          <label class="form-label">Symbol</label>
          <input v-model="form.symbol" type="text" class="form-input" placeholder="e.g. BTCUSDT" />
        </div>
        <div class="form-group">
          <label class="form-label">Exchange</label>
          <select v-model="form.exchange" class="form-input">
            <option value="">Select exchange</option>
            <option value="binance">Binance</option>
            <option value="okx">OKX</option>
          </select>
        </div>
        <div class="form-group">
          <label class="form-label">Reason</label>
          <textarea v-model="form.reason" class="form-textarea" placeholder="Why is this pair being blacklisted?" rows="3"></textarea>
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <button class="cancel-btn" @click="showAddDialog = false">Cancel</button>
          <button class="confirm-btn" @click="handleAdd" :disabled="!form.symbol || !form.exchange">Add</button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useBlacklistStore } from '@/stores/blacklist'
import { ElMessageBox } from 'element-plus'

const store = useBlacklistStore()
const exchangeFilter = ref('')
const showAddDialog = ref(false)

const form = ref({
  symbol: '',
  exchange: '',
  reason: ''
})

const filteredItems = computed(() => {
  if (!exchangeFilter.value) return store.items
  const q = exchangeFilter.value.toLowerCase()
  return store.items.filter(i => i.exchange.toLowerCase().includes(q))
})

function formatDate(ts: string): string {
  return new Date(ts).toLocaleDateString('en-US', {
    year: 'numeric', month: 'short', day: 'numeric',
    hour: '2-digit', minute: '2-digit'
  })
}

async function handleAdd() {
  if (!form.value.symbol || !form.value.exchange) return
  await store.addItem({
    symbol: form.value.symbol,
    exchange: form.value.exchange,
    reason: form.value.reason || undefined
  })
  form.value = { symbol: '', exchange: '', reason: '' }
  showAddDialog.value = false
}

async function confirmRemove(item: { id: string; symbol: string }) {
  try {
    await ElMessageBox.confirm(
      `Remove ${item.symbol} from the blacklist?`,
      'Confirm Removal',
      { confirmButtonText: 'Remove', cancelButtonText: 'Cancel', type: 'warning' }
    )
    await store.removeItem(item.id)
  } catch {
    // cancelled
  }
}

onMounted(() => {
  store.fetchItems()
})
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&family=Outfit:wght@300;400;500;600;700&display=swap');

.blacklist-page {
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

.primary-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border-radius: 10px;
  background: rgba(0, 217, 255, 0.12);
  border: 1px solid rgba(0, 217, 255, 0.25);
  color: var(--accent-cyan);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: inherit;
}

.primary-btn svg {
  width: 18px;
  height: 18px;
}

.primary-btn:hover {
  background: rgba(0, 217, 255, 0.2);
  border-color: var(--accent-cyan);
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

.reason-text {
  font-size: 13px;
  color: var(--text-secondary);
}

.time-value {
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
}

.action-btn {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.action-btn svg {
  width: 16px;
  height: 16px;
}

.action-btn.delete:hover {
  border-color: var(--accent-red);
  color: var(--accent-red);
}

.empty-cell {
  text-align: center;
  padding: 48px 20px !important;
}

.empty-text {
  font-size: 14px;
  color: var(--text-tertiary);
}

.table-footer {
  padding: 16px 20px;
  border-top: 1px solid var(--border-color);
}

.footer-info {
  font-size: 12px;
  color: var(--text-tertiary);
}

/* Dialog */
.dialog-body {
  padding: 8px 0;
}

.form-group {
  margin-bottom: 20px;
}

.form-label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.form-input {
  width: 100%;
  padding: 10px 14px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  font-size: 14px;
  color: var(--text-primary);
  outline: none;
  transition: all 0.2s ease;
  font-family: inherit;
  box-sizing: border-box;
}

.form-input:focus {
  border-color: var(--accent-cyan);
  box-shadow: 0 0 0 3px rgba(0, 217, 255, 0.1);
}

.form-input::placeholder {
  color: var(--text-tertiary);
}

select.form-input {
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%237d8590' stroke-width='2'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
}

.form-textarea {
  width: 100%;
  padding: 10px 14px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  font-size: 14px;
  color: var(--text-primary);
  outline: none;
  transition: all 0.2s ease;
  font-family: inherit;
  resize: vertical;
  box-sizing: border-box;
}

.form-textarea:focus {
  border-color: var(--accent-cyan);
  box-shadow: 0 0 0 3px rgba(0, 217, 255, 0.1);
}

.form-textarea::placeholder {
  color: var(--text-tertiary);
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.cancel-btn {
  padding: 10px 20px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
  background: transparent;
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  font-family: inherit;
}

.cancel-btn:hover {
  border-color: var(--text-tertiary);
  color: var(--text-primary);
}

.confirm-btn {
  padding: 10px 24px;
  border-radius: 8px;
  border: none;
  background: rgba(0, 217, 255, 0.2);
  color: var(--accent-cyan);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.15s ease;
  font-family: inherit;
}

.confirm-btn:hover:not(:disabled) {
  background: rgba(0, 217, 255, 0.3);
}

.confirm-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
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
