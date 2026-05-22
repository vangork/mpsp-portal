import { Project, StageLabel, StageIndex } from '../../pages/user/projects/types'
import { AdminProject } from '../../pages/admin/projects/types'

export type Pagination = {
  page: number
  perPage: number
  total: number
}

export type Filters = {
  stages: { text: StageLabel; value: StageIndex }[]
  search: string
}

export const getAdminProjects = async () => {
  return PROJECT_SAMPLES.map((p) => ({
    ...p,
    username: p.contact + '@hospital.com',
  }))
  // const response = await fetch(api.adminDnas())
  // if (!response.ok) {
  //   throw await error(response)
  // }
  // const items: AdminDNAItem[] = await response.json()
  // items.forEach((i) => (i.determinated_at = new Date(i.determinated_at + 'Z')))
  // return items
}

export const updateAdminProject = async (item: AdminProject) => {
  // const response = await fetch(api.adminDnas(), {
  //   method: 'PUT',
  //   headers: { 'Content-Type': 'application/json' },
  //   body: JSON.stringify({
  //     id: item.id,
  //     score: item.score,
  //   }),
  // })
  // if (!response.ok) {
  //   throw await error(response)
  // }
}

export const PROJECT_SAMPLES: Project[] = [
  // Stage 0: 接收样本
  {
    id: 'P-2026-001',
    name: '申康肿瘤DNA甲基化研究',
    email: 'zhangsan@hospital.com',
    phone: '13800138000',
    sampleType: '血浆',
    testItem: '全基因组测序',
    institution: '上海市肿瘤医院',
    contact: '张三',
    sampleCount: 12,
    priority: 'high',
    stage: 0,
    createdAt: '2026-05-15 10:20:21',
    notes: '需优先处理，样本已到位',
  },
  {
    id: 'P-2026-002',
    name: '肝癌转录组联合分析',
    email: 'lisi@hospital.com',
    phone: '13800138001',
    sampleType: '新鲜冻存组织',
    testItem: '转录组测序(Poly-A法)',
    institution: '复旦大学附属中山医院',
    contact: '李四',
    sampleCount: 8,
    priority: 'medium',
    stage: 0,
    createdAt: '2026-05-17 14:30:00',
    notes: '',
  },
  // Stage 1: 核算提取
  {
    id: 'P-2025-018',
    name: '蛋白质组基线研究',
    email: 'wangwu@hospital.com',
    phone: '13800138002',
    sampleType: '细胞',
    testItem: '蛋白质组',
    institution: '申康生物样本库',
    contact: '王五',
    sampleCount: 24,
    priority: 'medium',
    stage: 1,
    createdAt: '2026-05-10 09:00:00',
    notes: '',
  },
  {
    id: 'P-2025-019',
    name: '血浆代谢组标准化验证',
    email: 'zhaoliu@hospital.com',
    phone: '13800138003',
    sampleType: '血清',
    testItem: '代谢组',
    institution: '上海交大医学院',
    contact: '赵六',
    sampleCount: 6,
    priority: 'low',
    stage: 1,
    createdAt: '2026-05-12 16:00:00',
    notes: '',
  },
  // Stage 2: 样本质检
  {
    id: 'P-2025-015',
    name: '外显子突变频谱分析',
    email: 'qianqi@hospital.com',
    phone: '13800138004',
    sampleType: 'FFPE白片',
    testItem: '全外显子组测序',
    institution: '上海市第一人民医院',
    contact: '钱七',
    sampleCount: 16,
    priority: 'high',
    stage: 2,
    createdAt: '2026-05-05 09:00:00',
    notes: '样本来自2024年存档',
  },
  {
    id: 'P-2025-016',
    name: '小RNA调控网络研究',
    sampleType: '血浆',
    testItem: '小RNA测序',
    institution: '复旦大学遗传所',
    contact: '孙八',
    email: 'sunba@hospital.com',
    phone: '13800138005',
    sampleCount: 20,
    priority: 'medium',
    stage: 2,
    createdAt: '2026-05-07 10:00:00',
    notes: '',
  },
  {
    id: 'P-2025-017',
    name: '全血基因组变异研究',
    sampleType: '全血',
    testItem: '全基因组测序',
    institution: '华山医院',
    contact: '周九',
    email: 'zhoujiu@hospital.com',
    phone: '13800138006',
    sampleCount: 10,
    priority: 'low',
    stage: 2,
    createdAt: '2026-05-08 14:00:00',
    notes: '',
  },
  // Stage 3: 建库
  {
    id: 'P-2025-010',
    name: '细胞系基因组稳定性',
    sampleType: '细胞',
    testItem: '全基因组测序',
    institution: '生科院',
    contact: '吴十',
    email: 'wushi@hospital.com',
    phone: '13800138007',
    sampleCount: 4,
    priority: 'medium',
    stage: 3,
    createdAt: '2026-04-28 09:00:00',
    notes: '',
  },
  {
    id: 'P-2025-011',
    name: 'Plasmix蛋白组标准化',
    sampleType: '血清',
    testItem: '蛋白质组',
    institution: '申康质控中心',
    contact: '郑一',
    email: 'zhengyi@hospital.com',
    phone: '13800138008',
    sampleCount: 8,
    priority: 'high',
    stage: 3,
    createdAt: '2026-04-30 10:00:00',
    notes: 'Plasmix标准品验证项目',
  },
  {
    id: 'P-2025-012',
    name: '代谢通路干预研究',
    sampleType: '血浆',
    testItem: '代谢组',
    institution: '仁济医院',
    contact: '冯二',
    email: 'fenger@hospital.com',
    phone: '13800138009',
    sampleCount: 30,
    priority: 'medium',
    stage: 3,
    createdAt: '2026-05-02 10:00:00',
    notes: '',
  },
  // Stage 4: 质量评估
  {
    id: 'P-2025-007',
    name: '结肠癌转录组测序',
    sampleType: '新鲜冻存组织',
    testItem: '转录组测序(Ribo-zero法)',
    institution: '上海肿瘤医院',
    contact: '陈三',
    email: 'chen3@hospital.com',
    phone: '13800138010',
    sampleCount: 18,
    priority: 'medium',
    stage: 4,
    createdAt: '2026-04-20 09:00:00',
    notes: '',
  },
  {
    id: 'P-2025-008',
    name: '遗传性肿瘤外显子研究',
    sampleType: '细胞',
    testItem: '全外显子组测序',
    institution: '遗传所',
    contact: '楚四',
    email: 'chusi@hospital.com',
    phone: '13800138011',
    sampleCount: 12,
    priority: 'high',
    stage: 4,
    createdAt: '2026-04-22 10:00:00',
    notes: 'QC结果待审核',
  },
  // Stage 5: 测序
  {
    id: 'P-2025-003',
    name: '液体活检基因组研究',
    sampleType: '血浆',
    testItem: '全基因组测序',
    institution: '东方医院',
    contact: '蒋五',
    email: 'jiangwu@hospital.com',
    phone: '13800138012',
    sampleCount: 25,
    priority: 'low',
    stage: 5,
    createdAt: '2026-04-10 09:00:00',
    notes: '',
  },
  {
    id: 'P-2025-004',
    name: '代谢组疾病标志物',
    sampleType: '血清',
    testItem: '代谢组',
    institution: '新华医院',
    contact: '沈六',
    email: 'shenliu@hospital.com',
    phone: '13800138013',
    sampleCount: 15,
    priority: 'medium',
    stage: 5,
    createdAt: '2026-04-12 10:00:00',
    notes: '',
  },
  {
    id: 'P-2025-005',
    name: '蛋白组织差异分析',
    sampleType: '新鲜冻存组织',
    testItem: '蛋白质组',
    institution: '儿科医院',
    contact: '韩七',
    email: 'hanqi@hospital.com',
    phone: '13800138015',
    sampleCount: 9,
    priority: 'low',
    stage: 5,
    createdAt: '2026-04-15 10:00:00',
    notes: '',
  },
  {
    id: 'P-2025-006',
    name: '神经退行性转录研究',
    sampleType: '细胞',
    testItem: '转录组测序(Poly-A法)',
    institution: '精神卫生中心',
    contact: '杨八',
    email: 'yangba@hospital.com',
    phone: '13800138014',
    sampleCount: 6,
    priority: 'medium',
    stage: 5,
    createdAt: '2026-04-18 10:00:00',
    notes: '',
  },
]
