<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { getDefaultReceiver } from '../data/pages/receivers'
import { Receiver } from '../pages/admin/config/types'
import { SAMPLE_TYPE_OPTIONS, TEST_ITEM_OPTIONS } from './user/projects/types'

const router = useRouter()

const collaborator = ref({
  name: '',
  institution: '',
  phone: '',
  email: '',
})

const receiver = ref<Receiver>({
  id: -1,
  name: 'Loading...',
  institution: 'Loading...',
  phone: 'Loading...',
  email: 'Loading...',
  default: true,
})

interface SampleEntry {
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

const samples = ref<SampleEntry[]>([
  {
    id: 0,
    originalName: 'Sample1',
    sampleType: '细胞',
    species: '人',
    tubeCount: '1',
    concentration: '757.6',
    volume: '40',
    testItem: '全外显子组测序',
    platform: 'Illumina',
    grouping: '',
    remark: '20G',
  },
])

const nextId = ref(1)

const addSample = () => {
  samples.value.push({
    id: nextId.value++,
    originalName: '',
    sampleType: '',
    species: '',
    tubeCount: '',
    concentration: '',
    volume: '',
    testItem: '',
    platform: '',
    grouping: '',
    remark: '',
  })
}

const removeSample = (index: number) => {
  samples.value.splice(index, 1)
}

const submitted = ref(false)
const showConfirm = ref(false)

const handleSubmit = () => {
  showConfirm.value = true
}

const confirmSubmit = () => {
  showConfirm.value = false
  submitted.value = true
}

const cancelSubmit = () => {
  showConfirm.value = false
}

onMounted(async () => {
  try {
    const defaultReceiver = await getDefaultReceiver()
    if (defaultReceiver) {
      receiver.value = defaultReceiver
      return
    }
  } catch (err) {
    console.error('Failed to fetch default receiver:', err)
  }
  console.warn('No default receiver configured')
})
</script>

<template>
  <div class="order-page">
    <!-- ── CONFIRM DIALOG ── -->
    <Transition name="dialog-fade">
      <div v-if="showConfirm" class="dialog-overlay" @click.self="cancelSubmit">
        <div class="dialog-box">
          <div class="dialog-icon"><i class="fas fa-paper-plane"></i></div>
          <h3 class="dialog-title">确认提交</h3>
          <p class="dialog-body">
            共 <strong>{{ samples.length }}</strong> 个样本条目，确认提交后将发送至平台。
          </p>
          <div class="dialog-meta">
            <span><i class="fas fa-user"></i> {{ collaborator.name || '—' }}</span>
            <span><i class="fas fa-university"></i> {{ collaborator.institution || '—' }}</span>
          </div>
          <div class="dialog-actions">
            <button class="btn-ghost-outline" @click="cancelSubmit">
              <i class="fas fa-times"></i>
              取消
            </button>
            <button class="btn-primary" @click="confirmSubmit">
              <i class="fas fa-check"></i>
              确认提交
            </button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- background -->
    <div class="page-bg">
      <div class="blob blob-1"></div>
      <div class="blob blob-2"></div>
      <div class="grid-overlay"></div>
    </div>

    <!-- ── NAVBAR ── -->
    <nav class="navbar">
      <div class="nav-inner">
        <button class="back-btn" @click="router.push({ name: 'index' })">
          <i class="fas fa-arrow-left"></i>
          返回首页
        </button>
        <div class="brand">
          <i class="fas fa-dna brand-icon"></i>
          <span>MPSP <span class="brand-highlight">Portal</span></span>
        </div>
      </div>
    </nav>

    <!-- ── PAGE HEADER ── -->
    <header class="page-header">
      <div class="header-inner">
        <div class="section-tag">
          <i class="fas fa-file-medical-alt"></i>
          样本提交
        </div>
        <h1 class="page-title">发送样本信息单</h1>
        <p class="page-sub">请填写合作者信息及样本明细，提交后平台会尽快与您联系确认。</p>
      </div>
    </header>

    <main class="main">
      <!-- success state -->
      <div v-if="submitted" class="success-wrap">
        <div class="success-card">
          <div class="success-icon"><i class="fas fa-check-circle"></i></div>
          <h2>提交成功</h2>
          <p>感谢您的样本申请，平台工作人员将尽快与您联系确认详情。</p>
          <button class="btn-primary" @click="router.push({ name: 'index' })">
            <i class="fas fa-home"></i>
            返回首页
          </button>
        </div>
      </div>

