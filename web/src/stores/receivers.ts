import { defineStore } from 'pinia'
import { useToast } from 'vuestic-ui'
import { addReceiver, deleteReceiver, getReceivers, updateReceiver } from '../data/pages/receivers'
import { Receiver } from '../pages/admin/config/types'

const { init } = useToast()

export const useReceiversStore = defineStore('receivers', {
  state: () => {
    return {
      items: [] as Receiver[],
      loading: false,
    }
  },

  actions: {
    async getAll() {
      this.loading = true
      try {
        this.items = await getReceivers()
      } catch (error) {
        init({ message: `Failed to fetch receivers: ${String(error)}`, color: 'danger' })
      } finally {
        this.loading = false
      }
    },

    async add(receiver: Receiver) {
      await addReceiver(receiver)
      await this.getAll()
    },

    async delete(id: number) {
      await deleteReceiver(id)
      await this.getAll()
    },

    async update(receiver: Receiver) {
      await updateReceiver(receiver)
      await this.getAll()
    },
  },
})
