import { defineStore } from 'pinia'
import {
  getAdminDNAItems,
  getDNAReport,
  updateAdminDNAScore,
  getAdminRNAItems,
  getRNAReport,
  updateAdminRNANotes,
  getAdminProteinItems,
  getProteinReport,
  updateAdminProteinNotes,
  getAdminMetabolismItems,
  getMetabolismReport,
  updateAdminMetabolismNotes,
  getPlasmixProteinReport,
  getAdminPlasmixProteinItems,
  updateAdminPlasmixProteinNotes,
  getAdminPlasmixMetabolismItems,
  getPlasmixMetabolismReport,
  updateAdminPlasmixMetabolismNotes,
} from '../data/pages/omics'
import {
  AdminDNAItem,
  AdminMetabolismItem,
  AdminPlasmixMetabolismItem,
  AdminPlasmixProteinItem,
  AdminProteinItem,
  AdminRNAItem,
} from '../pages/admin/omics/types'

export const useAdminOmicsStore = defineStore('adminOmics', {
  state: () => {
    return {
      dnaItems: [] as AdminDNAItem[],
      dnaPreventDownload: false,
      dnaLoading: false,

      rnaItems: [] as AdminRNAItem[],
      rnaPreventDownload: false,
      rnaLoading: false,

      proteinItems: [] as AdminProteinItem[],
      proteinPreventDownload: false,
      proteinLoading: false,

      metabolismItems: [] as AdminMetabolismItem[],
      metabolismPreventDownload: false,
      metabolismLoading: false,

      plasmixProteinItems: [] as AdminPlasmixProteinItem[],
      plasmixProteinPreventDownload: false,
      plasmixProteinLoading: false,

      plasmixMetabolismItems: [] as AdminPlasmixMetabolismItem[],
      plasmixMetabolismPreventDownload: false,
      plasmixMetabolismLoading: false,
    }
  },
  actions: {
    async refreshDNA() {
      this.dnaLoading = true
      try {
        this.dnaItems = await getAdminDNAItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.dnaLoading = false
      }
    },
    async getDNAReportUrl(items: AdminDNAItem[]) {
      try {
        this.dnaPreventDownload = true
        const url = await getDNAReport(items)
        return url
      } finally {
        this.dnaPreventDownload = false
      }
    },
    async updateDNAScore(item: AdminDNAItem) {
      await updateAdminDNAScore(item)
      await this.refreshDNA()
    },

    async refreshRNA() {
      this.rnaLoading = true
      try {
        this.rnaItems = await getAdminRNAItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.rnaLoading = false
      }
    },
    async getRNAReportUrl(item: AdminRNAItem) {
      try {
        this.rnaPreventDownload = true
        const url = await getRNAReport(item)
        return url
      } finally {
        this.rnaPreventDownload = false
      }
    },
    async updateRNANotes(item: AdminRNAItem) {
      await updateAdminRNANotes(item)
      await this.refreshRNA()
    },

    async refreshProtein() {
      this.proteinLoading = true
      try {
        this.proteinItems = await getAdminProteinItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.proteinLoading = false
      }
    },
    async getProteinReportUrl(item: AdminProteinItem) {
      try {
        this.proteinPreventDownload = true
        const url = await getProteinReport(item)
        return url
      } finally {
        this.proteinPreventDownload = false
      }
    },
    async updateProteinNotes(item: AdminProteinItem) {
      await updateAdminProteinNotes(item)
      await this.refreshProtein()
    },

    async refreshMetabolism() {
      this.metabolismLoading = true
      try {
        this.metabolismItems = await getAdminMetabolismItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.metabolismLoading = false
      }
    },
    async getMetabolismReportUrl(item: AdminMetabolismItem) {
      try {
        this.metabolismPreventDownload = true
        const url = await getMetabolismReport(item)
        return url
      } finally {
        this.metabolismPreventDownload = false
      }
    },
    async updateMetabolismNotes(item: AdminMetabolismItem) {
      await updateAdminMetabolismNotes(item)
      await this.refreshMetabolism()
    },

    async refreshPlasmixProtein() {
      this.plasmixProteinLoading = true
      try {
        this.plasmixProteinItems = await getAdminPlasmixProteinItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.plasmixProteinLoading = false
      }
    },
    async getPlasmixProteinReportUrl(item: AdminPlasmixProteinItem) {
      try {
        this.plasmixProteinPreventDownload = true
        const url = await getPlasmixProteinReport(item)
        return url
      } finally {
        this.plasmixProteinPreventDownload = false
      }
    },
    async updatePlasmixProteinNotes(item: AdminPlasmixProteinItem) {
      await updateAdminPlasmixProteinNotes(item)
      await this.refreshPlasmixProtein()
    },

    async refreshPlasmixMetabolism() {
      this.plasmixMetabolismLoading = true
      try {
        this.plasmixMetabolismItems = await getAdminPlasmixMetabolismItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.plasmixMetabolismLoading = false
      }
    },
    async getPlasmixMetabolismReportUrl(item: AdminPlasmixMetabolismItem) {
      try {
        this.plasmixMetabolismPreventDownload = true
        const url = await getPlasmixMetabolismReport(item)
        return url
      } finally {
        this.plasmixMetabolismPreventDownload = false
      }
    },
    async updatePlasmixMetabolismNotes(item: AdminPlasmixMetabolismItem) {
      await updateAdminPlasmixMetabolismNotes(item)
      await this.refreshPlasmixMetabolism()
    },
  },
})