      <template v-else>
        <!-- ── SECTION 1: Collaborator Info ── -->
        <section class="form-section">
          <div class="form-section-header">
            <div class="step-num">01</div>
            <div>
              <h2 class="form-section-title">合作者信息</h2>
              <p class="form-section-desc">请填写负责本次送样的合作者基本信息</p>
            </div>
          </div>
          <div class="info-card">
            <div class="field-grid">
              <div class="field">
                <label>姓名 <span class="required">*</span></label>
                <input v-model="collaborator.name" type="text" placeholder="请输入姓名" />
              </div>
              <div class="field">
                <label>送样单位 <span class="required">*</span></label>
                <input v-model="collaborator.institution" type="text" placeholder="请输入所在单位或机构" />
              </div>
              <div class="field">
                <label>联系电话 <span class="required">*</span></label>
                <input v-model="collaborator.phone" type="tel" placeholder="请输入手机号码" />
              </div>
              <div class="field">
                <label>联系邮箱 <span class="required">*</span></label>
                <input v-model="collaborator.email" type="email" placeholder="请输入电子邮箱" />
              </div>
            </div>
          </div>
        </section>

        <!-- ── SECTION 2: Sample Table ── -->
        <section class="form-section">
          <div class="form-section-header">
            <div class="step-num">02</div>
            <div>
              <h2 class="form-section-title">送样信息单</h2>
              <p class="form-section-desc">请逐行填写每个样本的详细信息，可添加多个条目</p>
            </div>
          </div>

          <div class="table-wrap">
            <div class="table-scroll">
              <table class="sample-table">
                <thead>
                  <tr>
                    <th class="col-id">#</th>
                    <th class="col-name">样本原始名称 <span class="required">*</span></th>
                    <th class="col-type">样本类型 <span class="required">*</span></th>
                    <th class="col-species">物种 <span class="required">*</span></th>
                    <th class="col-tube">管数 <span class="required">*</span></th>
                    <th class="col-conc">浓度(ng/μL)</th>
                    <th class="col-vol">体积(μL)</th>
                    <th class="col-test">检测项目 <span class="required">*</span></th>
                    <th class="col-platform">测序平台 <span class="required">*</span></th>
                    <th class="col-group">分组情况(差异表达分析必填)</th>
                    <th class="col-remark">备注</th>
                    <th class="col-action"></th>
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="(sample, index) in samples" :key="sample.id" :class="{ 'example-row': index === 0 }">
                    <td class="col-id">
                      <span class="id-badge">{{ sample.id }}</span>
                    </td>
                    <td><input v-model="sample.originalName" type="text" placeholder="样本名称" /></td>
                    <td>
                      <select v-model="sample.sampleType">
                        <option value="">请选择</option>
                        <option v-for="option in SAMPLE_TYPE_OPTIONS" :value="option">
                          {{ option }}
                        </option>
                      </select>
                    </td>
                    <td><input v-model="sample.species" type="text" placeholder="物种" /></td>
                    <td><input v-model="sample.tubeCount" type="number" min="1" placeholder="管数" /></td>
                    <td><input v-model="sample.concentration" type="number" min="0" placeholder="ng/μL" /></td>
                    <td><input v-model="sample.volume" type="number" min="0" placeholder="μL" /></td>
                    <td>
                      <select v-model="sample.testItem">
                        <option value="">请选择</option>
                        <option v-for="option in TEST_ITEM_OPTIONS" :value="option">
                          {{ option }}
                        </option>
                      </select>
                    </td>
                    <td><input v-model="sample.platform" type="text" placeholder="测序平台" /></td>
                    <td><input v-model="sample.grouping" type="text" placeholder="如：对照组/实验组" /></td>
                    <td><input v-model="sample.remark" type="text" placeholder="备注" /></td>
                    <td class="col-action">
                      <button class="del-btn" title="删除" @click="removeSample(index)">
                        <i class="fas fa-trash-alt"></i>
                      </button>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>

            <div class="table-footer">
              <button class="btn-add" @click="addSample">
                <i class="fas fa-plus"></i>
                添加样本条目
              </button>
              <span class="sample-count">共 {{ samples.length }} 个样本</span>
            </div>
          </div>

