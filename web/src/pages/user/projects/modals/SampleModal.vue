<template>
  <VaModal v-if="!!project" v-model="model" size="large" no-outside-dismiss hide-default-actions>
    <template #header>
      <div class="modal-header">
        <VaIcon name="folder_open" size="1.4rem" color="primary" />
        <div>
          <div class="modal-title">{{ STAGES[project.stage].label }} · {{ project.id }}</div>
          <div class="modal-subtitle">{{ project.createdAt }}</div>
        </div>
      </div>
    </template>

    <div>
      <!-- Details grid -->
      <div class="detail-grid">
        <VaInput v-model="editInstitution" label="送样单位" placeholder="请输入送样单位" />
        <VaInput v-model="editContact" label="合作者姓名" placeholder="请输入合作者姓名" />
        <VaInput v-model="editPhone" label="合作者联系电话" placeholder="请输入联系电话" type="tel" />
        <VaInput v-model="editEmail" label="合作者联系邮箱" placeholder="请输入联系邮箱" type="email" />
        <VaInput v-model="editReceiverName" label="收样人姓名" placeholder="请输入收样人姓名" />
        <VaInput v-model="editReceiverAddress" label="收样人地址" placeholder="请输入收样人地址" />
        <VaInput v-model="editReceiverPhone" label="收样人联系电话" placeholder="请输入收样人联系电话" type="tel" />
        <VaInput v-model="editReceiverEmail" label="收样人联系邮箱" placeholder="请输入收样人联系邮箱" type="email" />
        <div class="detail-item detail-item--clickable" @click="showSamplesModal = true">
          <VaIcon name="inventory_2" size="1.1rem" color="primary" />
          <div style="flex: 1">
            <div class="detail-label">样本数量</div>
            <div class="detail-value">{{ editSamples.length }} 份</div>
          </div>
          <VaIcon name="open_in_new" size="1rem" color="secondary" />
        </div>
        <div class="detail-item">
          <VaButton preset="secondary" @click="">生成入库样本信息确认单</VaButton>
        </div>
      </div>

      <SampleDetailsModal v-model="showSamplesModal" :samples="editSamples" @update:samples="editSamples = $event" />

      <VaDivider />

      <div class="detail-grid">
        <VaSelect v-model="editPriority" label="优先级" :options="priorityOptions" value-by="value" text-by="label" />
      </div>

      <div class="detail-grid">
        <VaSelect
          v-model="editProcessType"
          label="流程选择"
          :options="processTypeOptions"
          value-by="value"
          text-by="label"
          placeholder="请选择流程"
        />
        <!-- Operator assignments (shown based on process type) -->
        <div v-if="editProcessType" class="operator-container">
          <VaSelect
            v-if="editProcessType === 'nucleicAcid'"
            v-model="editNucleicAcidOperator"
            label="核酸提取操作员"
            :options="OPERATOR_OPTIONS"
            placeholder="请选择操作员"
          />
          <VaSelect
            v-if="editProcessType === 'nucleicAcid' || editProcessType === 'library'"
            v-model="editLibraryOperator"
            label="建库操作员"
            :options="OPERATOR_OPTIONS"
            placeholder="请选择操作员"
          />
          <VaSelect
            v-model="editSequencingOperator"
            label="测序操作员"
            :options="OPERATOR_OPTIONS"
            placeholder="请选择操作员"
          />
        </div>
      </div>
      <VaDivider />

      <div class="upload-section">
        <div class="upload-label">
          <VaIcon name="mark_email_read" size="1.1rem" color="primary" />
          <span>上传客户确认邮件</span>
        </div>
        <VaFileUpload
          v-model="confirmationEmails"
          file-types=".eml,.msg,.pdf,.jpg,.jpeg,.png"
          dropzone
          dropzone-text="拖拽文件至此或点击上传（支持 .eml / .msg / .pdf / 图片）"
          upload-button-text="选择文件"
        />
      </div>
    </div>

    <div class="flex justify-end mt-2 gap-2">
      <VaButton preset="secondary" color="secondary" @click="model = false"> 取消 </VaButton>
      <VaButton preset="primary" @click="model = false"> 确认 </VaButton>
      <VaButton color="primary" icon="arrow_forward" @click="">
        推进至「{{ STAGES[project.stage + 1]?.label }}」
      </VaButton>
    </div>
  </VaModal>
</template>

<script lang="ts" setup>
import { ref, watch, PropType } from 'vue'
import { Project, SampleEntry, STAGES, STAGE_STEPS } from '../types'
import SampleDetailsModal from './SampleDetailsModal.vue'

const model = defineModel({
  type: Boolean,
  required: true,
})

const props = defineProps({
  project: {
    type: Object as PropType<Project | null>,
    default: null,
  },
})

