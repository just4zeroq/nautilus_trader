import axios from 'axios'

export interface Node {
  id: string
  name: string
  node_type: string
  exchange: string | null
  status: string
  last_heartbeat: string | null
  latency: number | null
  version: string | null
}

export const nodeApi = {
  list: () => axios.get<{ nodes: Node[]; total: number }>('/api/v1/nodes')
}