          <!-- ── NOTES ── -->
          <div class="notes">
            <div class="notes-title">
              <i class="fas fa-info-circle"></i>
              注意事项
            </div>
            <ul class="notes-list">
              <li><span class="required">*</span> 为必填项</li>
              <li>
                <span class="field-name">样本原始名称</span>
                只能包含字母、数字和符号下划线 <code>_</code>
              </li>
            </ul>
          </div>
        </section>

        <!-- ── SECTION 3: Receiver Info ── -->
        <section class="form-section">
          <div class="form-section-header">
            <div class="step-num">03</div>
            <div>
              <h2 class="form-section-title">样本接收者信息</h2>
              <p class="form-section-desc">请将样本快递至如下地址信息，或与平台工作人员确认</p>
            </div>
          </div>
          <div class="info-card">
            <div class="field-grid">
              <div class="field">
                <label>姓名</label>
                <input v-model="receiver.name" type="text" placeholder="请输入姓名" disabled />
              </div>
              <div class="field">
                <label>送样地址</label>
                <input v-model="receiver.institution" type="text" placeholder="请输入所在单位或机构" />
              </div>
              <div class="field">
                <label>联系电话</label>
                <input v-model="receiver.phone" type="tel" placeholder="请输入手机号码" />
              </div>
              <div class="field">
                <label>联系邮箱</label>
                <input v-model="receiver.email" type="email" placeholder="请输入电子邮箱" />
              </div>
            </div>
          </div>
        </section>

        <!-- ── SUBMIT ── -->
        <div class="submit-row">
          <button class="btn-ghost-outline" @click="router.push({ name: 'index' })">
            <i class="fas fa-times"></i>
            取消
          </button>
          <button class="btn-primary submit-btn" @click="handleSubmit">
            <i class="fas fa-paper-plane"></i>
            提交样本信息单
          </button>
        </div>
      </template>
    </main>

    <!-- ── FOOTER ── -->
    <footer class="footer">
      <div class="footer-inner">
        <div class="footer-brand">
          <i class="fas fa-dna"></i>
          <span>MPSP Portal</span>
        </div>
        <div class="footer-copy">© 2026 复旦大学人类表型组研究院 · 分子表型核酸组学平台</div>
      </div>
    </footer>
  </div>
</template>

<style lang="scss" scoped>
// ── tokens (same as Index.vue) ──
$primary: #1a6dbb;
$primary-light: #4fc3f7;
$dark: #0c2048;
$darker: #07132b;
$text: #e2eaf4;
$muted: rgba(255, 255, 255, 0.65);
$card-bg: rgba(255, 255, 255, 0.06);
$card-border: rgba(255, 255, 255, 0.12);
$radius: 16px;
$input-bg: rgba(255, 255, 255, 0.07);
$input-border: rgba(255, 255, 255, 0.18);
$input-focus: rgba(79, 195, 247, 0.5);

// ── base ──
.order-page {
  background: $darker;
  color: $text;
  font-family: 'Inter', sans-serif;
  min-height: 100vh;
  overflow-x: hidden;
}

// ── background ──
.page-bg {
  position: fixed;
  inset: 0;
  z-index: 0;
  pointer-events: none;
  overflow: hidden;
}
.blob {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.18;
}
.blob-1 {
  width: 500px;
  height: 500px;
  background: $primary;
  top: -100px;
  right: -80px;
}
.blob-2 {
  width: 350px;
  height: 350px;
  background: $primary-light;
  bottom: 5%;
  left: -60px;
}
.grid-overlay {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(rgba(255, 255, 255, 0.025) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255, 255, 255, 0.025) 1px, transparent 1px);
  background-size: 60px 60px;
}

// ── navbar ──
.navbar {
  position: sticky;
  top: 0;
  z-index: 200;
  background: rgba(7, 19, 43, 0.88);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid $card-border;
}
.nav-inner {
  max-width: 1200px;
  margin: 0 auto;
  padding: 14px 24px;
  display: flex;
  align-items: center;
  gap: 20px;
}
.back-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  background: transparent;
  border: 1px solid $card-border;
  color: $muted;
  padding: 7px 16px;
  border-radius: 50px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.25s;
  &:hover {
    color: white;
    border-color: rgba(255, 255, 255, 0.3);
    background: $card-bg;
  }
}
.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 1.2rem;
  font-weight: 700;
  margin-left: auto;
}
.brand-icon {
  color: $primary-light;
  font-size: 1.3rem;
}
.brand-highlight {
  color: $primary-light;
}

