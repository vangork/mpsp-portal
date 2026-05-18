<template>
  <h1 class="page-title font-bold">{{ t('menu.protein') }}</h1>
  <VaCard>
    <VaCardContent>
      <div class="flex flex-col xs:flex-row gap-2 mb-2 justify-between">
        <VaButton :disabled="loadingOssToken || !omicsStore.webrReady" @click="showAddItemModal">{{
          t('omics.add_item')
        }}</VaButton>
      </div>

      <PlasmixProteinTable
        :items="items"
        :loading="omicsStore.plasmixProteinLoading"
        :prevent-download="omicsStore.plasmixProteinPreventDownload"
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
    <h1 class="va-h5">{{ itemToEdit ? t('plasmix_protein.edit_data') : t('plasmix_protein.add_data') }}</h1>
    <EditPlasmixProteinForm
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
import PlasmixProteinTable from './widgets/PlasmixProteinTable.vue'
import EditPlasmixProteinForm from './widgets/EditPlasmixProteinForm.vue'
import { Pagination, getAliOssToken } from '../../data/pages/omics'
import { AliOSSToken, PlasmixProteinItem } from './types'
import { download } from './composables/useOmics'

const { t } = useI18n()

const omicsStore = useOmicsStore()

const pagination = ref<Pagination>({
  page: 1,
  perPage: 10,
  total: 1,
} as Pagination)

onMounted(async () => {
  await omicsStore.refreshPlasmixProtein()
})

watch(
  () => omicsStore.plasmixProteinItems.length,
  (val) => {
    pagination.value.total = val
  },
  { immediate: true },
)

const items = computed(() => {
  return omicsStore.plasmixProteinItems.slice(
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

const itemToEdit = ref<PlasmixProteinItem | null>(null)
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

const showEditItemModal = (item: PlasmixProteinItem) => {
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

const onItemSaved = async (item: PlasmixProteinItem) => {
  if (itemToEdit.value) {
    await omicsStore.updatePlasmixProtein(item)
  } else {
    await omicsStore.addPlasmixProtein(item)
  }
}

const onItemDownload = async (item: PlasmixProteinItem) => {
  const url = await omicsStore.getPlasmixProteinReportUrl(item)
  download(url, `${item.name}_Plasmix_Protein_QC_Report.pdf`)
}
</script>