const emit = defineEmits<{
  (
    e: 'save',
    updated: Pick<
      Project,
      | 'id'
      | 'institution'
      | 'contact'
      | 'phone'
      | 'email'
      | 'receiverName'
      | 'receiverAddress'
      | 'receiverPhone'
      | 'receiverEmail'
      | 'priority'
      | 'samples'
      | 'processType'
      | 'nucleicAcidOperator'
      | 'libraryOperator'
      | 'sequencingOperator'
    >,
  ): void
}>()

const editInstitution = ref('')
const editContact = ref('')
const editPhone = ref('')
const editEmail = ref('')
const editReceiverName = ref('')
const editReceiverAddress = ref('')
const editReceiverPhone = ref('')
const editReceiverEmail = ref('')
const editPriority = ref<Project['priority']>('medium')
const editProcessType = ref<Project['processType']>('')
const editNucleicAcidOperator = ref('')
const editLibraryOperator = ref('')
const editSequencingOperator = ref('')
const confirmationEmails = ref<File[]>([])

const editSamples = ref<SampleEntry[]>([
  {
    id: 1,
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
  },
])
const showSamplesModal = ref(false)

const priorityOptions = [
  { label: '高优先级', value: 'high' as const },
  { label: '中优先级', value: 'medium' as const },
  { label: '低优先级', value: 'low' as const },
]

const processTypeOptions = [
  { label: '核酸提取流程', value: 'nucleicAcid' as const },
  { label: '建库流程', value: 'library' as const },
  { label: '测序流程', value: 'sequencing' as const },
]

const OPERATOR_OPTIONS = ['张工', '李工', '王工', '赵工', '陈工', '刘工', '吴工', '孙工', '周工', '郑工']

watch(
  () => props.project,
  (p) => {
    editInstitution.value = p?.institution ?? ''
    editContact.value = p?.contact ?? ''
    editPhone.value = p?.phone ?? ''
    editEmail.value = p?.email ?? ''
    editReceiverName.value = p?.receiverName ?? ''
    editReceiverAddress.value = p?.receiverAddress ?? ''
    editReceiverPhone.value = p?.receiverPhone ?? ''
    editReceiverEmail.value = p?.receiverEmail ?? ''
    editPriority.value = p?.priority ?? 'medium'
    editProcessType.value = p?.processType ?? ''
    editNucleicAcidOperator.value = p?.nucleicAcidOperator ?? ''
    editLibraryOperator.value = p?.libraryOperator ?? ''
    editSequencingOperator.value = p?.sequencingOperator ?? ''
    confirmationEmails.value = []
  },
  { immediate: true },
)

const handleSave = () => {
  if (props.project) {
    emit('save', {
      id: props.project.id,
      institution: editInstitution.value,
      contact: editContact.value,
      phone: editPhone.value,
      email: editEmail.value,
      receiverName: editReceiverName.value,
      receiverAddress: editReceiverAddress.value,
      receiverPhone: editReceiverPhone.value,
      receiverEmail: editReceiverEmail.value,
      priority: editPriority.value,
      processType: editProcessType.value,
      nucleicAcidOperator: editNucleicAcidOperator.value,
      libraryOperator: editLibraryOperator.value,
      sequencingOperator: editSequencingOperator.value,
      samples: editSamples.value,
    })
  }
  model.value = false
}

const priorityColor = (p: string) =>
  ({ high: 'danger', medium: 'warning', low: 'success' })[p as 'high' | 'medium' | 'low'] ?? 'secondary'
const priorityLabel = (p: string) =>
  ({ high: '高优先级', medium: '中优先级', low: '低优先级' })[p as 'high' | 'medium' | 'low'] ?? ''
</script>

<style lang="scss">
.modal-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  padding: 4px 0;
}

.modal-title {
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--va-text-primary);
  line-height: 1.3;
}

.modal-subtitle {
  font-size: 0.82rem;
  color: var(--va-secondary);
  margin-top: 2px;
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
  margin: 16px 0;

  @media (max-width: 600px) {
    grid-template-columns: 1fr;
  }
}

.detail-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 12px 14px;
  background: var(--va-background-element);
  border-radius: 8px;

  &--clickable {
    cursor: pointer;
    transition:
      background 0.15s,
      box-shadow 0.15s;

    &:hover {
      background: var(--va-background-secondary);
      box-shadow: 0 0 0 2px var(--va-primary);
    }
  }
}

.detail-label {
  font-size: 0.75rem;
  color: var(--va-secondary);
  margin-bottom: 3px;
}

.detail-value {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--va-text-primary);
}

.operator-container > .va-select {
  margin-bottom: 16px;
}
.operator-container > .va-select:last-child {
  margin-bottom: 0;
}

.upload-section {
  margin: 16px 0 8px;
}

.upload-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--va-text-primary);
  margin-bottom: 10px;
}
</style>
