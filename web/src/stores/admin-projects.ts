import { defineStore } from 'pinia'
import {
  getAdminProjects,
  updateAdminProject,
} from '../data/pages/projects'
import { 
  AdminProject
} from '../pages/admin/projects/types'

export const useAdminProjectsStore = defineStore('adminProjects', {
  state: () => {
    return {
      projects: [] as AdminProject[],
      loading: false,
    }
  },
  actions: {
    async refreshProjects() {
      this.loading = true
      try {
        this.projects = await getAdminProjects()
      } catch (error) {
        console.log(error)
      } finally {
        this.loading = false
      }
    },
    async updateProject(item: AdminProject) {
      await updateAdminProject(item)
      await this.refreshProjects()
    },
  },
})
