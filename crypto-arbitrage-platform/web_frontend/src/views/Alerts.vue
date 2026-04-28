<template>
  <div class="alerts-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Alert Rules</h1>
        <p class="page-subtitle">Configure monitoring alerts and notifications</p>
      </div>
      <div class="header-controls">
        <button class="refresh-btn" @click="refreshAll" :class="{ loading: store.loading }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
            <path d="M21 3v5h-5"/>
          </svg>
        </button>
      </div>
    </div>

    <div class="stats-bar">
      <div class="stat-item">
        <span class="stat-value">{{ store.rules.length }}</span>
        <span class="stat-label">Rules</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat-item">
        <span class="stat-value">{{ store.channels.length }}</span>
        <span class="stat-label">Channels</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat-item">
        <span class="stat-value">{{ store.history.length }}</span>
        <span class="stat-label">Recent Events</span>
      </div>
    </div>

    <el-tabs v-model="activeTab" class="custom-tabs">
      <!-- Rules Tab -->
      <el-tab-pane label="Rules" name="rules">
        <div class="tab-header">
          <span class="tab-subtitle">{{ store.rules.length }} rule{{ store.rules.length !== 1 ? 's' : '' }}</span>
          <button class="primary-btn" @click="openRuleDialog()">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 5v14M5 12h14"/>
            </svg>
            Create Rule
          </button>
        </div>
        <div class="table-container">
          <table class="data-table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Level</th>
                <th>Conditions</th>
                <th>Channels</th>
                <th>Enabled</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="rule in store.rules" :key="rule.id" class="table-row">
                <td><span class="rule-name">{{ rule.name }}</span></td>
                <td>
                  <span class="level-badge" :class="rule.level">{{ rule.level }}</span>
                </td>
                <td><span class="cond-text">{{ formatConditions(rule.conditions) }}</span></td>
                <td><span class="channel-count">{{ rule.channels?.length || 0 }}</span></td>
                <td>
                  <el-switch
                    :model-value="rule.enabled"
                    @change="toggleRule(rule)"
                    class="custom-switch"
                  />
                </td>
                <td>
                  <div class="action-buttons">
                    <button class="action-btn edit" @click="openRuleDialog(rule)">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                        <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
                      </svg>
                    </button>
                    <button class="action-btn delete" @click="confirmDeleteRule(rule)">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                      </svg>
                    </button>
                  </div>
                </td>
              </tr>
              <tr v-if="store.rules.length === 0">
                <td colspan="6" class="empty-cell">
                  <span class="empty-text">No alert rules configured</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </el-tab-pane>

      <!-- Channels Tab -->
      <el-tab-pane label="Channels" name="channels">
        <div class="tab-header">
          <span class="tab-subtitle">{{ store.channels.length }} channel{{ store.channels.length !== 1 ? 's' : '' }}</span>
          <button class="primary-btn" @click="openChannelDialog()">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 5v14M5 12h14"/>
            </svg>
            Create Channel
          </button>
        </div>
        <div class="table-container">
          <table class="data-table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Type</th>
                <th>Config</th>
                <th>Enabled</th>
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
                <td>
                  <div class="action-buttons">
                    <button class="action-btn edit" @click="openChannelDialog(ch)">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                        <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
                      </svg>
                    </button>
                    <button class="action-btn test" @click="handleTestChannel(ch)">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z"/>
                      </svg>
                    </button>
                    <button class="action-btn delete" @click="confirmDeleteChannel(ch)">
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                      </svg>
                    </button>
                  </div>
                </td>
              </tr>
              <tr v-if="store.channels.length === 0">
                <td colspan="5" class="empty-cell">
                  <span class="empty-text">No notification channels configured</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </el-tab-pane>

      <!-- History Tab -->
      <el-tab-pane label="History" name="history">
        <div class="tab-header">
          <span class="tab-subtitle">{{ store.history.length }} event{{ store.history.length !== 1 ? 's' : '' }}</span>
        </div>
        <div class="table-container">
          <table class="data-table">
            <thead>
              <tr>
                <th>Time</th>
                <th>Rule Name</th>
                <th>Level</th>
                <th>Triggered Value</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in store.history" :key="item.id" class="table-row">
                <td><span class="mono-value">{{ formatTime(item.created_at) }}</span></td>
                <td><span class="rule-name">{{ item.rule_name }}</span></td>
                <td>
                  <span class="level-badge" :class="item.level">{{ item.level }}</span>
                </td>
                <td><span class="mono-value">{{ formatTriggeredValue(item.triggered_value) }}</span></td>
                <td>
                  <span class="status-tag" :class="item.status">{{ item.status }}</span>
                </td>
              </tr>
              <tr v-if="store.history.length === 0">
                <td colspan="5" class="empty-cell">
                  <span class="empty-text">No alert history yet</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </el-tab-pane>
    </el-tabs>

    <!-- Rule Dialog -->
    <el-dialog v-model="showRuleDialog" :title="editingRule ? 'Edit Rule' : 'Create Rule'" width="520px" class="custom-dialog">
      <div class="dialog-body">
        <div class="form-group">
          <label class="form-label">Name</label>
          <input v-model="ruleForm.name" type="text" class="form-input" placeholder="Rule name" />
        </div>
        <div class="form-group">
          <label class="form-label">Level</label>
          <select v-model="ruleForm.level" class="form-input">
            <option value="info">Info</option>
            <option value="warn">Warning</option>
            <option value="critical">Critical</option>
          </select>
        </div>
        <div class="form-group">
          <label class="form-label">Conditions (JSON)</label>
          <textarea v-model="ruleForm.conditions" class="form-textarea" placeholder='{"field": "spread", "operator": "gt", "value": 0.5}' rows="4"></textarea>
        </div>
        <div class="form-group">
          <label class="form-label">Channels</label>
          <div class="channel-select">
            <label v-for="ch in store.channels" :key="ch.id" class="channel-checkbox">
              <input type="checkbox" :value="ch.id" v-model="ruleForm.selectedChannels" />
              <span>{{ ch.name }}</span>
            </label>
            <span v-if="store.channels.length === 0" class="no-channels">No channels available</span>
          </div>
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <button class="cancel-btn" @click="showRuleDialog = false">Cancel</button>
          <button class="confirm-btn" @click="saveRule">Save</button>
        </div>
      </template>
    </el-dialog>

    <!-- Channel Dialog -->
    <el-dialog v-model="showChannelDialog" :title="editingChannel ? 'Edit Channel' : 'Create Channel'" width="520px" class="custom-dialog">
      <div class="dialog-body">
        <div class="form-group">
          <label class="form-label">Name</label>
          <input v-model="channelForm.name" type="text" class="form-input" placeholder="Channel name" />
        </div>
        <div class="form-group">
          <label class="form-label">Type</label>
          <select v-model="channelForm.channel_type" class="form-input">
            <option value="feishu">Feishu Webhook</option>
            <option value="sms">SMS</option>
            <option value="phone">Phone Call</option>
          </select>
        </div>
        <div class="form-group">
          <label class="form-label">Webhook URL / Recipient</label>
          <input v-model="channelForm.configTarget" type="text" class="form-input" :placeholder="channelPlaceholder" />
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <button class="cancel-btn" @click="showChannelDialog = false">Cancel</button>
          <button class="confirm-btn" @click="saveChannel">Save</button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useAlertStore } from '@/stores/alerts'
