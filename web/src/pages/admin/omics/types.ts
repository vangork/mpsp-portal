import {
  DNAItem,
  MetabolismItem,
  PlasmixMetabolismItem,
  PlasmixProteinItem,
  ProteinItem,
  RNAItem,
} from '../../omics/types'

export type AdminDNAItem = DNAItem & {
  username: string
}

export type AdminRNAItem = RNAItem & {
  username: string
}

export type AdminProteinItem = ProteinItem & {
  username: string
}

export type AdminMetabolismItem = MetabolismItem & {
  username: string
}

export type AdminPlasmixProteinItem = PlasmixProteinItem & {
  username: string
}

export type AdminPlasmixMetabolismItem = PlasmixMetabolismItem & {
  username: string
}

export type AdminRNAData = {
  meta_file: string
  exp_file: string
  count_file: string
}

export type AdminOmicsData = {
  meta_file: string
  exp_file: string
}

export type RawDataType = 'meta' | 'exp' | 'count'
