<template>
  <VaModal v-if="!!project" v-model="model" size="large" no-outside-dismiss hide-default-actions>
    <template #header>
      <div class="modal-header">
        <VaIcon name="folder_open" size="1.4rem" color="primary" />
        <div>
          <div class="modal-title">{{ STAGES[project.stage].label }} · {{ project.id }}</div>
          <div class="modal-subtitle">{{ project.createdAt }}</div>
        </div>

        <VaChip size="small" :color="priorityColor(project.priority)" outline>
          {{ priorityLabel(project.priority) }}
        </VaChip>
      </div>
    </template>

    <div>
      <!-- Details grid -->
      <div class="detail-grid">
        <div class="detail-item">
          <VaIcon name="domain" size="1.1rem" color="primary" />
          <div>
            <div class="detail-label">送样单位</div>
            <div class="detail-value">{{ project.institution }}</div>
          </div>
        </div>
        <div class="detail-item">
          <VaIcon name="person" size="1.1rem" color="primary" />
          <div>
            <div class="detail-label">联系人</div>
            <div class="detail-value">{{ project.contact }}</div>
          </div>
        </div>
        <div class="detail-item">
          <VaIcon name="science" size="1.1rem" color="primary" />
          <div>
            <div class="detail-label">样本类型</div>
            <div class="detail-value">{{ project.sampleType }}</div>
          </div>
        </div>
        <div class="detail-item">
          <VaIcon name="biotech" size="1.1rem" color="primary" />
          <div>
            <div class="detail-label">检测项目</div>
            <div class="detail-value">{{ project.testItem }}</div>
          </div>
        </div>
        <div class="detail-item detail-item--clickable" @click="showSamplesModal = true">
          <VaIcon name="inventory_2" size="1.1rem" color="primary" />
          <div style="flex: 1">
            <div class="detail-label">样本数量</div>
            <div class="detail-value">{{ 0 }} 份</div>
          </div>
          <VaIcon name="open_in_new" size="1rem" color="secondary" />
        </div>
        <div class="detail-item">
          <VaIcon name="calendar_today" size="1.1rem" color="primary" />
          <div>
            <div class="detail-label">提交日期</div>
            <div class="detail-value">{{ project.createdAt }}</div>
          </div>
        </div>
      </div>

      <VaDivider />

      <!-- 上传提取照片 -->
      <div class="upload-section">
        <div class="upload-label">
          <VaIcon name="add_a_photo" size="1.1rem" color="primary" />
          <span>上传提取照片</span>
        </div>
        <VaFileUpload
          v-model="extractionPhotos"
          type="gallery"
          file-types=".jpg,.jpeg,.png,.webp,.tiff"
          dropzone
          dropzone-text="拖拽图片至此或点击上传"
          upload-button-text="选择图片"
        />
      </div>

      <VaDivider />

      <!-- 上传质检报告 -->
      <div class="upload-section">
        <div class="upload-label">
          <VaIcon name="description" size="1.1rem" color="primary" />
          <span>上传质检报告</span>
        </div>
        <VaFileUpload
          v-model="qcReports"
          file-types=".pdf,.xlsx,.xls,.docx,.doc,.csv"
          dropzone
          dropzone-text="拖拽文件至此或点击上传（支持 PDF / Excel / Word）"
          upload-button-text="选择文件"
        />
      </div>
    </div>

    <SampleDetailsModal
      v-model="showSamplesModal"
      :samples="editSamples"
      readonly
      @update:samples="editSamples = $event"
    />

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
import { PropType, ref } from 'vue'
import { Project, SampleEntry, STAGES } from '../types'
import SampleDetailsModal from './SampleDetailsModal.vue'

const model = defineModel({
  type: Boolean,
  required: true,
})

defineProps({
  project: {
    type: Object as PropType<Project | null>,
    default: null,
  },
})

const showSamplesModal = ref(false)
const extractionPhotos = ref<File[]>([])
const qcReports = ref<File[]>([])
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