import { ElMessageBox } from 'element-plus'
import type { AlertRule, AlertChannel } from '@/api/alerts'

const store = useAlertStore()
const activeTab = ref('rules')

const showRuleDialog = ref(false)
const showChannelDialog = ref(false)
const editingRule = ref<AlertRule | null>(null)
const editingChannel = ref<AlertChannel | null>(null)

const ruleForm = ref({
  name: '',
  level: 'info',
  conditions: '',
  selectedChannels: [] as string[]
})

const channelForm = ref({
  name: '',
  channel_type: 'feishu',
  configTarget: ''
})

const channelPlaceholder = computed(() => {
  switch (channelForm.value.channel_type) {
    case 'feishu': return 'https://open.feishu.cn/open-apis/bot/v2/hook/...'
    case 'sms': return '+1234567890'
    case 'phone': return '+1234567890'
    default: return 'Configuration value'
  }
})

function formatConditions(cond: any): string {
  if (!cond) return '-'
  if (typeof cond === 'string') return cond
  try { return JSON.stringify(cond) } catch { return String(cond) }
}

function formatConfig(ch: AlertChannel): string {
  if (!ch.config) return '-'
  if (typeof ch.config === 'string') return ch.config
  try { return JSON.stringify(ch.config) } catch { return String(ch.config) }
}

function formatTriggeredValue(val: any): string {
  if (val === null || val === undefined) return '-'
  if (typeof val === 'object') {
    try { return JSON.stringify(val) } catch { return String(val) }
  }
  return String(val)
}

function formatTime(ts: string): string {
  return new Date(ts).toLocaleString('en-US', { hour12: false })
}

