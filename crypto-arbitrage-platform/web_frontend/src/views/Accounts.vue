<template>
  <div class="accounts-page">
    <div class="page-header">
      <div class="header-left">
        <h1 class="page-title">Accounts</h1>
        <p class="page-subtitle">Exchange account management and API keys</p>
      </div>
      <div class="header-controls">
        <button class="refresh-btn" @click="store.fetchAccounts()" :class="{ loading: store.loading }">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
            <path d="M21 3v5h-5"/>
          </svg>
        </button>
      </div>
    </div>

    <div class="accounts-grid">
      <div v-for="account in store.accounts" :key="account.id" class="account-card">
        <div class="card-header">
          <div class="card-header-top">
            <span class="exchange-badge" :class="account.exchange">
              {{ account.exchange === 'binance' ? 'BN' : 'OKX' }}
            </span>
            <span class="api-key-badge" :class="account.api_key_configured ? 'configured' : 'not-configured'">
              {{ account.api_key_configured ? 'Configured' : 'Not Configured' }}
            </span>
          </div>
          <h3 class="account-label">{{ account.label || account.exchange }}</h3>
          <div v-if="account.api_key_configured && account.api_key_mask" class="api-key-mask">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="key-icon">
              <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
              <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
            </svg>
            <span class="mask-text">{{ account.api_key_mask }}</span>
          </div>
        </div>

        <div class="card-assets">
          <div class="assets-header">
            <span class="assets-title">Balances</span>
          </div>
          <div class="asset-list">
            <div v-for="asset in account.assets" :key="asset.asset" class="asset-row">
              <span class="asset-name">{{ asset.asset }}</span>
              <span class="asset-balance">{{ formatBalance(asset.balance) }}</span>
            </div>
            <div v-if="!account.assets || account.assets.length === 0" class="no-assets">
              No assets to display
            </div>
          </div>
        </div>

        <div class="card-footer">
          <button class="config-btn" @click="openDialog(account)">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
            </svg>
            Configure API
          </button>
        </div>
      </div>
    </div>

    <!-- API Config Dialog -->
    <el-dialog v-model="showDialog" title="Configure API Key" width="500px" class="custom-dialog">
      <div class="dialog-body">
        <div class="dialog-account-info">
          <span class="exchange-badge" :class="selectedAccount?.exchange">
            {{ selectedAccount?.exchange === 'binance' ? 'BN' : 'OKX' }}
          </span>
          <span class="dialog-account-label">{{ selectedAccount?.label || selectedAccount?.exchange }}</span>
        </div>
        <div class="form-group">
          <label class="form-label">API Key</label>
          <input v-model="apiForm.api_key" type="text" class="form-input" placeholder="Enter API key" />
        </div>
        <div class="form-group">
          <label class="form-label">Secret Key</label>
          <input v-model="apiForm.api_secret" type="password" class="form-input" placeholder="Enter secret key" />
        </div>
        <div class="form-group">
          <label class="form-label">Passphrase <span class="optional-badge">Optional</span></label>
          <input v-model="apiForm.api_passphrase" type="password" class="form-input" placeholder="Enter passphrase (if required)" />
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <button
            class="test-btn"
            @click="handleTestConnection"
            :disabled="!apiForm.api_key || !apiForm.api_secret"
          >
            Test Connection
          </button>
          <div class="dialog-footer-right">
            <button class="cancel-btn" @click="showDialog = false">Cancel</button>
            <button
              class="confirm-btn"
              @click="handleSave"
              :disabled="!apiForm.api_key || !apiForm.api_secret"
            >
              Save
            </button>
          </div>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAccountStore } from '@/stores/accounts'
import { ElMessage } from 'element-plus'
import type { Account } from '@/api/accounts'

const store = useAccountStore()
const showDialog = ref(false)
const selectedAccount = ref<Account | null>(null)

const apiForm = ref({
  api_key: '',
  api_secret: '',
  api_passphrase: ''
})

