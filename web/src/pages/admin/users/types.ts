const KeyToVal = {
  0: 'user',
  1: 'owner',
  2: 'admin',
} as const

export type UserRoleKeys = keyof typeof KeyToVal
export type UserRole = (typeof KeyToVal)[UserRoleKeys]

export type User = {
  id: number
  username: string
  email: string
  password: string
  role: UserRoleKeys
  active: boolean
  notes: string
  last_seen: Date
}

export const role = (key: UserRoleKeys): UserRole => {
  return KeyToVal[key]
}
