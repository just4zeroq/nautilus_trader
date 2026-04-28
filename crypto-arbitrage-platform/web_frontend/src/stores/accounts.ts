import { defineStore } from 'pinia'
import { ref } from 'vue'
import { accountApi, type Account } from '@/api/accounts'
import { ElMessage } from 'element-plus'

export const useAccountStore = defineStore('account', () => {
  const accounts = ref<Account[]>([])
  const loading = ref(false)

  async function fetchAccounts() {
    loading.value = true
    try { const res = await accountApi.list(); accounts.value = res.data.accounts }
    finally { loading.value = false }
  }

  async function configureApiKey(id: string, data: { api_key: string; api_secret: string; api_passphrase?: string }) {
    await accountApi.configureApiKey(id, data)
    ElMessage.success('API Key configured')
    await fetchAccounts()
  }

  return { accounts, loading, fetchAccounts, configureApiKey }
})
