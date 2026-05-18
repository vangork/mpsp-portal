<script setup lang="ts">
import { PropType, computed, toRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { defineVaDataTableColumns, useModal } from 'vuestic-ui'
import { ProteinItem } from '../types'
import { Pagination } from '../../../data/pages/omics'
import { formatDate, recoverFileName } from '../composables/useOmics'

const { t } = useI18n()

const columns = defineVaDataTableColumns([
  { label: 'ID', key: 'id', sortable: true },
  { label: t('protein.name'), key: 'name', sortable: true },
  { label: t('protein.ms_instrument'), key: 'ms_instrument', sortable: true },
  { label: t('protein.exp_file'), key: 'exp_file' },
  { label: t('protein.determinated_at'), key: 'determinated_at', sortable: true },
  { label: ' ', key: 'actions', align: 'right' },
])

const props = defineProps({
  items: {
    type: Array as PropType<ProteinItem[]>,
    required: true,
  },
  preventDownload: { type: Boolean, default: false },
  loading: { type: Boolean, default: false },
  pagination: { type: Object as PropType<Pagination>, required: true },
})

const emit = defineEmits<{
  (event: 'edit-item', item: ProteinItem): void
  (event: 'download-item', item: ProteinItem): void
}>()

const items = toRef(props, 'items')

const totalPages = computed(() => Math.ceil(props.pagination.total / props.pagination.perPage))

const { confirm } = useModal()

const onItemDownload = async (item: ProteinItem) => {
  const agreed = await confirm({
    title: t('omics.download_report'),
    message: t('omics.confirm_download_with_name', { name: item.name }),
    okText: t('omics.download'),
    cancelText: t('omics.cancel'),
    size: 'small',
    maxWidth: '380px',
  })
  if (agreed) {
    emit('download-item', item)
  }
}
</script>

<template>
  <VaDataTable
    sort-by="id"
    sorting-order="desc"
    :columns="columns"
    :items="items"
    :loading="$props.loading"
    :no-data-html="t('vuestic.no_items')"
  >
    <template #cell(id)="{ rowIndex }">
      <div class="max-w-[120px] ellipsis">
        {{ rowIndex + 1 + ($props.pagination.page - 1) * $props.pagination.perPage }}
      </div>
    </template>

    <template #cell(name)="{ rowData }">
      <div class="max-w-[120px] ellipsis">
        {{ rowData.name }}
      </div>
    </template>

    <template #cell(ms_instrument)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ rowData.ms_instrument }}
      </div>
    </template>

    <template #cell(exp_file)="{ rowData }">
      <div>
        {{ recoverFileName(rowData.exp_file) }}
      </div>
    </template>

    <template #cell(determinated_at)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ formatDate(rowData.determinated_at) }}
      </div>
    </template>

    <template #cell(actions)="{ rowData }">
      <div class="flex gap-2 justify-end">
        <VaButton
          preset="primary"
          size="small"
          icon="mso-edit"
          aria-label="Edit item"
          @click="$emit('edit-item', rowData as ProteinItem)"
        />
        <VaButton
          preset="primary"
          size="small"
          icon="mso-download"
          color="info"
          aria-label="Download QC Report"
          :disabled="preventDownload"
          @click="onItemDownload(rowData as ProteinItem)"
        />
      </div>
    </template>
  </VaDataTable>

  <div class="flex flex-col-reverse md:flex-row gap-2 justify-between items-center py-2">
    <div>
      <b> {{ t('vuestic.total_results', { value: $props.pagination.total }) }}.</b>
      {{ t('vuestic.results_per_page') }}
      <VaSelect v-model="$props.pagination.perPage" class="!w-20" :options="[10, 30, 50]" />
    </div>

    <div v-if="totalPages > 1" class="flex">
      <VaButton
        preset="secondary"
        icon="va-arrow-left"
        aria-label="Previous page"
        :disabled="$props.pagination.page === 1"
        @click="$props.pagination.page--"
      />
      <VaButton
        class="mr-2"
        preset="secondary"
        icon="va-arrow-right"
        aria-label="Next page"
        :disabled="$props.pagination.page === totalPages"
        @click="$props.pagination.page++"
      />
      <VaPagination
        v-model="$props.pagination.page"
        buttons-preset="secondary"
        :pages="totalPages"
        :visible-pages="5"
        :boundary-links="false"
        :direction-links="false"
      />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.va-data-table {
  ::v-deep(.va-data-table__table-tr) {
    border-bottom: 1px solid var(--va-background-border);
  }
}
</style>
