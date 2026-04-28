import { defineStore } from 'pinia'
import { ref } from 'vue'
import { nodeApi, type Node } from '@/api/nodes'

export const useNodeStore = defineStore('node', () => {
  const nodes = ref<Node[]>([])
  const loading = ref(false)

  async function fetchNodes() {
    loading.value = true
    try {
      const res = await nodeApi.list()
      nodes.value = res.data.nodes
    } catch {
      nodes.value = []
    } finally {
      loading.value = false
    }
  }

  return { nodes, loading, fetchNodes }
})
