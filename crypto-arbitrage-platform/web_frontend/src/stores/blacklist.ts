import { defineStore } from 'pinia'
import { ref } from 'vue'
import { blacklistApi, type BlacklistItem } from '@/api/blacklist'
import { ElMessage } from 'element-plus'

export const useBlacklistStore = defineStore('blacklist', () => {
  const items = ref<BlacklistItem[]>([])
  const loading = ref(false)

  async function fetchItems() {
    loading.value = true
    try {
      const res = await blacklistApi.list()
      items.value = res.data.items
    } finally { loading.value = false }
  }

  async function addItem(data: { symbol: string; exchange: string; reason?: string }) {
    await blacklistApi.create(data)
    ElMessage.success('Added to blacklist')
    await fetchItems()
  }

  async function removeItem(id: string) {
    await blacklistApi.delete(id)
    ElMessage.success('Removed from blacklist')
    await fetchItems()
  }

  return { items, loading, fetchItems, addItem, removeItem }
})
