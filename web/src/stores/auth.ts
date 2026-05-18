import { defineStore } from 'pinia'
import { deleteToken, getUserProfile, postToken } from '../data/pages/auth'
import { Credential, Profile } from '../pages/auth/types'

export const useAuthStore = defineStore('auth', {
  state: () => {
    return {
      profile: {
        id: 0,
        username: '',
        email: '',
        role: 0,
        last_seen: null,
        created_at: new Date(),
      } as Profile,
      status: '',
    }
  },
  getters: {
    isLoggedIn: (state) => state.profile.email != '',
    isLogining: (state) => state.status == 'logining',
    isAdmin: (state) => state.profile.role == 2,
  },
  actions: {
    async login(email: string, password: string) {
      this.status = 'logining'
      const cred: Credential = {
        email,
        password,
      }
      try {
        this.profile = await postToken(cred)
      } finally {
        this.status = ''
      }
    },
    async logout() {
      try {
        await deleteToken()
        this.resetProfile()
      } catch (error) {
        if (error instanceof Error) {
          if (error.name == '401') {
            this.resetProfile()
          }
        }
        console.log(error)
      }
    },
    async getProfile() {
      try {
        this.profile = await getUserProfile()
      } catch (error) {
        if (error instanceof Error) {
          if (error.name == '401') {
            this.resetProfile()
          }
        }
        console.log(error)
      }
    },
    resetProfile() {
      this.profile.id = 0
      this.profile.email = ''
      this.profile.username = ''
      this.profile.last_seen = null
      this.profile.created_at = new Date()
    },
  },
})
