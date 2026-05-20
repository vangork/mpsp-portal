export type Priority = 'high' | 'medium' | 'low'

export const STAGES = [
  { idx:0, label: '接收样本', icon: 'inbox', color: 'info', colour: 'var(--va-info)', },
  { idx:1, label: '核算提取', icon: 'science', color: 'warning', colour: 'var(--va-warning)', },
  { idx:2, label: '样本质检', icon: 'biotech', color: 'danger', colour: 'var(--va-danger)', },
  { idx:3, label: '建库', icon: 'build', color: 'secondary', colour: 'var(--va-secondary)', },
  { idx:4, label: '质量评估', icon: 'analytics', color: 'success', colour: 'var(--va-success)', },
  { idx:5, label: '测序', icon: 'memory', color: 'primary', colour: 'var(--va-primary)', },
] as const

export type StageIndex = typeof STAGES[number]['idx']

export const STAGE_STEPS = STAGES.map((s) => ({ label: s.label }))

export type Project = {
  id: string
  name: string
  sampleType: string
  testItem: string
  institution: string
  contact: string
  sampleCount: number
  priority: Priority
  stage: StageIndex
  createdAt: string
  notes: string
}
