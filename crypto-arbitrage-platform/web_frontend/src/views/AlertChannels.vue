<template>
  <div class="channels-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Alert Channels</h1>
        <p class="page-subtitle">Manage notification channels</p>
      </div>
      <div class="header-controls">
        <button class="refresh-btn" @click="store.fetchChannels()" :class="{ loading: store.loading }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
            <path d="M21 3v5h-5"/>
          </svg>
        </button>
        <button class="primary-btn" @click="openDialog()">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 5v14M5 12h14"/>
          </svg>
          Create Channel
        </button>
      </div>
    </div>

    <div class="stats-bar">
      <div class="stat-item">
        <span class="stat-value">{{ store.channels.length }}</span>
        <span class="stat-label">Total Channels</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat-item">
        <span class="stat-value">{{ enabledCount }}</span>
        <span class="stat-label">Enabled</span>
      </div>
    </div>

    <div class="table-container">
      <table class="data-table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Type</th>
            <th>Config</th>
            <th>Enabled</th>
            <th>Created</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="ch in store.channels" :key="ch.id" class="table-row">
            <td><span class="channel-name">{{ ch.name }}</span></td>
            <td>
              <span class="type-badge" :class="ch.channel_type">{{ ch.channel_type }}</span>
            </td>
            <td><span class="config-text">{{ formatConfig(ch) }}</span></td>
            <td>
              <el-switch
                :model-value="ch.enabled"
                @change="toggleChannel(ch)"
                class="custom-switch"
              />
            </td>
            <td><span class="time-value">{{ formatDate(ch.created_at) }}</span></td>
            <td>
              <div class="action-buttons">
                <button class="action-btn edit" @click="openDialog(ch)">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                    <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
                  </svg>
                </button>
                <button class="action-btn test" @click="handleTest(ch)">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z"/>
                  </svg>
                </button>
                <button class="action-btn delete" @click="confirmDelete(ch)">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                  </svg>
                </button>
              </div>
            </td>
          </tr>
          <tr v-if="store.channels.length === 0">
            <td colspan="6" class="empty-cell">
              <span class="empty-text">No channels configured yet</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Create/Edit Dialog -->
    <el-dialog v-model="showDialog" :title="editingChannel ? 'Edit Channel' : 'Create Channel'" width="520px" class="custom-dialog">
      <div class="dialog-body">
        <div class="form-group">
          <label class="form-label">Name</label>
          <input v-model="form.name" type="text" class="form-input" placeholder="Channel name" />
        </div>
        <div class="form-group">
          <label class="form-label">Type</label>
          <select v-model="form.channel_type" class="form-input">
            <option value="feishu">Feishu Webhook</option>
            <option value="sms">SMS</option>
            <option value="phone">Phone Call</option>
          </select>
        </div>
        <div class="form-group">
          <label class="form-label">Webhook URL / Recipient</label>
          <input v-model="form.configTarget" type="text" class="form-input" :placeholder="inputPlaceholder" />
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <button class="cancel-btn" @click="showDialog = false">Cancel</button>
          <button class="confirm-btn" @click="save">Save</button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAlertStore } from '@/stores/alerts'
import { ElMessageBox } from 'element-plus'
import type { AlertChannel } from '@/api/alerts'

const store = useAlertStore()
const showDialog = ref(false)
const editingChannel = ref<AlertChannel | null>(null)

const form = ref({
  name: '',
  channel_type: 'feishu',
  configTarget: ''
})

const inputPlaceholder = computed(() => {
  switch (form.value.channel_type) {
    case 'feishu': return 'https://open.feishu.cn/open-apis/bot/v2/hook/...'
    case 'sms': return '+1234567890'
    case 'phone': return '+1234567890'
    default: return 'Configuration value'
  }
})

const enabledCount = computed(() =>
  store.channels.filter(c => c.enabled).length
)

function formatConfig(ch: AlertChannel): string {
  if (!ch.config) return '-'
  if (typeof ch.config === 'string') return ch.config
  try { return JSON.stringify(ch.config) } catch { return String(ch.config) }
}

function formatDate(ts: string): string {
  return new Date(ts).toLocaleDateString('en-US', {
    month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit'
  })
}

function openDialog(channel?: AlertChannel) {
  if (channel) {
    editingChannel.value = channel
    form.value = {
      name: channel.name,
      channel_type: channel.channel_type,
      configTarget: typeof channel.config === 'string' ? channel.config : JSON.stringify(channel.config)
    }
  } else {
    editingChannel.value = null
    form.value = { name: '', channel_type: 'feishu', configTarget: '' }
  }
  showDialog.value = true
}

async function save() {
  const data: Partial<AlertChannel> = {
    name: form.value.name,
    channel_type: form.value.channel_type,
    config: form.value.configTarget
  }
  if (editingChannel.value) {
    await store.updateChannel(editingChannel.value.id, data)
  } else {
    await store.createChannel(data)
  }
  showDialog.value = false
}

async function confirmDelete(ch: AlertChannel) {
  try {
    await ElMessageBox.confirm(`Delete channel "${ch.name}"?`, 'Confirm', {
      confirmButtonText: 'Delete', cancelButtonText: 'Cancel', type: 'warning'
    })
    await store.deleteChannel(ch.id)
  } catch { /* cancelled */ }
}

async function handleTest(ch: AlertChannel) {
  await store.testChannel(ch.id)
}

async function toggleChannel(ch: AlertChannel) {
  await store.updateChannel(ch.id, { enabled: !ch.enabled })
}

onMounted(() => {
  store.fetchChannels()
})
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&family=Outfit:wght@300;400;500;600;700&display=swap');

.channels-page {
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

.channel-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
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

.type-badge.feishu {
  background: rgba(0, 217, 255, 0.12);
  color: var(--accent-cyan);
}

.type-badge.sms {
  background: rgba(0, 255, 136, 0.12);
  color: var(--accent-green);
}

.type-badge.phone {
  background: rgba(168, 85, 247, 0.15);
  color: var(--accent-purple);
}

.config-text {
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
  max-width: 200px;
  display: inline-block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.time-value {
  font-size: 13px;
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
}

.action-buttons {
  display: flex;
  gap: 6px;
}

.action-btn {
  width: 30px;
  height: 30px;
  border-radius: 6px;
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
  width: 14px;
  height: 14px;
}

.action-btn.edit:hover {
  border-color: var(--accent-cyan);
  color: var(--accent-cyan);
}

.action-btn.test:hover {
  border-color: var(--accent-green);
  color: var(--accent-green);
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

/* Switch Override */
.custom-switch {
  --el-switch-on-color: rgba(0, 255, 136, 0.6);
  --el-switch-off-color: rgba(125, 133, 144, 0.4);
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

.confirm-btn:hover {
  background: rgba(0, 217, 255, 0.3);
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
