<script lang="ts" setup>
import { ref, computed } from 'vue'
import SampleModal from './modals/SampleModal.vue'
import { Project, STAGES } from './types'
import { PROJECT_SAMPLES } from '../../../data/pages/projects'


const projects = ref<Project[]>(PROJECT_SAMPLES)

const selectedProject = ref<Project | null>(null)
const showModal = ref(false)


const openProject = (project: Project) => {
  selectedProject.value = { ...project }
  showModal.value = true
}

const projectsByStage = (stageIndex: number) => projects.value.filter((p) => p.stage === stageIndex)

const priorityColor = (p: string) => ({ high: 'danger', medium: 'warning', low: 'success' })[p as 'high' | 'medium' | 'low'] ?? 'secondary'
const priorityLabel = (p: string) => ({ high: '高优先级', medium: '中优先级', low: '低优先级' })[p as 'high' | 'medium' | 'low'] ?? ''

const totalProjects = computed(() => projects.value.length)
const sequencingProjects = computed(() => projects.value.filter((p) => p.stage === 5).length)
const inProgressProjects = computed(() => projects.value.filter((p) => p.stage > 0 && p.stage < 5).length)
</script>

<template>
  <div class="projects-page">
    <!-- ── PIPELINE LEGEND ── -->
    <VaCard class="pipeline-legend" outlined>
      <div class="legend-inner">
        <template v-for="(stage, idx) in STAGES" :key="idx">
          <div class="legend-step">
            <VaIcon :name="stage.icon" size="1rem" :color="stage.color" />
            <span class="legend-label">{{ stage.label }}</span>
          </div>
          <VaIcon v-if="idx < STAGES.length - 1" name="chevron_right" size="1rem" color="secondary" class="legend-arrow" />
        </template>
      </div>
    </VaCard>

    <!-- ── KANBAN BOARD ── -->
    <VaCard outlined>
      <div class="kanban-board">
        <div v-for="(stage, stageIdx) in STAGES" :key="stageIdx" class="kanban-column">
          <!-- Column header -->
          <div class="column-header" :style="{ borderTopColor: stage.colour }">
            <div class="column-header-left">
              <VaIcon :name="stage.icon" size="1.1rem" :color="stage.color" />
              <span class="column-title">{{ stage.label }}</span>
            </div>
            <VaBadge :text="String(projectsByStage(stageIdx).length)" :color="stage.color" />
          </div>

          <!-- Cards list -->
          <div class="column-body">
            <VaCard
              v-for="project in projectsByStage(stageIdx)"
              :key="project.id"
              class="project-card"
              hoverable
              @click="openProject(project)"
            >
              <!-- Top row: id + priority -->
              <div class="card-top">
                <span class="project-id">{{ project.id }}</span>
                <VaChip size="small" :color="priorityColor(project.priority)" outline>
                  {{ priorityLabel(project.priority) }}
                </VaChip>
              </div>

              <!-- Project name -->
              <div class="card-name">{{ project.name }}</div>

              <!-- Tags -->
              <div class="card-tags">
                <VaChip size="small" :color="stage.color" flat icon="science">
                  {{ project.sampleType }}
                </VaChip>
                <VaChip size="small" color="secondary" flat icon="biotech">
                  {{ project.sampleCount }} 份
                </VaChip>
              </div>

              <VaDivider style="margin: 8px 0 6px" />

              <!-- Footer: contact + date -->
              <div class="card-footer">
                <div class="card-contact">
                  <VaAvatar size="small" :color="stage.color">{{ project.contact[0] }}</VaAvatar>
                  <span class="contact-name">{{ project.contact }}</span>
                </div>
                <span class="card-date">{{ project.createdAt }}</span>
              </div>
            </VaCard>

            <!-- Empty state -->
            <div v-if="projectsByStage(stageIdx).length === 0" class="empty-column">
              <VaIcon name="inbox" size="2rem" color="secondary" />
              <p class="empty-text">暂无项目</p>
            </div>
          </div>
        </div>
      </div>
    </VaCard>

    <!-- ── PROJECT DETAIL MODAL ── -->
    <SampleModal v-model="showModal" :project="selectedProject"></SampleModal>
  </div>
