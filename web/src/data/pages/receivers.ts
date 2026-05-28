import { Receiver } from '../../pages/admin/config/types'
import api, { error } from '../../services/api'


export const getReceivers = async () => {
  const response = await fetch(api.allReceivers())
  if (!response.ok) {
    throw await error(response)
  }
  const items: Receiver[] = await response.json()
  return items
}

export const addReceiver = async (receiver: Receiver) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.allReceivers(), { method: 'POST', body: JSON.stringify(receiver), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const updateReceiver = async (receiver: Receiver) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.receiver(receiver.id), { method: 'PUT', body: JSON.stringify(receiver), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const deleteReceiver = async (id: number) => {
  const response = await fetch(api.receiver(id), { method: 'DELETE' })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const getDefaultReceiver = async () => {
  const response = await fetch(api.defaultReceiver())
  if (!response.ok) {
    throw await error(response)
  }
  const item: Receiver | null = await response.json()
  return item
}
