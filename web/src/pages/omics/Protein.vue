<template>
  <h1 class="page-title font-bold">{{ t('menu.protein') }}</h1>
  <VaCard>
    <VaCardContent>
      <div class="flex flex-col xs:flex-row gap-2 mb-2 justify-between">
        <VaButton :disabled="loadingOssToken || !omicsStore.webrReady" @click="showAddItemModal">{{
          t('omics.add_item')
        }}</VaButton>
      </div>

      <ProteinTable
        :items="items"
        :loading="omicsStore.proteinLoading"
        :prevent-download="omicsStore.proteinPreventDownload"
        :pagination="pagination"
        @editItem="showEditItemModal"
        @downloadItem="onItemDownload"
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
    <h1 class="va-h5">{{ itemToEdit ? t('protein.edit_data') : t('protein.add_data') }}</h1>
    <EditProteinForm
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
import ProteinTable from './widgets/ProteinTable.vue'
import EditProteinForm from './widgets/EditProteinForm.vue'
import { Pagination, getAliOssToken } from '../../data/pages/omics'
import { AliOSSToken, ProteinItem } from './types'
import { download } from './composables/useOmics'

const { t } = useI18n()

const omicsStore = useOmicsStore()

const pagination = ref<Pagination>({
  page: 1,
  perPage: 10,
  total: 1,
} as Pagination)

onMounted(async () => {
  await omicsStore.refreshProtein()
})

watch(
  () => omicsStore.proteinItems.length,
  (val) => {
    pagination.value.total = val
  },
  { immediate: true },
)

const items = computed(() => {
  return omicsStore.proteinItems.slice(
    (pagination.value.page - 1) * pagination.value.perPage,
    pagination.value.page * pagination.value.perPage,
  )
})

watch(
  () => pagination.value.perPage,
  () => {
    pagination.value.page = 1
  },
)

const doShowEditItemModal = ref(false)

const itemToEdit = ref<ProteinItem | null>(null)
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

const showEditItemModal = (item: ProteinItem) => {
  itemToEdit.value = item
  ossToken.value = null
  doShowEditItemModal.value = true
}

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

const onItemSaved = async (item: ProteinItem) => {
  if (itemToEdit.value) {
    await omicsStore.updateProtein(item)
  } else {
    await omicsStore.addProtein(item)
  }
}

const onItemDownload = async (item: ProteinItem) => {
  const url = await omicsStore.getProteinReportUrl(item)
  download(url, `${item.name}_Quartet_Protein_QC_Report.pdf`)
}
</script>
