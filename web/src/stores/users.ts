import { defineStore } from 'pinia'
import { addUser, getUsers, updateUser } from '../data/pages/users'
import { User } from '../pages/admin/users/types'

export const useUsersStore = defineStore('users', {
  state: () => {
    return {
      items: [] as User[],
    }
  },

  actions: {
    async getAll() {
      try {
        this.items = await getUsers()
      } catch (error) {
        console.error('Failed to fetch users:', error)
      }
    },

    async add(user: User) {
      await addUser(user)
      await this.getAll()
    },

    async update(user: User) {
      await updateUser(user)
      await this.getAll()
    },
  },
})
