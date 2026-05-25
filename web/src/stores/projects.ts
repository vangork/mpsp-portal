import { defineStore } from 'pinia'
import {
  getProjects,
  updateProject,
} from '../data/pages/projects'
import { 
  Project
} from '../pages/user/projects/types'

export const useProjectsStore = defineStore('projects', {
  state: () => {
    return {
      projects: [] as Project[],
      loading: false,
    }
  },
  actions: {
    async refreshProjects() {
      this.loading = true
      try {
        this.projects = await getProjects()
      } catch (error) {
        console.log(error)
      } finally {
        this.loading = false
      }
    },
    async updateProject(item: Project) {
      await updateProject(item)
      await this.refreshProjects()
    },
  },
})
