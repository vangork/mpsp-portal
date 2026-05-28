<script setup lang="ts">
import { onMounted } from 'vue'
import { defineVaDataTableColumns } from 'vuestic-ui'
import { useProjectsStore } from '../../../../stores/projects'
import { Pagination } from '../../../../data/pages/projects'
import { STAGES } from '../../projects/types'
import { ref } from 'vue'

const columns = defineVaDataTableColumns([
  { label: 'ID', key: 'id', sortable: true },
  { label: '机构', key: 'institution', sortable: true },
  { label: '样本数量', key: 'count', sortable: true },
  { label: '状态', key: 'stage', sortable: true },
])

const pagination = ref<Pagination>({ page: 1, perPage: 5, total: 0 })
const projectsStore = useProjectsStore()

// const { projects, isLoading, sorting } = useProjects({
//   pagination,
// })

onMounted(async () => {
  await projectsStore.refreshProjects()
})
</script>

<template>
  <VaCard>
    <VaCardTitle class="flex items-start justify-between">
      <h1 class="card-title text-secondary font-bold uppercase">Projects</h1>
      <VaButton preset="primary" size="small" to="/user/projects">View all projects</VaButton>
    </VaCardTitle>
    <VaCardContent>
      <div v-if="projectsStore.projects.length > 0">
        <VaDataTable :items="projectsStore.projects.slice(0, 6)" :columns="columns" :loading="projectsStore.loading">
          <template #cell(id)="{ rowData }">
            {{ rowData.id }}
          </template>
          <template #cell(institution)="{ rowData }">
            <div class="ellipsis max-w-[230px] lg:max-w-[450px]">
              {{ rowData.institution }}
            </div>
          </template>
          <template #cell(contact)="{ rowData }">
            <div class="flex items-center gap-2 ellipsis max-w-[230px]">
              {{ rowData.contact }}
            </div>
          </template>
          <template #cell(count)="{ rowData }">
            <!-- <VaAvatarGroup size="small" :options="getTeamOptions(project.team)" :max="2" /> -->
            {{ rowData.sampleCount }}
          </template>
          <template #cell(stage)="{ rowData }">
            <VaBadge :text="STAGES[rowData.stage]?.label || '未知'" :color="STAGES[rowData.stage]?.color || 'dark'" />
          </template>
        </VaDataTable>
      </div>
      <div v-else class="p-4 flex justify-center items-center text-[var(--va-secondary)]">No projects</div>
    </VaCardContent>
  </VaCard>
</template>
