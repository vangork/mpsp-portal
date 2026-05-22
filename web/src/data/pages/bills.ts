import { Project, StageLabel, StageIndex } from '../../pages/user/projects/types'
import { PROJECT_SAMPLES } from './projects'
import { AdminBill } from '../../pages/admin/billing/types'

export type Pagination = {
  page: number
  perPage: number
  total: number
}

export type Filters = {
  stages: { text: StageLabel; value: StageIndex }[]
  search: string
}

export const getAdminBills = async () => {
  return PROJECT_SAMPLES.map((p) => ({
    ...p,
    price: 0,
  }))
  // const response = await fetch(api.adminDnas())
  // if (!response.ok) {
  //   throw await error(response)
  // }
  // const items: AdminDNAItem[] = await response.json()
  // items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  // return items
}

export const updateAdminBill = async (item: AdminBill) => {
  // const response = await fetch(api.adminDnas(), {
  //   method: 'PUT',
  //   headers: { 'Content-Type': 'application/json' },
  //   body: JSON.stringify({
  //     id: item.id,
  //     score: item.score,
  //   }),
  // })
  // if (!response.ok) {
  //   throw await error(response)
  // }
}