// ── page header ──
.page-header {
  position: relative;
  z-index: 10;
  padding: 60px 24px 40px;
  text-align: center;
  border-bottom: 1px solid $card-border;
  background: rgba(255, 255, 255, 0.02);
}
.header-inner {
  max-width: 700px;
  margin: 0 auto;
}
.section-tag {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: $primary-light;
  font-size: 0.8rem;
  font-weight: 700;
  letter-spacing: 2px;
  text-transform: uppercase;
  background: rgba(79, 195, 247, 0.12);
  border: 1px solid rgba(79, 195, 247, 0.3);
  padding: 5px 16px;
  border-radius: 50px;
  margin-bottom: 20px;
}
.page-title {
  font-size: 2.6rem;
  font-weight: 800;
  margin-bottom: 14px;
  line-height: 1.2;
}
.page-sub {
  color: $muted;
  font-size: 1rem;
  line-height: 1.7;
}

// ── main ──
.main {
  position: relative;
  z-index: 10;
  max-width: 1200px;
  margin: 0 auto;
  padding: 48px 24px 80px;
}

// ── form sections ──
.form-section {
  margin-bottom: 48px;
}
.form-section-header {
  display: flex;
  align-items: flex-start;
  gap: 20px;
  margin-bottom: 24px;
}
.step-num {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  border-radius: 14px;
  background: linear-gradient(135deg, rgba(79, 195, 247, 0.2), rgba(26, 109, 187, 0.2));
  border: 1px solid rgba(79, 195, 247, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.85rem;
  font-weight: 800;
  color: $primary-light;
  letter-spacing: 1px;
}
.form-section-title {
  font-size: 1.3rem;
  font-weight: 700;
  margin-bottom: 4px;
}
.form-section-desc {
  color: $muted;
  font-size: 0.9rem;
}

// ── info card & field grid ──
.info-card {
  background: $card-bg;
  border: 1px solid $card-border;
  border-radius: $radius;
  padding: 32px;
  backdrop-filter: blur(8px);
}
.field-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 24px;
}
.field {
  display: flex;
  flex-direction: column;
  gap: 8px;
  label {
    font-size: 0.875rem;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.85);
  }
  .required {
    color: #f87171;
    margin-left: 2px;
  }
  input {
    background: $input-bg;
    border: 1px solid $input-border;
    border-radius: 10px;
    color: $text;
    padding: 11px 14px;
    font-size: 0.95rem;
    font-family: inherit;
    transition:
      border-color 0.2s,
      box-shadow 0.2s;
    outline: none;
    &::placeholder {
      color: rgba(255, 255, 255, 0.3);
    }
    &:focus {
      border-color: $input-focus;
      box-shadow: 0 0 0 3px rgba(79, 195, 247, 0.12);
    }
  }
}

