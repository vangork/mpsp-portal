export type Credential = {
  email: string
  password: string
}

export type Password = {
  current_password: string
  new_password: string
}

export type Profile = {
  id: number
  username: string
  email: string
  role: number
  last_seen: Date | null
  created_at: Date
}
