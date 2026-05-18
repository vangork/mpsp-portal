<template>
  <h1 class="page-title font-bold">{{ t('menu.metabolism') }}</h1>
  <VaCard>
    <VaCardContent>
      <div class="flex flex-col xs:flex-row gap-2 mb-2 justify-between">
        <VaButton :disabled="loadingOssToken || !omicsStore.webrReady" @click="showAddItemModal">{{
          t('omics.add_item')
        }}</VaButton>
      </div>

      <MetabolismTable
        :items="items"
        :loading="omicsStore.metabolismLoading"
        :prevent-download="omicsStore.metabolismPreventDownload"
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
    <h1 class="va-h5">{{ itemToEdit ? t('metabolism.edit_data') : t('metabolism.add_data') }}</h1>
    <EditMetabolismForm
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
import MetabolismTable from './widgets/MetabolismTable.vue'
import EditMetabolismForm from './widgets/EditMetabolismForm.vue'
import { Pagination, getAliOssToken } from '../../data/pages/omics'
import { AliOSSToken, MetabolismItem } from './types'
import { download } from './composables/useOmics'

const { t } = useI18n()

const omicsStore = useOmicsStore()

const pagination = ref<Pagination>({
  page: 1,
  perPage: 10,
  total: 1,
} as Pagination)

onMounted(async () => {
  await omicsStore.refreshMetabolism()
})

watch(
  () => omicsStore.metabolismItems.length,
  (val) => {
    pagination.value.total = val
  },
  { immediate: true },
)

const items = computed(() => {
  return omicsStore.metabolismItems.slice(
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

const itemToEdit = ref<MetabolismItem | null>(null)
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

const showEditItemModal = (item: MetabolismItem) => {
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

const onItemSaved = async (item: MetabolismItem) => {
  if (itemToEdit.value) {
    await omicsStore.updateMetabolism(item)
  } else {
    await omicsStore.addMetabolism(item)
  }
}

const onItemDownload = async (item: MetabolismItem) => {
  const url = await omicsStore.getMetabolismReportUrl(item)
  download(url, `${item.name}_Quartet_Metabolism_QC_Report.pdf`)
}
</script>
