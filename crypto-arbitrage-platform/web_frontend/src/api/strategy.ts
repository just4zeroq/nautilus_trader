import axios from 'axios'

const api = axios.create({
  baseURL: '/api/v1'
})

export interface Strategy {
  id: string
  name: string
  strategy_type: string
  enabled: boolean
  params: Record<string, unknown>
}

export interface StrategyListResponse {
  strategies: Strategy[]
  total: number
}

export interface StrategyStatusResponse {
  strategy_id: string
  status: string
  message?: string
}

export const strategyApi = {
  list: () => api.get<StrategyListResponse>('/strategies'),

  get: (id: string) => api.get<Strategy>(`/strategies/${id}`),

  start: (id: string) => api.post<StrategyStatusResponse>(`/strategies/${id}/start`),

  stop: (id: string) => api.post<StrategyStatusResponse>(`/strategies/${id}/stop`),

  update: (id: string, params: Record<string, unknown>) =>
    api.put<StrategyStatusResponse>(`/strategies/${id}`, { params })
}
