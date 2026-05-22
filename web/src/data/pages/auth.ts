import { Credential, Password, Profile } from '../../pages/auth/types'
import api, { error } from '../../services/api'

export const postToken = async (cred: Credential) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.token(), { method: 'POST', body: JSON.stringify(cred), headers })
  if (!response.ok) {
    throw await error(response)
  }
  const profile: Profile = await response.json()
  if (profile.last_seen) {
    profile.last_seen = new Date(profile.last_seen + 'Z')
  }
  profile.created_at = new Date(profile.created_at + 'Z')
  return profile
}

export const deleteToken = async () => {
  const response = await fetch(api.token(), { method: 'DELETE' })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const getUserProfile = async () => {
  const response = await fetch(api.profile())
  if (!response.ok) {
    throw await error(response)
  }
  const profile: Profile = await response.json()
  if (profile.last_seen) {
    profile.last_seen = new Date(profile.last_seen + 'Z')
  }
  profile.created_at = new Date(profile.created_at + 'Z')
  return profile
}

export const resetPassword = async (password: Password) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.password(), { method: 'PUT', body: JSON.stringify(password), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}
