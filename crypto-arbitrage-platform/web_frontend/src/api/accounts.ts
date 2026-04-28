import axios from 'axios'

export interface Asset {
  exchange: string; asset: string; balance: number; updated_at: string
}

export interface Account {
  id: string; exchange: string; label: string | null
  api_key_configured: boolean; api_key_mask: string | null; assets: Asset[]
}

export const accountApi = {
  list: () => axios.get<{ accounts: Account[]; total: number }>('/api/v1/accounts'),
  configureApiKey: (id: string, data: { api_key: string; api_secret: string; api_passphrase?: string }) =>
    axios.post(`/api/v1/accounts/${id}/api-key`, data)
}
