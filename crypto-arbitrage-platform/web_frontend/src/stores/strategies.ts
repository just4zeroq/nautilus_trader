import { defineStore } from 'pinia'
import { ref } from 'vue'
import { strategyApi, type Strategy } from '@/api/strategy'

export const useStrategyStore = defineStore('strategy', () => {
  const strategies = ref<Strategy[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function fetchStrategies() {
    loading.value = true
    error.value = null
    try {
      const response = await strategyApi.list()
      strategies.value = response.data.strategies
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to fetch strategies'
    } finally {
      loading.value = false
    }
  }

  async function startStrategy(id: string) {
    await strategyApi.start(id)
    await fetchStrategies()
  }

  async function stopStrategy(id: string) {
    await strategyApi.stop(id)
    await fetchStrategies()
  }

  return {
    strategies,
    loading,
    error,
    fetchStrategies,
    startStrategy,
    stopStrategy
  }
})
