<template>
  <VaModal v-if="!!project" v-model="model" size="large" close-button no-outside-dismiss>
    <template #header>
      <div class="modal-header">
        <VaIcon name="folder_open" size="1.4rem" color="primary" />
        <div>
          <div class="modal-title">{{ project.name }}</div>
          <div class="modal-subtitle">{{ project.id }} · {{ project.institution }}</div>
        </div>
      </div>
    </template>

    <div>
      <!-- Progress stepper -->
      <div class="modal-stepper-wrap">
        <VaStepper :steps="STAGE_STEPS" :model-value="project.stage" navigation-disabled />
      </div>

      <VaDivider />

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
        <div class="detail-item">
          <VaIcon name="person" size="1.1rem" color="primary" />
          <div>
            <div class="detail-label">联系人</div>
            <div class="detail-value">{{ project.contact }}</div>
          </div>
        </div>
        <div class="detail-item">
          <VaIcon name="inventory_2" size="1.1rem" color="primary" />
          <div>
            <div class="detail-label">样本数量</div>
            <div class="detail-value">{{ project.sampleCount }} 份</div>
          </div>
        </div>
        <div class="detail-item">
          <VaIcon name="calendar_today" size="1.1rem" color="primary" />
          <div>
            <div class="detail-label">提交日期</div>
            <div class="detail-value">{{ project.createdAt }}</div>
          </div>
        </div>
        <div class="detail-item">
          <VaIcon name="flag" size="1.1rem" :color="priorityColor(project.priority)" />
          <div>
            <div class="detail-label">优先级</div>
            <div class="detail-value">
              <VaChip size="small" :color="priorityColor(project.priority)" outline>
                {{ priorityLabel(project.priority) }}
              </VaChip>
            </div>
          </div>
        </div>
        <div class="detail-item">
          <VaIcon name="local_activity" size="1.1rem" color="primary" />
          <div>
            <div class="detail-label">当前阶段</div>
            <div class="detail-value">
              <VaChip size="small" :color="STAGES[project.stage].color" flat :icon="STAGES[project.stage].icon">
                {{ STAGES[project.stage].label }}
              </VaChip>
            </div>
          </div>
        </div>
      </div>

      <!-- Notes -->
      <VaAlert v-if="project.notes" color="warning" icon="notes" class="mt-4">
        {{ project.notes }}
      </VaAlert>
    </div>

    <template #footer>
      <div class="modal-footer">
        <VaButton preset="secondary" @click="model = false">关闭</VaButton>
        <VaChip v-if="project.stage === STAGES.length - 1" color="success" icon="check_circle" flat>
          已进入测序阶段
        </VaChip>
        <VaButton v-else-if="project" color="primary" icon="arrow_forward" @click="">
          推进至「{{ STAGES[project.stage + 1]?.label }}」
        </VaButton>
      </div>
    </template>
  </VaModal>
</template>

<script lang="ts" setup>
import { PropType } from 'vue'
import { Project, STAGES, STAGE_STEPS } from '../types'

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

.modal-stepper-wrap {
  margin-bottom: 20px;
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

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  width: 100%;
}
</style>
