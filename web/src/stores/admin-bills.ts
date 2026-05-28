import { defineStore } from 'pinia'
import { AdminBill } from '../pages/admin/billing/types'
import { getAdminBills, updateAdminBill } from '../data/pages/bills'

export const useAdminBillsStore = defineStore('adminBills', {
  state: () => {
    return {
      bills: [] as AdminBill[],
      loading: false,
    }
  },
  actions: {
    async refreshBills() {
      this.loading = true
      try {
        this.bills = await getAdminBills()
      } catch (error) {
        console.log(error)
      } finally {
        this.loading = false
      }
    },
    async updateBill(item: AdminBill) {
      await updateAdminBill(item)
      await this.refreshBills()
    },
  },
})
