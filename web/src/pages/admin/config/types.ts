export type Contact = {
  name: string
  email: string
  phone: string
  institution: string
}

export type Receiver = Contact & {
  id: number
  default: boolean
}

export type Config = {
  sample_types: string
  test_items: string
}
