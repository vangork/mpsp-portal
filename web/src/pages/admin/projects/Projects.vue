<template>
  <h1 class="page-title font-bold">{{ t('menu.projects_details') }}</h1>
  <VaCard>
    <VaCardContent>
      <div class="flex flex-col xs:flex-row gap-2 mb-2 justify-between">
        <div class="flex flex-col md:flex-row gap-2 justify-start">
          <VaSelect
            v-model="filters.stages"
            :options="stagesSelectOptions"
            :placeholder="t('project.filter_by_stage')"
            multiple
            searchable
            highlight-matched-text
            clearable
          />
          <VaInput v-model="filters.search" :placeholder="t('vuestic.search')" clearable>
            <template #prependInner>
              <VaIcon name="search" color="secondary" size="small" />
            </template>
          </VaInput>
        </div>
      </div>

      <ProjectTable
        :items="items"
        :loading="adminProjectsStore.loading"
        :pagination="pagination"
        @editItem="showEditItemModal"
        @viewItem="showViewItemModal"
      />
    </VaCardContent>
  </VaCard>

  <VaModal
    v-slot="{ cancel, ok }"
    v-model="doShowEditItemModal"
    size="small"
    mobile-fullscreen
    hide-default-actions
    close-button
  >
    <h1 class="va-h5">{{ `${t('dna.edit_data')}: ${itemToEdit?.id}` }}</h1>
    <EditDNAForm
      :item="itemToEdit!"
      @close="cancel"
      @save="
        (item) => {
          onItemSaved(item, ok)
        }
      "
    />
    <div v-if="!!errorMsg" class="flex gap-4 w-full">
      <VaAlert dense closeable color="danger" class="w-full">
        {{ errorMsg }}
      </VaAlert>
    </div>
  </VaModal>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useModal } from 'vuestic-ui'
import { Filters, Pagination } from '../../../data/pages/projects'
import { useAdminProjectsStore } from '../../../stores/admin-projects'
import { STAGES, StageLabel, StageIndex } from '../../user/projects/types'
import { AdminProject } from './types'
import ProjectTable from './widgets/ProjectTable.vue'
import EditDNAForm from './widgets/EditDNAForm.vue'
import ViewDNAForm from './widgets/ViewDNAForm.vue'

const { t } = useI18n()

const adminProjectsStore = useAdminProjectsStore()

const pagination = ref<Pagination>({
  page: 1,
  perPage: 10,
  total: 1,
} as Pagination)

const filters = ref<Partial<Filters>>({ stages: [], search: '' })
const stagesSelectOptions = ref<{ text: StageLabel; value: StageIndex }[]>([
  ...STAGES.map((stage) => ({ text: stage.label, value: stage.idx })),
])

const filteredItems = computed(() => {
  const items = adminProjectsStore.projects.filter((i) => {
    if (
      filters.value.stages !== undefined &&
      filters.value.stages.length > 0 &&
      !filters.value.stages.find((s) => s.value === i.stage)
    ) {
      return false
    }
    if (filters.value.search) {
      const search = filters.value.search.toLowerCase()
      return i.id.toLowerCase().includes(search) || i.institution.toLowerCase().includes(search)
    }
    return true
  })
  return items
})

const items = computed(() => {
  return filteredItems.value.slice(
    (pagination.value.page - 1) * pagination.value.perPage,
    pagination.value.page * pagination.value.perPage,
  )
})

watch(
  () => filteredItems.value.length,
  (val) => {
    pagination.value.total = val
  },
  { immediate: true },
)

// watch(
//   [() => adminOmicsStore.dnaItems.length],
//   () => {
//     const userMap = new Map<number, string>()
//     adminOmicsStore.dnaItems.forEach((item) => {
//       if (!userMap.has(item.user_id)) {
//         userMap.set(item.user_id, item.username)
//       }
//     })
//     usersSelectOptions.value = Array.from(userMap.entries())
//       .map(([value, text]) => ({ text, value }))
//       .sort((a, b) => a.text.localeCompare(b.text))
//     filters.value.users = Array.from(
//       usersSelectOptions.value
//         .entries()
//         .filter(([, option]) => filters.value.users?.map((u) => u.value).includes(option.value)),
//     ).map(([, option]) => option)
//   },
//   { immediate: true },
// )

watch(
  [() => filters.value, () => pagination.value.perPage],
  () => {
    pagination.value.page = 1
  },
  { deep: true },
)

onMounted(async () => {
  await adminProjectsStore.refreshProjects()
})

const doShowEditItemModal = ref(false)
const doShowViewItemModal = ref(false)
const itemToEdit = ref<AdminProject | null>(null)
const errorMsg = ref('')

const showEditItemModal = (item: AdminProject) => {
  itemToEdit.value = item
  errorMsg.value = ''
  doShowEditItemModal.value = true
}

const showViewItemModal = (item: AdminProject) => {
  itemToEdit.value = item
  doShowViewItemModal.value = true
}

const { confirm } = useModal()

const onItemSaved = async (item: AdminProject, ok: () => void) => {
  errorMsg.value = ''
  const agreed = await confirm({
    title: t('dna.update_score'),
    message: t('dna.confirm_update_score', { name: item.name }),
    okText: t('omics.save'),
    cancelText: t('omics.cancel'),
    size: 'small',
    maxWidth: '380px',
  })
  if (agreed) {
    try {
      await adminProjectsStore.updateProject(item)
      ok()
    } catch (e) {
      errorMsg.value = e as string
    }
  }
}
</script>
