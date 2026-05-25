import { Project } from '../../user/projects/types'

export type FeeItem = {
  unitPrice: number | null
  totalPrice: number | null
}

export type AdminBill = Project & {
  price: number
  extractionFee: FeeItem
  qualityCheckFee: FeeItem
  libraryFee: FeeItem
  sequencingFee: FeeItem
  analysisServiceFee: FeeItem
  status: 'pending' | 'paid' | 'finish'
}
