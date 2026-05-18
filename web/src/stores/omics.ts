import { defineStore } from 'pinia'
import {
  DNAItem,
  RNAItem,
  ProteinItem,
  MetabolismItem,
  PlasmixProteinItem,
  PlasmixMetabolismItem,
} from '../pages/omics/types'
import {
  addDNAItem,
  getDNAItems,
  updateDNAItem,
  getDNAReport,
  addRNAItem,
  getRNAItems,
  updateRNAItem,
  getRNAReport,
  addProteinItem,
  getProteinItems,
  updateProteinItem,
  getProteinReport,
  addMetabolismItem,
  getMetabolismItems,
  updateMetabolismItem,
  getMetabolismReport,
  addPlasmixProteinItem,
  getPlasmixProteinItems,
  updatePlasmixProteinItem,
  getPlasmixProteinReport,
  addPlasmixMetabolismItem,
  getPlasmixMetabolismItems,
  updatePlasmixMetabolismItem,
  getPlasmixMetabolismReport,
} from '../data/pages/omics'

export const useOmicsStore = defineStore('omics', {
  state: () => {
    return {
      dnaItems: [] as DNAItem[],
      dnaPreventDownload: false,
      dnaLoading: false,

      rnaItems: [] as RNAItem[],
      rnaPreventDownload: false,
      rnaLoading: false,

      proteinItems: [] as ProteinItem[],
      proteinPreventDownload: false,
      proteinLoading: false,

      metabolismItems: [] as MetabolismItem[],
      metabolismPreventDownload: false,
      metabolismLoading: false,

      plasmixProteinItems: [] as PlasmixProteinItem[],
      plasmixProteinPreventDownload: false,
      plasmixProteinLoading: false,

      plasmixMetabolismItems: [] as PlasmixMetabolismItem[],
      plasmixMetabolismPreventDownload: false,
      plasmixMetabolismLoading: false,
    }
  },
  actions: {
    async refreshDNA() {
      this.dnaLoading = true
      try {
        this.dnaItems = await getDNAItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.dnaLoading = false
      }
    },
    async addDNA(item: DNAItem) {
      try {
        await addDNAItem(item)
        await this.refreshDNA()
      } catch (error) {
        console.log(error)
      }
    },
    async updateDNA(item: DNAItem) {
      try {
        await updateDNAItem(item)
        await this.refreshDNA()
      } catch (error) {
        console.log(error)
      }
    },
    async getDNAReportUrl(items: DNAItem[]) {
      try {
        this.dnaPreventDownload = true
        const url = await getDNAReport(items)
        return url
      } finally {
        this.dnaPreventDownload = false
      }
    },

    async refreshRNA() {
      this.rnaLoading = true
      try {
        this.rnaItems = await getRNAItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.rnaLoading = false
      }
    },
    async addRNA(item: RNAItem) {
      try {
        await addRNAItem(item)
        await this.refreshRNA()
      } catch (error) {
        console.log(error)
      }
    },
    async updateRNA(item: RNAItem) {
      try {
        await updateRNAItem(item)
        await this.refreshRNA()
      } catch (error) {
        console.log(error)
      }
    },
    async getRNAReportUrl(item: RNAItem) {
      try {
        this.rnaPreventDownload = true
        const url = await getRNAReport(item)
        return url
      } finally {
        this.rnaPreventDownload = false
      }
    },

    async refreshProtein() {
      this.proteinLoading = true
      try {
        this.proteinItems = await getProteinItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.proteinLoading = false
      }
    },
    async addProtein(item: ProteinItem) {
      try {
        await addProteinItem(item)
        await this.refreshProtein()
      } catch (error) {
        console.log(error)
      }
    },
    async updateProtein(item: ProteinItem) {
      try {
        await updateProteinItem(item)
        await this.refreshProtein()
      } catch (error) {
        console.log(error)
      }
    },
    async getProteinReportUrl(item: ProteinItem) {
      try {
        this.proteinPreventDownload = true
        const url = await getProteinReport(item)
        return url
      } finally {
        this.proteinPreventDownload = false
      }
    },

    async refreshMetabolism() {
      this.metabolismLoading = true
      try {
        this.metabolismItems = await getMetabolismItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.metabolismLoading = false
      }
    },
    async addMetabolism(item: MetabolismItem) {
      try {
        await addMetabolismItem(item)
        await this.refreshMetabolism()
      } catch (error) {
        console.log(error)
      }
    },
    async updateMetabolism(item: MetabolismItem) {
      try {
        await updateMetabolismItem(item)
        await this.refreshMetabolism()
      } catch (error) {
        console.log(error)
      }
    },
    async getMetabolismReportUrl(item: MetabolismItem) {
      try {
        this.metabolismPreventDownload = true
        const url = await getMetabolismReport(item)
        return url
      } finally {
        this.metabolismPreventDownload = false
      }
    },

    async refreshPlasmixProtein() {
      this.plasmixProteinLoading = true
      try {
        this.plasmixProteinItems = await getPlasmixProteinItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.plasmixProteinLoading = false
      }
    },
    async addPlasmixProtein(item: PlasmixProteinItem) {
      try {
        await addPlasmixProteinItem(item)
        await this.refreshPlasmixProtein()
      } catch (error) {
        console.log(error)
      }
    },
    async updatePlasmixProtein(item: PlasmixProteinItem) {
      try {
        await updatePlasmixProteinItem(item)
        await this.refreshPlasmixProtein()
      } catch (error) {
        console.log(error)
      }
    },
    async getPlasmixProteinReportUrl(item: PlasmixProteinItem) {
      try {
        this.plasmixProteinPreventDownload = true
        const url = await getPlasmixProteinReport(item)
        return url
      } finally {
        this.plasmixProteinPreventDownload = false
      }
    },

    async refreshPlasmixMetabolism() {
      this.plasmixMetabolismLoading = true
      try {
        this.plasmixMetabolismItems = await getPlasmixMetabolismItems()
      } catch (error) {
        console.log(error)
      } finally {
        this.plasmixMetabolismLoading = false
      }
    },
    async addPlasmixMetabolism(item: PlasmixMetabolismItem) {
      try {
        await addPlasmixMetabolismItem(item)
        await this.refreshPlasmixMetabolism()
      } catch (error) {
        console.log(error)
      }
    },
    async updatePlasmixMetabolism(item: PlasmixMetabolismItem) {
      try {
        await updatePlasmixMetabolismItem(item)
        await this.refreshPlasmixMetabolism()
      } catch (error) {
        console.log(error)
      }
    },
    async getPlasmixMetabolismReportUrl(item: PlasmixMetabolismItem) {
      try {
        this.plasmixMetabolismPreventDownload = true
        const url = await getPlasmixMetabolismReport(item)
        return url
      } finally {
        this.plasmixMetabolismPreventDownload = false
      }
    },
  },
})
