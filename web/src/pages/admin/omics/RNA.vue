<template>
  <h1 class="page-title font-bold">{{ t('menu.rna') }}</h1>
  <VaCard>
    <VaCardContent>
      <div class="flex flex-col xs:flex-row gap-2 mb-2 justify-between">
        <div class="flex flex-col md:flex-row gap-2 justify-start">
          <VaSelect
            v-model="filters.users"
            :options="usersSelectOptions"
            :placeholder="t('omics.filter_by_user')"
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

      <RNATable
        :items="items"
        :loading="adminOmicsStore.rnaLoading"
        :prevent-download="adminOmicsStore.rnaPreventDownload"
        :pagination="pagination"
        @downloadItem="onRowDataDownload"
        @editItem="showEditItemModal"
        @viewItem="showViewItemModal"
        @downloadReport="onReportDownload"
      />
    </VaCardContent>
  </VaCard>

  <VaModal
    v-slot="{ ok }"
    v-model="doShowViewItemModal"
    size="small"
    mobile-fullscreen
    hide-default-actions
    close-button
  >
    <h1 class="va-h5">{{ `${t('rna.view_data')}: ${itemToEdit.id}` }}</h1>
    <ViewRNAForm :item="itemToEdit" @close="ok" />
  </VaModal>

  <VaModal
    v-slot="{ cancel, ok }"
    v-model="doShowEditItemModal"
    size="small"
    mobile-fullscreen
    hide-default-actions
    close-button
  >
    <h1 class="va-h5">{{ `${t('rna.edit_data')}: ${itemToEdit.id}` }}</h1>
    <EditRNAForm
      :item="itemToEdit"
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
import { Filters, Pagination, getAdminRNAData } from '../../../data/pages/omics'
import { useAdminOmicsStore } from '../../../stores/admin-omics'
import { download } from '../../omics/composables/useOmics'
import { AdminRNAItem, RawDataType } from './types'
import RNATable from './widgets/RNATable.vue'
import EditRNAForm from './widgets/EditRNAForm.vue'
import ViewRNAForm from './widgets/ViewRNAForm.vue'

const { t } = useI18n()

const adminOmicsStore = useAdminOmicsStore()

const pagination = ref<Pagination>({
  page: 1,
  perPage: 10,
  total: 1,
} as Pagination)

const filters = ref<Partial<Filters>>({ users: [], search: '' })
const usersSelectOptions = ref<{ text: string; value: number }[]>([])

const filteredItems = computed(() => {
  const items = adminOmicsStore.rnaItems.filter((i) => {
    if (
      filters.value.users !== undefined &&
      filters.value.users.length > 0 &&
      !filters.value.users.find((u) => u.value === i.user_id)
    ) {
      return false
    }
    if (filters.value.search) {
      const search = filters.value.search.toLowerCase()
      return i.name.toLowerCase().startsWith(search)
    }
    return true
  })
  return items
})

const items = computed(() => {
  const paginated = filteredItems.value.slice(
    (pagination.value.page - 1) * pagination.value.perPage,
    pagination.value.page * pagination.value.perPage,
  )
  return paginated
})

watch(
  () => filteredItems.value.length,
  (val) => {
    pagination.value.total = val
  },
  { immediate: true },
)

watch(
  [() => adminOmicsStore.rnaItems.length],
  () => {
    const userMap = new Map<number, string>()
    adminOmicsStore.rnaItems.forEach((item) => {
      if (!userMap.has(item.user_id)) {
        userMap.set(item.user_id, item.username)
      }
    })
    usersSelectOptions.value = Array.from(userMap.entries())
      .map(([value, text]) => ({ text, value }))
      .sort((a, b) => a.text.localeCompare(b.text))
    filters.value.users = Array.from(
      usersSelectOptions.value
        .entries()
        .filter(([, option]) => filters.value.users?.map((u) => u.value).includes(option.value)),
    ).map(([, option]) => option)
  },
  { immediate: true },
)

watch(
  [() => filters.value, () => pagination.value.perPage],
  () => {
    pagination.value.page = 1
  },
  { deep: true },
)

onMounted(async () => {
  await adminOmicsStore.refreshRNA()
})

const doShowEditItemModal = ref(false)
const doShowViewItemModal = ref(false)
const itemToEdit = ref<AdminRNAItem>(null as unknown as AdminRNAItem)
const errorMsg = ref('')

const showEditItemModal = (item: AdminRNAItem) => {
  itemToEdit.value = item
  errorMsg.value = ''
  doShowEditItemModal.value = true
}

const showViewItemModal = (item: AdminRNAItem) => {
  itemToEdit.value = item
  doShowViewItemModal.value = true
}

const { confirm } = useModal()

const onRowDataDownload = async (item: AdminRNAItem, type: RawDataType) => {
  const file = type === 'meta' ? item.meta_file : type === 'exp' ? item.exp_file : item.count_file
  const agreed = await confirm({
    title: t('omics.download_raw_data'),
    message: t('omics.confirm_download_raw_data', { file, name: item.name }),
    okText: t('omics.download'),
    cancelText: t('omics.cancel'),
    size: 'auto',
  })
  if (agreed) {
    const data = await getAdminRNAData(item)
    if (type === 'meta') {
      download(data.meta_file, `${item.id}_${item.name}_meta_file.csv`)
    } else if (type === 'exp') {
      download(data.exp_file, `${item.id}_${item.name}_exp_file.csv`)
    } else if (type === 'count') {
      download(data.count_file, `${item.id}_${item.name}_count_file.csv`)
    }
  }
}

const onReportDownload = async (item: AdminRNAItem) => {
  const agreed = await confirm({
    title: t('omics.download_report'),
    message: t('omics.confirm_download_with_name', { name: item.name }),
    okText: t('omics.download'),
    cancelText: t('omics.cancel'),
    size: 'small',
    maxWidth: '380px',
  })
  if (agreed) {
    const url = await adminOmicsStore.getRNAReportUrl(item)
    download(url, 'Quartet_RNA_QC_Report.pdf')
  }
}

const onItemSaved = async (item: AdminRNAItem, ok: () => void) => {
  errorMsg.value = ''
  const agreed = await confirm({
    title: t('omics.update_notes'),
    message: t('omics.confirm_update_notes', { name: item.name }),
    okText: t('omics.save'),
    cancelText: t('omics.cancel'),
    size: 'small',
    maxWidth: '380px',
  })
  if (agreed) {
    try {
      await adminOmicsStore.updateRNANotes(item)
      ok()
    } catch (e) {
      errorMsg.value = e as string
    }
  }
}
</script>
