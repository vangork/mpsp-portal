<template>
  <h1 class="page-title font-bold">{{ t('menu.dna') }}</h1>
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
        <div>
          <VaButton
            v-if="dnaTableRef?.selectedDNAItems.length > 0"
            class="mr-2"
            preset="primary"
            @click="onReportDownload(dnaTableRef.selectedDNAItems)"
            >{{ t('omics.download') }}</VaButton
          >
          <VaButton
            v-if="adminOmicsStore.dnaItems.length > 0 && !selectable"
            :disabled="adminOmicsStore.dnaPreventDownload"
            preset="primary"
            @click="selectable = true"
            >{{ t('omics.get_report') }}</VaButton
          >
          <VaButton
            v-else-if="adminOmicsStore.dnaItems.length > 0 && selectable"
            preset="primary"
            :disabled="adminOmicsStore.dnaPreventDownload"
            @click="selectable = false"
            >{{ t('omics.cancel') }}</VaButton
          >
        </div>
      </div>

      <DNATable
        ref="dnaTableRef"
        :items="items"
        :loading="adminOmicsStore.dnaLoading"
        :selectable="selectable"
        :pagination="pagination"
        @downloadItem="onVcfDownload"
        @editItem="showEditItemModal"
        @viewItem="showViewItemModal"
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
    <h1 class="va-h5">{{ `${t('dna.view_data')}: ${itemToEdit?.id}` }}</h1>
    <ViewDNAForm :item="itemToEdit" @close="ok" />
  </VaModal>

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
import { Filters, Pagination, getAdminDNAVcf } from '../../../data/pages/omics'
import { useAdminOmicsStore } from '../../../stores/admin-omics'
import { download } from '../../omics/composables/useOmics'
import { AdminDNAItem } from './types'
import DNATable from './widgets/DNATable.vue'
import EditDNAForm from './widgets/EditDNAForm.vue'
import ViewDNAForm from './widgets/ViewDNAForm.vue'

const { t } = useI18n()

const adminOmicsStore = useAdminOmicsStore()

const pagination = ref<Pagination>({
  page: 1,
  perPage: 10,
  total: 1,
} as Pagination)

const filters = ref<Partial<Filters>>({ users: [], search: '' })
const usersSelectOptions = ref<{ text: string; value: number }[]>([])
const selectable = ref(false)

const filteredItems = computed(() => {
  const items = adminOmicsStore.dnaItems.filter((i) => {
    if (
      filters.value.users !== undefined &&
      filters.value.users.length > 0 &&
      !filters.value.users.find((u) => u.value === i.user_id)
    ) {
      return false
    }
    if (filters.value.search) {
      const search = filters.value.search.toLowerCase()
      return i.name.toLowerCase().startsWith(search) || i.vcf_file.toLowerCase().includes(search)
    }
    return true
  })
  return items
})

const items = computed(() => {
  const array = selectable.value
    ? filteredItems.value.filter((i) => i.indel_number || i.snv_number)
    : filteredItems.value
  return array.slice(
    (pagination.value.page - 1) * pagination.value.perPage,
    pagination.value.page * pagination.value.perPage,
  )
})

watch(
  [() => filteredItems.value.length, () => selectable.value],
  ([val, reportOnly]) => {
    pagination.value.total = reportOnly
      ? filteredItems.value.filter((item) => item.indel_number || item.snv_number).length
      : val
  },
  { immediate: true },
)

watch(
  [() => adminOmicsStore.dnaItems.length],
  () => {
    const userMap = new Map<number, string>()
    adminOmicsStore.dnaItems.forEach((item) => {
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
  [() => filters.value, () => pagination.value.perPage, () => selectable.value],
  () => {
    pagination.value.page = 1
  },
  { deep: true },
)

onMounted(async () => {
  await adminOmicsStore.refreshDNA()
})

const doShowEditItemModal = ref(false)
const doShowViewItemModal = ref(false)
const itemToEdit = ref<AdminDNAItem | null>(null)
const errorMsg = ref('')

const showEditItemModal = (item: AdminDNAItem) => {
  itemToEdit.value = item
  errorMsg.value = ''
  doShowEditItemModal.value = true
}

const showViewItemModal = (item: AdminDNAItem) => {
  itemToEdit.value = item
  doShowViewItemModal.value = true
}

const onItemSaved = async (item: AdminDNAItem, ok: () => void) => {
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
      await adminOmicsStore.updateDNAScore(item)
      ok()
    } catch (e) {
      errorMsg.value = e as string
    }
  }
}

const dnaTableRef = ref()

const { confirm } = useModal()

const onVcfDownload = async (item: AdminDNAItem) => {
  const agreed = await confirm({
    title: t('dna.download_vcf'),
    message: t('dna.confirm_download', { file: item.vcf_file }),
    okText: t('omics.download'),
    cancelText: t('omics.cancel'),
    size: 'auto',
  })
  if (agreed) {
    const url = await getAdminDNAVcf(item)
    download(url, `${item.id}_${item.name}_vcf_file`)
  }
}

const onReportDownload = async (item: AdminDNAItem[]) => {
  const agreed = await confirm({
    title: t('omics.download_report'),
    message: t('omics.confirm_download'),
    okText: t('omics.download'),
    cancelText: t('omics.cancel'),
    size: 'small',
    maxWidth: '380px',
  })
  if (agreed) {
    selectable.value = false
    const url = await adminOmicsStore.getDNAReportUrl(item)
    download(url, 'Quartet_DNA_QC_Report.pdf')
  }
}
</script>
