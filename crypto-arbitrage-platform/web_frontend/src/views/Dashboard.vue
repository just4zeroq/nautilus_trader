<template>
  <div class="dashboard">
    <el-row :gutter="20">
      <el-col :span="6">
        <el-card class="stat-card">
          <template #header>
            <span>Total Balance</span>
          </template>
          <div class="stat-value">$100,000.00</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <template #header>
            <span>Daily P/L</span>
          </template>
          <div class="stat-value positive">+$1,234.56</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <template #header>
            <span>Active Strategies</span>
          </template>
          <div class="stat-value">2</div>
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <template #header>
            <span>Open Positions</span>
          </template>
          <div class="stat-value">5</div>
        </el-card>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-top: 20px">
      <el-col :span="12">
        <el-card>
          <template #header>
            <span>Recent Signals</span>
          </template>
          <el-table :data="recentSignals" style="width: 100%">
            <el-table-column prop="time" label="Time" width="120" />
            <el-table-column prop="strategy" label="Strategy" width="150" />
            <el-table-column prop="symbol" label="Symbol" width="120" />
            <el-table-column prop="signal" label="Signal" />
          </el-table>
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card>
          <template #header>
            <span>Active Positions</span>
          </template>
          <el-table :data="positions" style="width: 100%">
            <el-table-column prop="symbol" label="Symbol" width="120" />
            <el-table-column prop="exchange" label="Exchange" width="100" />
            <el-table-column prop="side" label="Side" width="80" />
            <el-table-column prop="quantity" label="Quantity" width="100" />
            <el-table-column prop="pnl" label="P/L" />
          </el-table>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const recentSignals = ref([
  { time: '10:30:00', strategy: 'Cross Exchange', symbol: 'BTCUSDT', signal: 'BUY' },
  { time: '10:25:00', strategy: 'Triangular', symbol: 'ETH/USDT', signal: 'SELL' },
  { time: '10:20:00', strategy: 'Statistical', symbol: 'ADAUSDT', signal: 'CLOSE' }
])

const positions = ref([
  { symbol: 'BTCUSDT', exchange: 'Binance', side: 'LONG', quantity: '0.5', pnl: '+200.00' },
  { symbol: 'ETHUSDT', exchange: 'OKX', side: 'LONG', quantity: '5.0', pnl: '+50.00' }
])
</script>

<style scoped>
.stat-card {
  text-align: center;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  padding: 10px 0;
}

.stat-value.positive {
  color: #67c23a;
}

.stat-value.negative {
  color: #f56c6c;
}
</style>