</template>

<style lang="scss" scoped>
.projects-page {
  padding: 0 4px 32px;
}

// ── Header ──
.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 16px;
  margin-bottom: 20px;
  padding: 20px 24px;
  background: linear-gradient(135deg, #0c2d48 0%, var(--va-primary) 60%, #4a9fe7 100%);
  border-radius: 12px;
  color: white;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-icon-wrap {
  width: 56px;
  height: 56px;
  border-radius: 14px !important;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.page-title {
  font-size: 1.6rem;
  font-weight: 700;
  margin: 0 0 4px;
}

.page-subtitle {
  font-size: 0.9rem;
  opacity: 0.85;
  margin: 0;
}

.header-stats {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

// ── Pipeline legend ──
.pipeline-legend {
  margin-bottom: 20px;
  padding: 12px 20px !important;
}

.legend-inner {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 4px;
}

.legend-step {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 20px;
  background: var(--va-background-element);
}

.legend-label {
  font-size: 0.82rem;
  font-weight: 600;
  color: var(--va-text-primary);
}

.legend-arrow {
  opacity: 0.4;
}

// ── Kanban board ──
.kanban-board {
  display: flex;
  gap: 14px;
  padding: 15px;
  overflow-x: auto;
  padding-bottom: 12px;
  align-items: flex-start;
  // justify-content: space-between;

  &::-webkit-scrollbar {
    height: 6px;
  }
  &::-webkit-scrollbar-track {
    background: var(--va-background-element);
    border-radius: 3px;
  }
  &::-webkit-scrollbar-thumb {
    background: var(--va-primary);
    border-radius: 3px;
    opacity: 0.5;
  }
  scrollbar-width: thin;
}

.kanban-column {
  flex: 0 0 260px;
  min-width: 260px;
  display: flex;
  flex-direction: column;
  gap: 0;
}

.column-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  background: var(--va-background-secondary);
  border-radius: 10px 10px 0 0;
  border-top: 3px solid transparent;
  border-left: 1px solid var(--va-background-border);
  border-right: 1px solid var(--va-background-border);
}

.column-header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.column-title {
  font-size: 0.88rem;
  font-weight: 700;
  color: var(--va-text-primary);
}

.column-body {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px;
  background: var(--va-background-secondary);
  border-radius: 0 0 10px 10px;
  border: 1px solid var(--va-background-border);
  border-top: none;
  min-height: 120px;
}

// ── Project card ──
.project-card {
  cursor: pointer;
  transition: transform 0.15s ease, box-shadow 0.15s ease;

  &:hover {
    transform: translateY(-2px);
  }

  // override VaCard inner padding
  :deep(.va-card__content) {
    padding: 12px 14px;
  }
}

.card-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}

.project-id {
  font-size: 0.72rem;
  font-weight: 700;
  color: var(--va-secondary);
  letter-spacing: 0.5px;
  font-family: 'Courier New', monospace;
}

.card-name {
  font-size: 0.88rem;
  font-weight: 600;
  color: var(--va-text-primary);
  line-height: 1.4;
  margin-bottom: 8px;
}

.card-tags {
  display: flex;
  gap: 5px;
  flex-wrap: wrap;
  margin-bottom: 2px;
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.card-contact {
  display: flex;
  align-items: center;
  gap: 6px;
}

.contact-name {
  font-size: 0.8rem;
  color: var(--va-text-primary);
  font-weight: 500;
}

.card-date {
  font-size: 0.75rem;
  color: var(--va-secondary);
}

// ── Empty column ──
.empty-column {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px 12px;
  gap: 8px;
  opacity: 0.5;
}

.empty-text {
  font-size: 0.82rem;
  color: var(--va-secondary);
  margin: 0;
}
</style>
