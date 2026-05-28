export type Priority = 'high' | 'medium' | 'low'

export const STAGES = [
  { idx: 0, label: '接收样本', icon: 'inbox', color: 'info', colour: 'var(--va-info)' },
  { idx: 1, label: '核酸提取', icon: 'science', color: 'warning', colour: 'var(--va-warning)' },
  { idx: 2, label: '样本质检', icon: 'biotech', color: 'danger', colour: 'var(--va-danger)' },
  { idx: 3, label: '建库', icon: 'build', color: 'secondary', colour: 'var(--va-secondary)' },
  { idx: 4, label: '质量评估', icon: 'analytics', color: 'success', colour: 'var(--va-success)' },
  { idx: 5, label: '测序', icon: 'memory', color: 'primary', colour: 'var(--va-primary)' },
] as const

export type StageLabel = (typeof STAGES)[number]['label']
export type StageIndex = (typeof STAGES)[number]['idx']

export const STAGE_STEPS = STAGES.map((s) => ({ label: s.label }))

export const SAMPLE_TYPE_OPTIONS = ['细胞', 'FFPE白片', '新鲜冻存组织', '全血', '血浆', '血清', 'cfDNA', 'DNA', 'RNA']

export const TEST_ITEM_OPTIONS = [
  '全基因组测序',
  '全外显子组测序',
  '转录组测序(Poly-A法)',
  '转录组测序(Ribo-zero法)',
  '小RNA测序',
]

export type SampleEntry = {
  id: number
  originalName: string
  sampleType: string
  species: string
  tubeCount: string
  concentration: string
  volume: string
  testItem: string
  platform: string
  grouping: string
  remark: string
}

export type Project = {
  id: string
  name: string
  sampleType: string
  testItem: string
  institution: string
  contact: string
  email: string
  phone: string
  sampleCount: number
  priority: Priority
  stage: StageIndex
  createdAt: string
  notes: string
  samples: SampleEntry[]
  receiverName: string
  receiverAddress: string
  receiverPhone: string
  receiverEmail: string
  processType: 'nucleicAcid' | 'library' | 'sequencing' | ''
  nucleicAcidOperator: string
  libraryOperator: string
  sequencingOperator: string
}