// ── table ──
.table-wrap {
  background: $card-bg;
  border: 1px solid $card-border;
  border-radius: $radius;
  overflow: hidden;
  backdrop-filter: blur(8px);
}
.table-scroll {
  overflow-x: auto;

  // custom scrollbar
  &::-webkit-scrollbar {
    height: 6px;
  }
  &::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.04);
    border-radius: 0 0 $radius $radius;
  }
  &::-webkit-scrollbar-thumb {
    background: rgba(79, 195, 247, 0.35);
    border-radius: 3px;
    &:hover {
      background: rgba(79, 195, 247, 0.6);
    }
  }
  scrollbar-width: thin;
  scrollbar-color: rgba(79, 195, 247, 0.35) rgba(255, 255, 255, 0.04);
}
.sample-table {
  width: 100%;
  border-collapse: collapse;
  min-width: 1400px;

  thead tr {
    background: rgba(79, 195, 247, 0.08);
    border-bottom: 1px solid $card-border;
  }
  th {
    padding: 14px 12px;
    font-size: 0.8rem;
    font-weight: 700;
    color: $primary-light;
    text-align: left;
    white-space: nowrap;
    letter-spacing: 0.5px;
  }
  tbody tr {
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    transition: background 0.2s;
    &:last-child {
      border-bottom: none;
    }
    &:hover {
      background: rgba(255, 255, 255, 0.04);
    }
    &.example-row {
      background: rgba(79, 195, 247, 0.04);
    }
  }
  td {
    padding: 10px 10px;
    vertical-align: middle;
  }
  input,
  select {
    width: 100%;
    background: $input-bg;
    border: 1px solid $input-border;
    border-radius: 8px;
    color: $text;
    padding: 8px 10px;
    font-size: 0.85rem;
    font-family: inherit;
    outline: none;
    transition:
      border-color 0.2s,
      box-shadow 0.2s,
      background 0.2s;
    &::placeholder {
      color: rgba(255, 255, 255, 0.25);
    }
    &:focus {
      border-color: $input-focus;
      box-shadow: 0 0 0 2px rgba(79, 195, 247, 0.1);
      background: rgba(255, 255, 255, 0.1);
    }
  }
  select option {
    background: #0f2040;
    color: $text;
  }

  // number inputs: hide native spinners, show tinted right border as visual cue
  input[type='number'] {
    appearance: textfield;
    -moz-appearance: textfield;
    padding-right: 8px;
    border-right: 3px solid rgba(79, 195, 247, 0.3);
    border-radius: 8px;
    text-align: right;
    font-variant-numeric: tabular-nums;
    letter-spacing: 0.02em;
    &::-webkit-inner-spin-button,
    &::-webkit-outer-spin-button {
      appearance: none;
      -webkit-appearance: none;
    }
    &:focus {
      border-right-color: $primary-light;
    }
  }
}

// column widths
.col-id {
  width: 56px;
}
.col-name {
  min-width: 140px;
}
.col-type {
  min-width: 130px;
}
.col-species {
  min-width: 100px;
}
.col-tube {
  min-width: 90px;
}
.col-conc {
  min-width: 110px;
}
.col-vol {
  min-width: 90px;
}
.col-test {
  min-width: 200px;
}
.col-platform {
  min-width: 200px;
}
.col-group {
  min-width: 130px;
}
.col-remark {
  min-width: 140px;
}
.col-action {
  width: 48px;
  text-align: center;
}

.id-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 8px;
  background: rgba(79, 195, 247, 0.15);
  border: 1px solid rgba(79, 195, 247, 0.25);
  font-size: 0.8rem;
  font-weight: 700;
  color: $primary-light;
}
.del-btn {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  background: rgba(248, 113, 113, 0.1);
  border: 1px solid rgba(248, 113, 113, 0.25);
  color: #f87171;
  font-size: 0.8rem;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  &:hover {
    background: rgba(248, 113, 113, 0.25);
    border-color: rgba(248, 113, 113, 0.5);
  }
}

.table-footer {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 16px 20px;
  border-top: 1px solid $card-border;
}
.btn-add {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  background: rgba(79, 195, 247, 0.12);
  border: 1px dashed rgba(79, 195, 247, 0.4);
  color: $primary-light;
  padding: 9px 20px;
  border-radius: 10px;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.25s;
  font-family: inherit;
  &:hover {
    background: rgba(79, 195, 247, 0.2);
    border-color: rgba(79, 195, 247, 0.65);
  }
}
.sample-count {
  font-size: 0.85rem;
  color: $muted;
  margin-left: auto;
}

// ── notes ──
.notes {
  margin-top: 16px;
  // background: rgba(79, 195, 247, 0.05);
  // border: 1px solid rgba(79, 195, 247, 0.18);
  // border-radius: 12px;
  // padding: 16px 20px;
}
.notes-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.85rem;
  font-weight: 700;
  color: $primary-light;
  margin-bottom: 10px;
  letter-spacing: 0.3px;
  i {
    font-size: 0.9rem;
  }
}
.notes-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
  li {
    font-size: 0.85rem;
    color: $muted;
    display: flex;
    align-items: baseline;
    gap: 6px;
    &::before {
      content: '·';
      color: $primary-light;
      font-weight: 700;
      flex-shrink: 0;
    }
  }
}
.field-name {
  color: rgba(255, 255, 255, 0.9);
  font-weight: 600;
}
code {
  background: rgba(79, 195, 247, 0.12);
  border: 1px solid rgba(79, 195, 247, 0.25);
  color: $primary-light;
  padding: 1px 6px;
  border-radius: 5px;
  font-size: 0.82rem;
  font-family: 'Courier New', monospace;
}

