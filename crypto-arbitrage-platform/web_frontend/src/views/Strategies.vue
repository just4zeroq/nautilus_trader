<template>
  <div class="strategies">
    <el-card>
      <template #header>
        <div class="card-header">
          <span>Strategy Management</span>
          <el-button type="primary" @click="refreshStrategies">
            Refresh
          </el-button>
        </div>
      </template>

      <el-table
        v-loading="store.loading"
        :data="store.strategies"
        stripe
        style="width: 100%"
      >
        <el-table-column prop="name" label="Name" width="200" />
        <el-table-column prop="strategy_type" label="Type" width="150">
          <template #default="{ row }">
            <el-tag v-if="row.strategy_type === 'cross_exchange'" type="success">
              Cross Exchange
            </el-tag>
            <el-tag v-else-if="row.strategy_type === 'triangular'" type="warning">
              Triangular
            </el-tag>
            <el-tag v-else type="info">
              Statistical
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="enabled" label="Status" width="100">
          <template #default="{ row }">
            <el-tag :type="row.enabled ? 'success' : 'danger'">
              {{ row.enabled ? 'Running' : 'Stopped' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="params" label="Parameters">
          <template #default="{ row }">
            <span class="params-text">{{ formatParams(row.params) }}</span>
          </template>
        </el-table-column>
        <el-table-column label="Actions" width="180">
          <template #default="{ row }">
            <el-button
              v-if="!row.enabled"
              type="success"
              size="small"
              @click="startStrategy(row.id)"
            >
              Start
            </el-button>
            <el-button
              v-else
              type="danger"
              size="small"
              @click="stopStrategy(row.id)"
            >
              Stop
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useStrategyStore } from '@/stores/strategies'
import { ElMessage } from 'element-plus'

const store = useStrategyStore()

onMounted(() => {
  store.fetchStrategies()
})

async function refreshStrategies() {
  await store.fetchStrategies()
}

async function startStrategy(id: string) {
  try {
    await store.startStrategy(id)
    ElMessage.success('Strategy started successfully')
  } catch {
    ElMessage.error('Failed to start strategy')
  }
}

async function stopStrategy(id: string) {
  try {
    await store.stopStrategy(id)
    ElMessage.success('Strategy stopped successfully')
  } catch {
    ElMessage.error('Failed to stop strategy')
  }
}

function formatParams(params: Record<string, unknown>): string {
  return Object.entries(params)
    .map(([k, v]) => `${k}: ${v}`)
    .join(', ')
}
</script>

<style scoped>
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.params-text {
  font-size: 12px;
  color: #909399;
}
</style>