function openRuleDialog(rule?: AlertRule) {
  if (rule) {
    editingRule.value = rule
    ruleForm.value = {
      name: rule.name,
      level: rule.level,
      conditions: typeof rule.conditions === 'string' ? rule.conditions : JSON.stringify(rule.conditions, null, 2),
      selectedChannels: rule.channels || []
    }
  } else {
    editingRule.value = null
    ruleForm.value = { name: '', level: 'info', conditions: '', selectedChannels: [] }
  }
  showRuleDialog.value = true
}

async function saveRule() {
  const data: Partial<AlertRule> = {
    name: ruleForm.value.name,
    level: ruleForm.value.level,
    conditions: ruleForm.value.conditions,
    channels: ruleForm.value.selectedChannels
  }
  if (editingRule.value) {
    await store.updateRule(editingRule.value.id, data)
  } else {
    await store.createRule(data)
  }
  showRuleDialog.value = false
}

async function confirmDeleteRule(rule: AlertRule) {
  try {
    await ElMessageBox.confirm(`Delete rule "${rule.name}"?`, 'Confirm', {
      confirmButtonText: 'Delete', cancelButtonText: 'Cancel', type: 'warning'
    })
    await store.deleteRule(rule.id)
  } catch { /* cancelled */ }
}

async function toggleRule(rule: AlertRule) {
  await store.updateRule(rule.id, { enabled: !rule.enabled })
}

function openChannelDialog(channel?: AlertChannel) {
  if (channel) {
    editingChannel.value = channel
    channelForm.value = {
      name: channel.name,
      channel_type: channel.channel_type,
      configTarget: typeof channel.config === 'string' ? channel.config : JSON.stringify(channel.config)
    }
  } else {
    editingChannel.value = null
    channelForm.value = { name: '', channel_type: 'feishu', configTarget: '' }
  }
  showChannelDialog.value = true
}

async function saveChannel() {
  const data: Partial<AlertChannel> = {
    name: channelForm.value.name,
    channel_type: channelForm.value.channel_type,
    config: channelForm.value.configTarget
  }
  if (editingChannel.value) {
    await store.updateChannel(editingChannel.value.id, data)
  } else {
    await store.createChannel(data)
  }
  showChannelDialog.value = false
}

async function confirmDeleteChannel(ch: AlertChannel) {
  try {
    await ElMessageBox.confirm(`Delete channel "${ch.name}"?`, 'Confirm', {
      confirmButtonText: 'Delete', cancelButtonText: 'Cancel', type: 'warning'
    })
    await store.deleteChannel(ch.id)
  } catch { /* cancelled */ }
}

async function handleTestChannel(ch: AlertChannel) {
  await store.testChannel(ch.id)
}

async function toggleChannel(ch: AlertChannel) {
  await store.updateChannel(ch.id, { enabled: !ch.enabled })
}

function refreshAll() {
  store.fetchRules()
  store.fetchChannels()
  store.fetchHistory()
}

onMounted(() => {
  refreshAll()
})
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&family=Outfit:wght@300;400;500;600;700&display=swap');

.alerts-page {
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

.tab-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.tab-subtitle {
  font-size: 14px;
  color: var(--text-secondary);
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

.rule-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.channel-name {
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

.cond-text,
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

.channel-count {
  font-size: 13px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-secondary);
}

.mono-value {
  font-size: 13px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-secondary);
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

/* Custom Tabs Overrides */
.custom-tabs {
  --el-tabs-header-height: 48px;
}

.custom-tabs :deep(.el-tabs__header) {
  margin: 0 0 24px 0;
  border-bottom: 1px solid var(--border-color);
}

.custom-tabs :deep(.el-tabs__nav-wrap::after) {
  display: none;
}

.custom-tabs :deep(.el-tabs__item) {
  font-family: 'Outfit', -apple-system, sans-serif;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  height: 48px;
  line-height: 48px;
  padding: 0 24px;
}

.custom-tabs :deep(.el-tabs__item:hover) {
  color: var(--text-primary);
}

.custom-tabs :deep(.el-tabs__item.is-active) {
  color: var(--accent-cyan);
}

.custom-tabs :deep(.el-tabs__active-bar) {
  background: var(--accent-cyan);
  height: 2px;
}

.custom-tabs :deep(.el-tabs__content) {
  padding: 0;
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

.channel-select {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
}

.channel-checkbox {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 14px;
  color: var(--text-primary);
  cursor: pointer;
}

.channel-checkbox input[type="checkbox"] {
  accent-color: var(--accent-cyan);
  width: 16px;
  height: 16px;
}

.no-channels {
  font-size: 13px;
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
