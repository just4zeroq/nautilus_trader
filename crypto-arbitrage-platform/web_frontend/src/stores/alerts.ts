import { defineStore } from 'pinia'
import { ref } from 'vue'
import { alertApi, type AlertRule, type AlertChannel, type AlertHistoryItem } from '@/api/alerts'
import { ElMessage } from 'element-plus'

export const useAlertStore = defineStore('alert', () => {
  const rules = ref<AlertRule[]>([])
  const channels = ref<AlertChannel[]>([])
  const history = ref<AlertHistoryItem[]>([])
  const loading = ref(false)

  async function fetchRules() {
    try { const res = await alertApi.listRules(); rules.value = res.data.rules }
    catch { rules.value = [] }
  }

  async function createRule(data: Partial<AlertRule>) {
    await alertApi.createRule(data); ElMessage.success('Rule created'); await fetchRules()
  }

  async function updateRule(id: string, data: Partial<AlertRule>) {
    await alertApi.updateRule(id, data); ElMessage.success('Rule updated'); await fetchRules()
  }

  async function deleteRule(id: string) {
    await alertApi.deleteRule(id); ElMessage.success('Rule deleted'); await fetchRules()
  }

  async function fetchChannels() {
    try { const res = await alertApi.listChannels(); channels.value = res.data.channels }
    catch { channels.value = [] }
  }

  async function createChannel(data: Partial<AlertChannel>) {
    await alertApi.createChannel(data); ElMessage.success('Channel created'); await fetchChannels()
  }

  async function updateChannel(id: string, data: Partial<AlertChannel>) {
    await alertApi.updateChannel(id, data); ElMessage.success('Channel updated'); await fetchChannels()
  }

  async function deleteChannel(id: string) {
    await alertApi.deleteChannel(id); ElMessage.success('Channel deleted'); await fetchChannels()
  }

  async function testChannel(id: string) {
    await alertApi.testChannel(id); ElMessage.success('Test sent')
  }

  async function fetchHistory() {
    try { const res = await alertApi.listHistory(); history.value = res.data.items }
    catch { history.value = [] }
  }

  return { rules, channels, history, loading, fetchRules, createRule, updateRule, deleteRule, fetchChannels, createChannel, updateChannel, deleteChannel, testChannel, fetchHistory }
})
