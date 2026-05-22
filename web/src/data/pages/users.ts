import { User } from '../../pages/admin/users/types'
import api, { error } from '../../services/api'

export type Pagination = {
  page: number
  perPage: number
  total: number
}

export type Filters = {
  isActive: boolean
  search: string
}

export const getUsers = async () => {
  const response = await fetch(api.allUsers())
  if (!response.ok) {
    throw await error(response)
  }
  const items: User[] = await response.json()
  items.forEach((i) => (i.last_seen = new Date(i.last_seen + 'Z')))
  return items
}

export const addUser = async (user: User) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.allUsers(), { method: 'POST', body: JSON.stringify(user), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const updateUser = async (user: User) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.user(user.id), { method: 'PUT', body: JSON.stringify(user), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}
