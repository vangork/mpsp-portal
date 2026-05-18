<script setup lang="ts">
import { PropType, computed, toRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { defineVaDataTableColumns } from 'vuestic-ui'
import { formatDate, roundTo } from '../../../omics/composables/useOmics'
import { Pagination } from '../../../../data/pages/omics'
import { AdminMetabolismItem, RawDataType } from '../types'

const { t } = useI18n()

const columns = defineVaDataTableColumns([
  { label: 'ID', key: 'id', sortable: true },
  { label: t('auth.username'), key: 'username', sortable: true },
  { label: t('metabolism.name'), key: 'name', sortable: true },
  { label: t('metabolism.meta_file'), key: 'meta_file' },
  { label: t('metabolism.exp_file'), key: 'exp_file' },
  { label: t('metabolism.determinated_at'), key: 'determinated_at', sortable: true },
  { label: t('omics.qc_result'), key: 'qc_result', sortable: true },
  { label: 'snr', key: 'snr', sortable: false },
  { label: 'pcc', key: 'pcc', sortable: false },
  { label: t('omics.sample_number'), key: 'sample_number', sortable: false },
  { label: t('omics.notes'), key: 'notes', sortable: false },
  { label: ' ', key: 'actions', align: 'right' },
])

const props = defineProps({
  items: {
    type: Array as PropType<AdminMetabolismItem[]>,
    required: true,
  },
  preventDownload: { type: Boolean, default: false },
  loading: { type: Boolean, default: false },
  pagination: { type: Object as PropType<Pagination>, required: true },
})

const emit = defineEmits<{
  (event: 'download-item', item: AdminMetabolismItem, type: RawDataType): void
  (event: 'edit-item', item: AdminMetabolismItem): void
  (event: 'view-item', item: AdminMetabolismItem): void
  (event: 'download-report', item: AdminMetabolismItem): void
}>()

const items = toRef(props, 'items')

const totalPages = computed(() => Math.ceil(props.pagination.total / props.pagination.perPage))
</script>

<template>
  <VaDataTable
    sort-by="id"
    sorting-order="desc"
    :columns="columns"
    :items="items"
    :loading="props.loading"
    :no-data-html="t('vuestic.no_items')"
  >
    <template #cell(id)="{ rowIndex }">
      <div class="max-w-[120px] ellipsis">
        {{ rowIndex + 1 + ($props.pagination.page - 1) * $props.pagination.perPage }}
      </div>
    </template>

    <template #cell(username)="{ rowData }">
      <div class="max-w-[120px] ellipsis">
        {{ rowData.username }}
      </div>
    </template>

    <template #cell(name)="{ rowData }">
      <div class="max-w-[200px] ellipsis">
        {{ rowData.name }}
      </div>
    </template>

    <template #cell(meta_file)="{ rowData }">
      <div>
        {{ rowData.meta_file }}
        <VaButton
          preset="primary"
          size="small"
          icon="mso-download"
          aria-label="Download Meta file"
          @click="emit('download-item', rowData as AdminMetabolismItem, 'meta')"
        />
      </div>
    </template>

    <template #cell(exp_file)="{ rowData }">
      <div>
        {{ rowData.exp_file }}
        <VaButton
          preset="primary"
          size="small"
          icon="mso-download"
          aria-label="Download Exp file"
          @click="emit('download-item', rowData as AdminMetabolismItem, 'exp')"
        />
      </div>
    </template>

    <template #cell(determinated_at)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ formatDate(rowData.determinated_at) }}
      </div>
    </template>

    <template #cell(qc_result)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        <VaBadge v-if="rowData.snr && rowData.pcc && rowData.snr >= 10" text="pass" color="success" />
        <VaBadge v-else text="fail" color="danger" />
      </div>
    </template>

    <template #cell(snr)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        {{ roundTo(rowData.snr, 3) }}
      </div>
    </template>

    <template #cell(pcc)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        {{ roundTo(rowData.pcc, 3) }}
      </div>
    </template>

    <template #cell(sample_number)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        {{ rowData.d5 + rowData.d6 + rowData.f7 + rowData.m8 }}
      </div>
    </template>

    <template #cell(notes)="{ rowData }">
      <div class="ellipsis max-w-[300px]">
        {{ rowData.notes }}
      </div>
    </template>

    <template #cell(actions)="{ rowData }">
      <div class="flex gap-2 justify-end">
        <VaButton
          preset="primary"
          size="small"
          icon="mso-visibility"
          aria-label="View item"
          @click="emit('view-item', rowData as AdminMetabolismItem)"
        />
        <VaButton
          preset="primary"
          size="small"
          icon="mso-download"
          color="info"
          aria-label="Download QC Report"
          :disabled="preventDownload"
          @click="emit('download-report', rowData as AdminMetabolismItem)"
        />
        <VaButton
          preset="primary"
          size="small"
          icon="mso-edit"
          aria-label="Edit item"
          @click="emit('edit-item', rowData as AdminMetabolismItem)"
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
::v-deep(.custom-class) {
  pointer-events: none;
  background-color: var(--va-background-element);
}
</style>
