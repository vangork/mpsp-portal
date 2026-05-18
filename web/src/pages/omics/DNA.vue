<template>
  <h1 class="page-title font-bold">{{ t('menu.dna') }}</h1>
  <VaCard>
    <VaCardContent>
      <VaAlert v-model="isCloseableAlertVisible" color="info" closeable close-icon="info" class="mb-6">
        {{ t('dna.upload_hint') }}
      </VaAlert>
      <div class="flex flex-col xs:flex-row gap-2 mb-2 justify-between">
        <VaButton :disabled="loadingOssToken || selectable" @click="showAddItemModal">{{
          t('omics.add_item')
        }}</VaButton>

        <div>
          <VaButton
            v-if="dnaTableRef?.selectedDNAItems.length > 0"
            class="mr-2"
            preset="primary"
            @click="onItemDownload(dnaTableRef.selectedDNAItems)"
            >{{ t('omics.download') }}</VaButton
          >
          <VaButton
            v-if="omicsStore.dnaItems.length > 0 && !selectable"
            :disabled="omicsStore.dnaPreventDownload"
            preset="primary"
            @click="selectable = true"
            >{{ t('omics.get_report') }}</VaButton
          >
          <VaButton
            v-else-if="omicsStore.dnaItems.length > 0 && selectable"
            preset="primary"
            :disabled="omicsStore.dnaPreventDownload"
            @click="selectable = false"
            >{{ t('omics.cancel') }}</VaButton
          >
        </div>
      </div>

      <DNATable
        ref="dnaTableRef"
        :items="items"
        :loading="omicsStore.dnaLoading"
        :selectable="selectable"
        :pagination="pagination"
        @editItem="showEditItemModal"
      />
    </VaCardContent>
  </VaCard>

  <VaModal
    v-slot="{ cancel, ok }"
    v-model="doShowEditItemModal"
    size="small"
    mobile-fullscreen
    close-button
    hide-default-actions
    :before-cancel="beforeEditFormModalClose"
  >
    <h1 class="va-h5">{{ itemToEdit ? t('dna.edit_data') : t('dna.add_data') }}</h1>
    <EditDNAForm
      ref="editFormRef"
      :item="itemToEdit"
      :token="ossToken"
      :save-button-label="itemToEdit ? t('omics.save') : t('omics.add')"
      @close="cancel"
      @save="
        (item) => {
          onItemSaved(item)
          ok()
        }
      "
    />
  </VaModal>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useModal } from 'vuestic-ui'
import { useOmicsStore } from '../../stores/omics'
import DNATable from './widgets/DNATable.vue'
import EditDNAForm from './widgets/EditDNAForm.vue'
import { Pagination, getAliOssToken } from '../../data/pages/omics'
import { AliOSSToken, DNAItem } from './types'
import { download } from './composables/useOmics'

const { t } = useI18n()

const omicsStore = useOmicsStore()

const pagination = ref<Pagination>({
  page: 1,
  perPage: 10,
  total: 1,
} as Pagination)

onMounted(async () => {
  await omicsStore.refreshDNA()
})

const isCloseableAlertVisible = ref(true)

const selectable = ref(false)

watch(
  [() => omicsStore.dnaItems.length, () => selectable.value],
  ([val, reportOnly]) => {
    pagination.value.total = reportOnly
      ? omicsStore.dnaItems.filter((item) => item.indel_number || item.snv_number).length
      : val
  },
  { immediate: true },
)

const items = computed(() => {
  const array = selectable.value
    ? omicsStore.dnaItems.filter((i) => i.indel_number || i.snv_number)
    : omicsStore.dnaItems

  return array.slice(
    (pagination.value.page - 1) * pagination.value.perPage,
    pagination.value.page * pagination.value.perPage,
  )
})

watch([() => pagination.value.perPage, () => selectable.value], () => {
  pagination.value.page = 1
})
const doShowEditItemModal = ref(false)

const itemToEdit = ref<DNAItem | null>(null)
const ossToken = ref<AliOSSToken | null>(null)
const loadingOssToken = ref(false)

const showAddItemModal = async () => {
  itemToEdit.value = null
  loadingOssToken.value = true
  try {
    ossToken.value = await getAliOssToken()
    doShowEditItemModal.value = true
  } finally {
    loadingOssToken.value = false
  }
}

const showEditItemModal = (item: DNAItem) => {
  itemToEdit.value = item
  ossToken.value = null
  doShowEditItemModal.value = true
}

const dnaTableRef = ref()

const editFormRef = ref()

const { confirm } = useModal()

const beforeEditFormModalClose = async (hide: () => unknown) => {
  if (editFormRef.value.isFormHasUnsavedChanges) {
    const agreed = await confirm({
      maxWidth: '380px',
      message: t('omics.confirm_form_change_disregard'),
      size: 'small',
    })
    if (agreed) {
      hide()
    }
  } else {
    hide()
  }
}

const onItemSaved = async (item: DNAItem) => {
  if (itemToEdit.value) {
    await omicsStore.updateDNA(item)
  } else {
    await omicsStore.addDNA(item)
  }
}

const onItemDownload = async (item: DNAItem[]) => {
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
    const url = await omicsStore.getDNAReportUrl(item)
    download(url, 'Quartet_DNA_QC_Report.pdf')
  }
}
</script>