// ── submit row ──
.submit-row {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 16px;
  margin-top: 16px;
  padding-top: 32px;
  border-top: 1px solid $card-border;
}
.btn-ghost-outline {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: $muted;
  background: transparent;
  border: 1px solid $card-border;
  padding: 14px 28px;
  font-size: 0.95rem;
  font-weight: 600;
  border-radius: 50px;
  cursor: pointer;
  font-family: inherit;
  transition: all 0.25s;
  &:hover {
    color: white;
    border-color: rgba(255, 255, 255, 0.3);
    background: $card-bg;
  }
}
.btn-primary {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  background: linear-gradient(135deg, $primary-light 0%, $primary 100%);
  color: white;
  border: none;
  padding: 14px 36px;
  font-size: 1rem;
  font-weight: 600;
  border-radius: 50px;
  cursor: pointer;
  font-family: inherit;
  transition: all 0.3s;
  box-shadow: 0 8px 24px rgba(26, 109, 187, 0.4);
  &:hover {
    transform: translateY(-3px);
    box-shadow: 0 14px 32px rgba(26, 109, 187, 0.55);
  }
}
.submit-btn {
  padding: 16px 44px;
  font-size: 1.05rem;
}

// ── success state ──
.success-wrap {
  display: flex;
  justify-content: center;
  padding: 60px 0;
}
.success-card {
  background: $card-bg;
  border: 1px solid $card-border;
  border-radius: $radius;
  padding: 56px 48px;
  text-align: center;
  max-width: 480px;
  width: 100%;
}
.success-icon {
  font-size: 4rem;
  color: #34d399;
  margin-bottom: 24px;
}
.success-card h2 {
  font-size: 1.8rem;
  font-weight: 800;
  margin-bottom: 14px;
}
.success-card p {
  color: $muted;
  line-height: 1.7;
  margin-bottom: 32px;
}

// ── confirm dialog ──
.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 500;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}
.dialog-box {
  background: #0f2040;
  border: 1px solid rgba(79, 195, 247, 0.25);
  border-radius: 20px;
  padding: 40px 36px;
  max-width: 440px;
  width: 100%;
  box-shadow: 0 24px 60px rgba(0, 0, 0, 0.5);
  text-align: center;
}
.dialog-icon {
  width: 60px;
  height: 60px;
  border-radius: 16px;
  background: linear-gradient(135deg, rgba(79, 195, 247, 0.2), rgba(26, 109, 187, 0.2));
  border: 1px solid rgba(79, 195, 247, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  color: $primary-light;
  margin: 0 auto 20px;
}
.dialog-title {
  font-size: 1.3rem;
  font-weight: 800;
  margin-bottom: 12px;
}
.dialog-body {
  color: $muted;
  font-size: 0.95rem;
  line-height: 1.7;
  margin-bottom: 20px;
  strong {
    color: white;
    font-weight: 700;
  }
}
.dialog-meta {
  display: flex;
  justify-content: center;
  gap: 20px;
  margin-bottom: 28px;
  span {
    display: flex;
    align-items: center;
    gap: 7px;
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.6);
    i {
      color: $primary-light;
      font-size: 0.8rem;
    }
  }
}
.dialog-actions {
  display: flex;
  gap: 12px;
  button {
    flex: 1;
    justify-content: center;
  }
}

// dialog transition
.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 0.2s ease;
  .dialog-box {
    transition:
      transform 0.2s ease,
      opacity 0.2s ease;
  }
}
.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
  .dialog-box {
    transform: scale(0.95);
    opacity: 0;
  }
}

// ── footer ──
.footer {
  position: relative;
  z-index: 10;
  border-top: 1px solid $card-border;
  padding: 28px 24px;
}
.footer-inner {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}
.footer-brand {
  display: flex;
  align-items: center;
  gap: 10px;
  font-weight: 700;
  color: $primary-light;
}
.footer-copy {
  font-size: 0.82rem;
  color: $muted;
}

// ── responsive ──
@media (max-width: 768px) {
  .page-title {
    font-size: 2rem;
  }
  .field-grid {
    grid-template-columns: 1fr;
  }
  .submit-row {
    flex-direction: column;
    align-items: stretch;
    button {
      width: 100%;
      justify-content: center;
    }
  }
}
</style>
