<template>
  <VaModal v-model="model" size="large" no-outside-dismiss hide-default-actions>
    <template #header>
      <div class="sd-header">
        <VaIcon name="biotech" size="1.4rem" color="primary" />
        <div>
          <div class="sd-title">送样信息单</div>
          <div class="sd-subtitle">共 {{ localSamples.length }} 个样本条目</div>
        </div>
      </div>
    </template>

    <div class="sd-body">
      <div class="sd-scroll">
        <table class="sd-table">
          <thead>
            <tr>
              <th class="col-idx">#</th>
              <th class="col-name">样本原始名称</th>
              <th class="col-type">样本类型</th>
              <th class="col-species">物种</th>
              <th class="col-tube">管数</th>
              <th class="col-conc">浓度(ng/μL)</th>
              <th class="col-vol">体积(μL)</th>
              <th class="col-test">检测项目</th>
              <th class="col-platform">测序平台</th>
              <th class="col-group">分组情况</th>
              <th class="col-remark">备注</th>
              <th v-if="!readonly" class="col-action"></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(row, idx) in localSamples" :key="row.id">
              <td class="col-idx">
                <VaChip size="small" color="primary" flat>{{ idx + 1 }}</VaChip>
              </td>
              <td class="col-name">
                <VaInput v-model="row.originalName" :readonly="readonly" placeholder="样本名称" class="cell-input" />
              </td>
              <td class="col-type">
                <VaSelect
                  v-model="row.sampleType"
                  :readonly="readonly"
                  :options="SAMPLE_TYPE_OPTIONS"
                  placeholder="请选择"
                  class="cell-input"
                />
              </td>
              <td class="col-species">
                <VaInput v-model="row.species" :readonly="readonly" placeholder="物种" class="cell-input" />
              </td>
              <td class="col-tube">
                <VaInput
                  v-model="row.tubeCount"
                  :readonly="readonly"
                  type="number"
                  placeholder="管数"
                  class="cell-input"
                />
              </td>
              <td class="col-conc">
                <VaInput
                  v-model="row.concentration"
                  :readonly="readonly"
                  type="number"
                  placeholder="ng/μL"
                  class="cell-input"
                />
              </td>
              <td class="col-vol">
                <VaInput v-model="row.volume" :readonly="readonly" type="number" placeholder="μL" class="cell-input" />
              </td>
              <td class="col-test">
                <VaSelect
                  v-model="row.testItem"
                  :readonly="readonly"
                  :options="TEST_ITEM_OPTIONS"
                  placeholder="请选择"
                  class="cell-input"
                />
              </td>
              <td class="col-platform">
                <VaInput v-model="row.platform" :readonly="readonly" placeholder="测序平台" class="cell-input" />
              </td>
              <td class="col-group">
                <VaInput
                  v-model="row.grouping"
                  :readonly="readonly"
                  placeholder="如：对照组/实验组"
                  class="cell-input"
                />
              </td>
              <td class="col-remark">
                <VaInput v-model="row.remark" :readonly="readonly" placeholder="备注" class="cell-input" />
              </td>
              <td v-if="!readonly" class="col-action">
                <VaButton icon="delete" color="danger" preset="plain" size="small" @click="removeRow(idx)" />
              </td>
            </tr>
            <tr v-if="localSamples.length === 0">
              <td colspan="12" class="empty-row">
                <VaIcon name="inbox" size="1.5rem" color="secondary" />
                <span>暂无样本条目，请点击下方按钮添加</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <template #footer>
      <div v-if="!readonly" class="sd-footer">
        <VaButton icon="add" preset="secondary" @click="addRow">添加样本条目</VaButton>
        <div class="sd-footer-right">
          <!-- <span class="sd-count">共 {{ localSamples.length }} 条</span>
          <VaButton color="primary" icon="save" @click="handleSave">保存并关闭</VaButton> -->
          <VaButton preset="secondary" color="secondary" @click="model = false"> 取消 </VaButton>
          <VaButton preset="primary" @click="model = false"> 确认 </VaButton>
        </div>
      </div>
      <div v-else class="sd-footer">
        <div></div>
        <VaButton preset="primary" @click="model = false"> 好的 </VaButton>
      </div>
    </template>
  </VaModal>
