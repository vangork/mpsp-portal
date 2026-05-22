import {
  AliOSSToken,
  DNAItem,
  RNAItem,
  ProteinItem,
  MetabolismItem,
  PlasmixProteinItem,
  PlasmixMetabolismItem,
} from '../../pages/omics/types'
import {
  AdminDNAItem,
  AdminRNAItem,
  AdminRNAData,
  AdminProteinItem,
  AdminOmicsData,
  AdminMetabolismItem,
  AdminPlasmixProteinItem,
  AdminPlasmixMetabolismItem,
} from '../../pages/admin/omics/types'
import api, { error } from '../../services/api'

export type Pagination = {
  page: number
  perPage: number
  total: number
}

export type Filters = {
  users: { text: string; value: number }[]
  search: string
}

export const getAliOssToken = async () => {
  const response = await fetch(api.ossToken())
  if (!response.ok) {
    throw await error(response)
  }
  const token: AliOSSToken = await response.json()
  return token
}

// DNA
export const getDNAItems = async () => {
  const response = await fetch(api.dnas())
  if (!response.ok) {
    throw await error(response)
  }
  const items: DNAItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const addDNAItem = async (item: DNAItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.dnas(), { method: 'POST', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const updateDNAItem = async (item: DNAItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  item.determinated_at = new Date(item.determinated_at)

  const response = await fetch(api.dna(item.id), { method: 'PUT', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const getDNAReport = async (items: DNAItem[]) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at)))

  const response = await fetch(api.dnaReport(), { method: 'POST', body: JSON.stringify(items), headers })
  if (!response.ok) {
    throw await error(response)
  }
  const report: string = await response.json()
  return report
}

// RNA
export const getRNAItems = async () => {
  const response = await fetch(api.rnas())
  if (!response.ok) {
    throw await error(response)
  }
  const items: RNAItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const addRNAItem = async (item: RNAItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.rnas(), { method: 'POST', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const updateRNAItem = async (item: RNAItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  item.determinated_at = new Date(item.determinated_at)

  const response = await fetch(api.rna(item.id), { method: 'PUT', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const getRNAReport = async (item: RNAItem) => {
  const response = await fetch(api.rnaReport(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const report: string = await response.json()
  return report
}

// Protein
export const getProteinItems = async () => {
  const response = await fetch(api.proteins())
  if (!response.ok) {
    throw await error(response)
  }
  const items: ProteinItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const addProteinItem = async (item: ProteinItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.proteins(), { method: 'POST', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const updateProteinItem = async (item: ProteinItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  item.determinated_at = new Date(item.determinated_at)

  const response = await fetch(api.protein(item.id), { method: 'PUT', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const getProteinReport = async (item: ProteinItem) => {
  const response = await fetch(api.proteinReport(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const report: string = await response.json()
  return report
}

// Metabolism
export const getMetabolismItems = async () => {
  const response = await fetch(api.metabolisms())
  if (!response.ok) {
    throw await error(response)
  }
  const items: MetabolismItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const addMetabolismItem = async (item: MetabolismItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.metabolisms(), { method: 'POST', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const updateMetabolismItem = async (item: MetabolismItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  item.determinated_at = new Date(item.determinated_at)

  const response = await fetch(api.metabolism(item.id), { method: 'PUT', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const getMetabolismReport = async (item: MetabolismItem) => {
  const response = await fetch(api.metabolismReport(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const report: string = await response.json()
  return report
}

// Plasmix Protein
export const getPlasmixProteinItems = async () => {
  const response = await fetch(api.plasmixProteins())
  if (!response.ok) {
    throw await error(response)
  }
  const items: PlasmixProteinItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const addPlasmixProteinItem = async (item: PlasmixProteinItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.plasmixProteins(), { method: 'POST', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const updatePlasmixProteinItem = async (item: PlasmixProteinItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  item.determinated_at = new Date(item.determinated_at)

  const response = await fetch(api.plasmixProtein(item.id), { method: 'PUT', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const getPlasmixProteinReport = async (item: PlasmixProteinItem) => {
  const response = await fetch(api.plasmixProteinReport(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const report: string = await response.json()
  return report
}

// Plasmix Metabolism
export const getPlasmixMetabolismItems = async () => {
  const response = await fetch(api.plasmixMetabolisms())
  if (!response.ok) {
    throw await error(response)
  }
  const items: PlasmixMetabolismItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const addPlasmixMetabolismItem = async (item: PlasmixMetabolismItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')

  const response = await fetch(api.plasmixMetabolisms(), { method: 'POST', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const updatePlasmixMetabolismItem = async (item: PlasmixMetabolismItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  item.determinated_at = new Date(item.determinated_at)

  const response = await fetch(api.plasmixMetabolism(item.id), { method: 'PUT', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

export const getPlasmixMetabolismReport = async (item: PlasmixMetabolismItem) => {
  const response = await fetch(api.plasmixMetabolismReport(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const report: string = await response.json()
  return report
}

// Admin DNA
export const getAdminDNAItems = async () => {
  const response = await fetch(api.adminDnas())
  if (!response.ok) {
    throw await error(response)
  }
  const items: AdminDNAItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const getAdminDNAVcf = async (item: AdminDNAItem) => {
  const response = await fetch(api.adminDnaVcf(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const vcf: string = await response.json()
  return vcf
}

export const updateAdminDNAScore = async (item: AdminDNAItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  const response = await fetch(api.adminDnaScore(item.id), { method: 'PUT', body: JSON.stringify(item), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

// Admin RNA
export const getAdminRNAItems = async () => {
  const response = await fetch(api.adminRnas())
  if (!response.ok) {
    throw await error(response)
  }
  const items: AdminRNAItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const getAdminRNAData = async (item: AdminRNAItem) => {
  const response = await fetch(api.adminRnaData(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const data: AdminRNAData = await response.json()
  return data
}

export const updateAdminRNANotes = async (item: AdminRNAItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  const response = await fetch(api.adminRnaNotes(item.id), { method: 'PUT', body: JSON.stringify(item.notes), headers })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

// Admin Protein
export const getAdminProteinItems = async () => {
  const response = await fetch(api.adminProteins())
  if (!response.ok) {
    throw await error(response)
  }
  const items: AdminProteinItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const getAdminProteinData = async (item: AdminProteinItem) => {
  const response = await fetch(api.adminProteinData(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const data: AdminOmicsData = await response.json()
  return data
}

export const updateAdminProteinNotes = async (item: AdminProteinItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  const response = await fetch(api.adminProteinNotes(item.id), {
    method: 'PUT',
    body: JSON.stringify(item.notes),
    headers,
  })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

// Admin Metabolism
export const getAdminMetabolismItems = async () => {
  const response = await fetch(api.adminMetabolisms())
  if (!response.ok) {
    throw await error(response)
  }
  const items: AdminMetabolismItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const getAdminMetabolismData = async (item: AdminMetabolismItem) => {
  const response = await fetch(api.adminMetabolismData(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const data: AdminOmicsData = await response.json()
  return data
}

export const updateAdminMetabolismNotes = async (item: AdminMetabolismItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  const response = await fetch(api.adminMetabolismNotes(item.id), {
    method: 'PUT',
    body: JSON.stringify(item.notes),
    headers,
  })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

// Admin Plasmix Protein
export const getAdminPlasmixProteinItems = async () => {
  const response = await fetch(api.adminPlasmixProteins())
  if (!response.ok) {
    throw await error(response)
  }
  const items: AdminPlasmixProteinItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const getAdminPlasmixProteinData = async (item: AdminPlasmixProteinItem) => {
  const response = await fetch(api.adminPlasmixProteinData(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const data: AdminOmicsData = await response.json()
  return data
}

export const updateAdminPlasmixProteinNotes = async (item: AdminPlasmixProteinItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  const response = await fetch(api.adminPlasmixProteinNotes(item.id), {
    method: 'PUT',
    body: JSON.stringify(item.notes),
    headers,
  })
  if (!response.ok) {
    throw await error(response)
  }
  return
}

// Admin Plasmix Metabolism
export const getAdminPlasmixMetabolismItems = async () => {
  const response = await fetch(api.adminPlasmixMetabolisms())
  if (!response.ok) {
    throw await error(response)
  }
  const items: AdminPlasmixMetabolismItem[] = await response.json()
  items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  return items
}

export const getAdminPlasmixMetabolismData = async (item: AdminPlasmixMetabolismItem) => {
  const response = await fetch(api.adminPlasmixMetabolismData(item.id))
  if (!response.ok) {
    throw await error(response)
  }
  const data: AdminOmicsData = await response.json()
  return data
}

export const updateAdminPlasmixMetabolismNotes = async (item: AdminPlasmixMetabolismItem) => {
  const headers = new Headers()
  headers.append('Content-Type', 'application/json')
  const response = await fetch(api.adminPlasmixMetabolismNotes(item.id), {
    method: 'PUT',
    body: JSON.stringify(item.notes),
    headers,
  })
  if (!response.ok) {
    throw await error(response)
  }
  return
}
