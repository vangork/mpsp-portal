import { Project } from '../../user/projects/types'

export type AdminBill = Project & {
  price: number
}
