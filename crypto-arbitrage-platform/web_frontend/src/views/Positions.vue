<template>
  <div class="positions">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>Account Balances</span>
          <el-button type="primary" @click="refreshBalances">
            Refresh
          </el-button>
        </div>
      </template>

      <el-table :data="balances" stripe style="width: 100%">
        <el-table-column prop="exchange" label="Exchange" width="150">
          <template #default="{ row }">
            <el-tag :type="row.exchange === 'binance' ? 'success' : 'warning'">
              {{ row.exchange.toUpperCase() }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="asset" label="Asset" width="150" />
        <el-table-column prop="balance" label="Balance" />
        <el-table-column prop="updated_at" label="Updated" />
      </el-table>
    </el-card>

    <el-card style="margin-top: 20px">
      <template #header>
        <div class="card-header">
          <span>Open Positions</span>
          <el-button type="primary" @click="refreshPositions">
            Refresh
          </el-button>
        </div>
      </template>

      <el-table :data="positions" stripe style="width: 100%">
        <el-table-column prop="symbol" label="Symbol" width="150" />
        <el-table-column prop="exchange" label="Exchange" width="120">
          <template #default="{ row }">
            <el-tag :type="row.exchange === 'binance' ? 'success' : 'warning'">
              {{ row.exchange.toUpperCase() }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="side" label="Side" width="100">
          <template #default="{ row }">
            <el-tag :type="row.side === 'LONG' ? 'success' : 'danger'">
              {{ row.side }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="quantity" label="Quantity" width="120" />
        <el-table-column prop="avg_price" label="Avg Price" />
      </el-table>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios from 'axios'

interface Balance {
  exchange: string
  asset: string
  balance: number
  updated_at: string
}

interface Position {
  id: string
  symbol: string
  exchange: string
  side: string
  quantity: number
  avg_price: number
}

const balances = ref<Balance[]>([])
const positions = ref<Position[]>([])

onMounted(() => {
  refreshBalances()
  refreshPositions()
})

async function refreshBalances() {
  try {
    const response = await axios.get('/api/v1/account')
    balances.value = response.data
  } catch (e) {
    console.error('Failed to fetch balances:', e)
  }
}

async function refreshPositions() {
  try {
    const response = await axios.get('/api/v1/account/positions')
    positions.value = response.data
  } catch (e) {
    console.error('Failed to fetch positions:', e)
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
