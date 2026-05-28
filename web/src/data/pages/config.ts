import { Config } from '../../pages/admin/config/types'
import api, { error } from '../../services/api'

export const getConfig = async () => {
  const response = await fetch(api.config())
  if (!response.ok) {
    throw await error(response)
  }
  const config: Config = await response.json()
  return config
}

export const updateConfig = async (config: Config) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.adminConfig(), { method: 'PUT', body: JSON.stringify(config), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}