</template>

<script lang="ts" setup>
import { PropType, ref, watch } from 'vue'
import { SampleEntry, SAMPLE_TYPE_OPTIONS, TEST_ITEM_OPTIONS } from '../types'

const model = defineModel({ type: Boolean, required: true })

const props = defineProps({
  samples: {
    type: Object as PropType<SampleEntry[]>,
    required: true,
  },
  readonly: {
    type: Boolean,
    default: false,
  },
})
const emit = defineEmits<{ (e: 'update:samples', val: SampleEntry[]): void }>()

const localSamples = ref<SampleEntry[]>([])
let nextId = 0

watch(
  () => props.samples,
  (s) => {
    localSamples.value = s.map((x) => ({ ...x }))
    nextId = s.length > 0 ? Math.max(...s.map((x) => x.id)) + 1 : 0
  },
  { immediate: true },
)

const addRow = () => {
  localSamples.value.push({
    id: nextId++,
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

const removeRow = (idx: number) => {
  localSamples.value.splice(idx, 1)
}

const handleSave = () => {
  emit(
    'update:samples',
    localSamples.value.map((x) => ({ ...x })),
  )
  model.value = false
}
</script>

<style lang="scss" scoped>
.sd-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 4px 0;
}

.sd-title {
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--va-text-primary);
}

.sd-subtitle {
  font-size: 0.82rem;
  color: var(--va-secondary);
  margin-top: 2px;
}

.sd-body {
  min-height: 200px;
}

.sd-scroll {
  overflow-x: auto;
  border: 1px solid var(--va-background-border);
  border-radius: 8px;

  &::-webkit-scrollbar {
    height: 6px;
  }
  &::-webkit-scrollbar-thumb {
    background: var(--va-primary);
    border-radius: 3px;
  }
  scrollbar-width: thin;
}

.sd-table {
  width: 100%;
  border-collapse: collapse;
  min-width: 1200px;

  thead tr {
    background: var(--va-background-secondary);
    border-bottom: 2px solid var(--va-background-border);
  }

  th {
    padding: 10px 8px;
    font-size: 0.78rem;
    font-weight: 700;
    color: var(--va-primary);
    text-align: left;
    white-space: nowrap;
  }

  tbody tr {
    border-bottom: 1px solid var(--va-background-border);
    transition: background 0.15s;

    &:last-child {
      border-bottom: none;
    }

    &:hover {
      background: var(--va-background-element);
    }
  }

  td {
    padding: 6px 6px;
    vertical-align: middle;
  }
}

.cell-input {
  :deep(input),
  :deep(.va-input__container) {
    font-size: 0.82rem;
  }
}

.col-idx {
  width: 48px;
  text-align: center;
}
.col-name {
  min-width: 130px;
}
.col-type {
  min-width: 130px;
}
.col-species {
  min-width: 90px;
}
.col-tube {
  min-width: 70px;
}
.col-conc {
  min-width: 100px;
}
.col-vol {
  min-width: 80px;
}
.col-test {
  min-width: 180px;
}
.col-platform {
  min-width: 110px;
}
.col-group {
  min-width: 120px;
}
.col-remark {
  min-width: 110px;
}
.col-action {
  width: 44px;
  text-align: center;
}

.empty-row {
  text-align: center;
  padding: 40px 0;
  color: var(--va-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  font-size: 0.9rem;
}

.sd-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  gap: 12px;
}

.sd-footer-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.sd-count {
  font-size: 0.85rem;
  color: var(--va-secondary);
}
</style>
