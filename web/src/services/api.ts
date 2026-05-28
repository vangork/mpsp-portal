const apiBaseUrl = import.meta.env.VITE_API_BASE_URL

export const error = async (resp: Response) => {
  const error = new Error(`${await resp.text()}`)
  error.name = `${resp.status}`
  return error
}

export default {
  token: () => `${apiBaseUrl}/api/auth/token`,
  profile: () => `${apiBaseUrl}/api/user/profile`,
  password: () => `${apiBaseUrl}/api/user/password`,
  ossToken: () => `${apiBaseUrl}/api/omics/token`,

  dnas: () => `${apiBaseUrl}/api/omics/quartet/dna`,
  dna: (id: number) => `${apiBaseUrl}/api/omics/quartet/dna/${id}`,
  dnaReport: () => `${apiBaseUrl}/api/omics/quartet/dna/report`,

  rnas: () => `${apiBaseUrl}/api/omics/quartet/rna`,
  rna: (id: number) => `${apiBaseUrl}/api/omics/quartet/rna/${id}`,
  rnaReport: (id: number) => `${apiBaseUrl}/api/omics/quartet/rna/${id}/report`,

  proteins: () => `${apiBaseUrl}/api/omics/quartet/protein`,
  protein: (id: number) => `${apiBaseUrl}/api/omics/quartet/protein/${id}`,
  proteinReport: (id: number) => `${apiBaseUrl}/api/omics/quartet/protein/${id}/report`,

  metabolisms: () => `${apiBaseUrl}/api/omics/quartet/metabolism`,
  metabolism: (id: number) => `${apiBaseUrl}/api/omics/quartet/metabolism/${id}`,
  metabolismReport: (id: number) => `${apiBaseUrl}/api/omics/quartet/metabolism/${id}/report`,

  plasmixProteins: () => `${apiBaseUrl}/api/omics/plasmix/protein`,
  plasmixProtein: (id: number) => `${apiBaseUrl}/api/omics/plasmix/protein/${id}`,
  plasmixProteinReport: (id: number) => `${apiBaseUrl}/api/omics/plasmix/protein/${id}/report`,

  plasmixMetabolisms: () => `${apiBaseUrl}/api/omics/plasmix/metabolism`,
  plasmixMetabolism: (id: number) => `${apiBaseUrl}/api/omics/plasmix/metabolism/${id}`,
  plasmixMetabolismReport: (id: number) => `${apiBaseUrl}/api/omics/plasmix/metabolism/${id}/report`,

  allUsers: () => `${apiBaseUrl}/api/admin/users`,
  user: (id: number) => `${apiBaseUrl}/api/admin/users/${id}`,
  users: ({ page, pageSize }: { page: number; pageSize: number }) =>
    `${apiBaseUrl}/users/?page=${page}&pageSize=${pageSize}`,

  allReceivers: () => `${apiBaseUrl}/api/admin/receivers`,
  receiver: (id: number) => `${apiBaseUrl}/api/admin/receivers/${id}`,
  defaultReceiver: () => `${apiBaseUrl}/api/default_receiver`,

  adminConfig: () => `${apiBaseUrl}/api/admin/config`,
  config: () => `${apiBaseUrl}/api/config`,

  adminDnas: () => `${apiBaseUrl}/api/admin/omics/quartet/dna`,
  adminDna: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/dna/${id}`,
  adminDnaVcf: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/dna/${id}/vcf`,
  adminDnaScore: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/dna/${id}/score`,

  adminRnas: () => `${apiBaseUrl}/api/admin/omics/quartet/rna`,
  adminRna: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/rna/${id}`,
  adminRnaData: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/rna/${id}/data`,
  adminRnaNotes: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/rna/${id}/notes`,

  adminProteins: () => `${apiBaseUrl}/api/admin/omics/quartet/protein`,
  adminProtein: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/protein/${id}`,
  adminProteinData: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/protein/${id}/data`,
  adminProteinNotes: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/protein/${id}/notes`,

  adminMetabolisms: () => `${apiBaseUrl}/api/admin/omics/quartet/metabolism`,
  adminMetabolism: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/metabolism/${id}`,
  adminMetabolismData: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/metabolism/${id}/data`,
  adminMetabolismNotes: (id: number) => `${apiBaseUrl}/api/admin/omics/quartet/metabolism/${id}/notes`,

  adminPlasmixProteins: () => `${apiBaseUrl}/api/admin/omics/plasmix/protein`,
  adminPlasmixProtein: (id: number) => `${apiBaseUrl}/api/admin/omics/plasmix/protein/${id}`,
  adminPlasmixProteinData: (id: number) => `${apiBaseUrl}/api/admin/omics/plasmix/protein/${id}/data`,
  adminPlasmixProteinNotes: (id: number) => `${apiBaseUrl}/api/admin/omics/plasmix/protein/${id}/notes`,

  adminPlasmixMetabolisms: () => `${apiBaseUrl}/api/admin/omics/plasmix/metabolism`,
  adminPlasmixMetabolism: (id: number) => `${apiBaseUrl}/api/admin/omics/plasmix/metabolism/${id}`,
  adminPlasmixMetabolismData: (id: number) => `${apiBaseUrl}/api/admin/omics/plasmix/metabolism/${id}/data`,
  adminPlasmixMetabolismNotes: (id: number) => `${apiBaseUrl}/api/admin/omics/plasmix/metabolism/${id}/notes`,

  allProjects: () => `${apiBaseUrl}/projects`,
  project: (id: string) => `${apiBaseUrl}/projects/${id}`,
  projects: ({ page, pageSize }: { page: number; pageSize: number }) =>
    `${apiBaseUrl}/projects/?page=${page}&pageSize=${pageSize}`,
}
