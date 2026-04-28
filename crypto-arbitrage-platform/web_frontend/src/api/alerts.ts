import axios from 'axios'

export interface AlertRule {
  id: string; name: string; level: string; conditions: any; channels: string[]
  enabled: boolean; created_at: string; updated_at: string
}

export interface AlertChannel {
  id: string; name: string; channel_type: string; config: any
  enabled: boolean; created_at: string
}

export interface AlertHistoryItem {
  id: string; rule_id: string; rule_name: string; level: string
  conditions: any; triggered_value: any; channels: any; status: string
  error_message: string | null; created_at: string
}

export const alertApi = {
  listRules: () => axios.get<{ rules: AlertRule[]; total: number }>('/api/v1/alerts/rules'),
  createRule: (data: Partial<AlertRule>) => axios.post('/api/v1/alerts/rules', data),
  updateRule: (id: string, data: Partial<AlertRule>) => axios.put(`/api/v1/alerts/rules/${id}`, data),
  deleteRule: (id: string) => axios.delete(`/api/v1/alerts/rules/${id}`),
  listChannels: () => axios.get<{ channels: AlertChannel[]; total: number }>('/api/v1/alerts/channels'),
  createChannel: (data: Partial<AlertChannel>) => axios.post('/api/v1/alerts/channels', data),
  updateChannel: (id: string, data: Partial<AlertChannel>) => axios.put(`/api/v1/alerts/channels/${id}`, data),
  deleteChannel: (id: string) => axios.delete(`/api/v1/alerts/channels/${id}`),
  testChannel: (id: string) => axios.post(`/api/v1/alerts/channels/${id}/test`),
  listHistory: () => axios.get<{ items: AlertHistoryItem[]; total: number }>('/api/v1/alerts/history')
}
