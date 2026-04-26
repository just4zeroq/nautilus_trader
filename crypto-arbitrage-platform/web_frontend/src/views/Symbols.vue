<template>
  <div class="symbols">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>Symbol Management</span>
          <el-button type="primary" @click="refreshSymbols">
            Refresh
          </el-button>
        </div>
      </template>

      <el-table
        :data="symbols"
        stripe
        style="width: 100%"
      >
        <el-table-column prop="symbol" label="Symbol" width="150" />
        <el-table-column prop="exchange" label="Exchange" width="120">
          <template #default="{ row }">
            <el-tag :type="row.exchange === 'binance' ? 'success' : 'warning'">
              {{ row.exchange.toUpperCase() }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="tier" label="Tier" width="100">
          <template #default="{ row }">
            <el-tag :type="tierType(row.tier)">
              Tier {{ row.tier }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="enabled" label="Enabled" width="100">
          <template #default="{ row }">
            <el-switch v-model="row.enabled" disabled />
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios from 'axios'

interface Symbol {
  id: string
  symbol: string
  exchange: string
  tier: number
  enabled: boolean
}

const symbols = ref<Symbol[]>([])

onMounted(() => {
  refreshSymbols()
})

async function refreshSymbols() {
  try {
    const response = await axios.get('/api/v1/symbols')
    symbols.value = response.data.symbols
  } catch (e) {
    console.error('Failed to fetch symbols:', e)
  }
}

function tierType(tier: number): string {
  switch (tier) {
    case 1:
      return 'success'
    case 2:
      return 'warning'
    case 3:
      return 'info'
    default:
      return ''
  }
}
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