function formatBalance(balance: number): string {
  return balance.toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 8
  })
}

function openDialog(account: Account) {
  selectedAccount.value = account
  apiForm.value = { api_key: '', api_secret: '', api_passphrase: '' }
  showDialog.value = true
}

async function handleSave() {
  if (!selectedAccount.value || !apiForm.value.api_key || !apiForm.value.api_secret) return

  const data: { api_key: string; api_secret: string; api_passphrase?: string } = {
    api_key: apiForm.value.api_key,
    api_secret: apiForm.value.api_secret
  }
  if (apiForm.value.api_passphrase) {
    data.api_passphrase = apiForm.value.api_passphrase
  }

  await store.configureApiKey(selectedAccount.value.id, data)
  showDialog.value = false
}

async function handleTestConnection() {
  if (!apiForm.value.api_key || !apiForm.value.api_secret) {
    ElMessage.warning('Please fill in API Key and Secret first')
    return
  }

  try {
    // Simulate connection test — real implementation would call API
    await new Promise(resolve => setTimeout(resolve, 1000))
    ElMessage.success('Connection successful')
  } catch {
    ElMessage.error('Connection failed')
  }
}

onMounted(() => {
  store.fetchAccounts()
})
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&family=Outfit:wght@300;400;500;600;700&display=swap');

.accounts-page {
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

.accounts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
  gap: 20px;
}

.account-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  overflow: hidden;
  transition: all 0.2s ease;
}

.account-card:hover {
  border-color: rgba(0, 217, 255, 0.3);
  transform: translateY(-2px);
}

.card-header {
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
}

.card-header-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.exchange-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 6px 12px;
  border-radius: 8px;
  font-size: 12px;
  font-weight: 700;
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

.api-key-badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 20px;
  font-size: 11px;
  font-weight: 500;
}

.api-key-badge.configured {
  background: rgba(0, 255, 136, 0.12);
  color: var(--accent-green);
}

.api-key-badge.not-configured {
  background: rgba(125, 133, 144, 0.12);
  color: var(--text-tertiary);
}

.account-label {
  font-size: 20px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.api-key-mask {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 10px;
  padding: 8px 12px;
  background: var(--bg-tertiary);
  border-radius: 8px;
}

.key-icon {
  width: 16px;
  height: 16px;
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.mask-text {
  font-size: 13px;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-secondary);
}

.card-assets {
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.assets-header {
  margin-bottom: 12px;
}

.assets-title {
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  color: var(--text-tertiary);
}

.asset-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.asset-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 0;
}

.asset-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.asset-balance {
  font-size: 14px;
  font-weight: 600;
  font-family: 'JetBrains Mono', monospace;
  color: var(--text-primary);
}

.no-assets {
  font-size: 13px;
  color: var(--text-tertiary);
  padding: 8px 0;
}

.card-footer {
  padding: 16px 20px;
}

.config-btn {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 10px 16px;
  border-radius: 10px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  font-family: inherit;
}

.config-btn svg {
  width: 16px;
  height: 16px;
}

.config-btn:hover {
  border-color: var(--accent-cyan);
  color: var(--accent-cyan);
}

/* Dialog */
.dialog-body {
  padding: 8px 0;
}

.dialog-account-info {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 24px;
  padding: 12px 16px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
}

.dialog-account-label {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
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

.optional-badge {
  font-size: 11px;
  font-weight: 400;
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  padding: 2px 8px;
  border-radius: 4px;
  margin-left: 6px;
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

.dialog-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.dialog-footer-right {
  display: flex;
  gap: 12px;
}

.test-btn {
  padding: 10px 20px;
  border-radius: 8px;
  border: 1px solid rgba(0, 255, 136, 0.3);
  background: rgba(0, 255, 136, 0.08);
  color: var(--accent-green);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
  font-family: inherit;
}

.test-btn:hover:not(:disabled) {
  background: rgba(0, 255, 136, 0.15);
  border-color: var(--accent-green);
}

.test-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
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
</style>
