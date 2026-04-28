import axios from 'axios'

export interface BlacklistItem {
  id: string
  symbol: string
  exchange: string
  reason: string | null
  created_at: string
  created_by: string
}

export const blacklistApi = {
  list: () => axios.get<{ items: BlacklistItem[]; total: number }>('/api/v1/blacklist'),
  create: (data: { symbol: string; exchange: string; reason?: string }) =>
    axios.post('/api/v1/blacklist', data),
  delete: (id: string) => axios.delete(`/api/v1/blacklist/${id}`)
}
