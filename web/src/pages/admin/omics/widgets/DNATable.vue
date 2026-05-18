<script setup lang="ts">
import { PropType, computed, ref, toRef, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { defineVaDataTableColumns } from 'vuestic-ui'
import { formatDate, roundTo } from '../../../omics/composables/useOmics'
import { Pagination } from '../../../../data/pages/omics'
import { AdminDNAItem } from '../types'

const { t } = useI18n()

const columns = defineVaDataTableColumns([
  { label: 'ID', key: 'id', sortable: true },
  { label: t('auth.username'), key: 'username', sortable: true },
  { label: t('dna.name'), key: 'name', sortable: true },
  { label: t('dna.vcf_file'), key: 'vcf_file' },
  { label: t('dna.determinated_at'), key: 'determinated_at', sortable: true },
  { label: t('omics.qc_result'), key: 'qc_result', sortable: true },
  { label: 'SNV No.', key: 'snv_num', sortable: false },
  { label: 'INDEL No.', key: 'indel_num', sortable: false },
  { label: 'SNV Precision', key: 'snv_precision', sortable: false },
  { label: 'Indel Precision', key: 'indel_precision', sortable: false },
  { label: 'SNV Recall', key: 'snv_recall', sortable: false },
  { label: 'Indel Recall', key: 'indel_recall', sortable: false },
  { label: t('omics.notes'), key: 'notes', sortable: false },
  { label: ' ', key: 'actions', align: 'right' },
])

const props = defineProps({
  items: {
    type: Array as PropType<AdminDNAItem[]>,
    required: true,
  },
  selectable: { type: Boolean, default: false },
  loading: { type: Boolean, default: false },
  pagination: { type: Object as PropType<Pagination>, required: true },
})

const emit = defineEmits<{
  (event: 'download-item', item: AdminDNAItem): void
  (event: 'edit-item', item: AdminDNAItem): void
  (event: 'view-item', item: AdminDNAItem): void
}>()

const items = toRef(props, 'items')
const selectedItems = ref<AdminDNAItem[]>([])
const unselectItem = (item: AdminDNAItem) => {
  selectedItems.value = selectedItems.value.filter((selectedItem) => selectedItem !== item)
}

watch(
  () => props.selectable,
  (val) => {
    if (!val) {
      selectedItems.value = []
    }
  },
)

const selectedDNAItems = computed(() => {
  return selectedItems.value
})

defineExpose({
  selectedDNAItems,
})

const totalPages = computed(() => Math.ceil(props.pagination.total / props.pagination.perPage))
</script>

<template>
  <VaDataTable
    v-model="selectedItems"
    sort-by="id"
    sorting-order="desc"
    :columns="columns"
    :items="items"
    :loading="$props.loading"
    :selectable="selectable"
    select-mode="multiple"
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

    <template #cell(vcf_file)="{ rowData }">
      <div>
        {{ rowData.vcf_file }}
        <VaButton
          preset="primary"
          size="small"
          icon="mso-download"
          aria-label="Download VCF file"
          @click="emit('download-item', rowData as AdminDNAItem)"
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
        <VaBadge
          v-if="
            rowData.indel_number &&
            rowData.snv_number &&
            rowData.indel_precision! >= 90 &&
            rowData.snv_precision! >= 99 &&
            rowData.indel_recall! >= 90 &&
            rowData.snv_recall! >= 98
          "
          text="pass"
          color="success"
        />
        <VaBadge v-else-if="rowData.indel_number || rowData.snv_number" text="fail" color="danger" />
        <VaBadge v-else-if="!!rowData.notes" text="audit" color="warning" />
        <VaBadge v-else text="processing" color="secondary" />
      </div>
    </template>

    <template #cell(indel_num)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        {{ rowData.indel_number }}
      </div>
    </template>

    <template #cell(snv_num)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        {{ rowData.snv_number }}
      </div>
    </template>

    <template #cell(indel_precision)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        {{ roundTo(rowData.indel_precision, 3) }}
      </div>
    </template>

    <template #cell(snv_precision)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        {{ roundTo(rowData.snv_precision, 3) }}
      </div>
    </template>

    <template #cell(indel_recall)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        {{ roundTo(rowData.indel_recall, 3) }}
      </div>
    </template>

    <template #cell(snv_recall)="{ rowData }">
      <div class="ellipsis max-w-[100px]">
        {{ roundTo(rowData.snv_recall, 3) }}
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
          @click="emit('view-item', rowData as AdminDNAItem)"
        />
        <VaButton
          preset="primary"
          size="small"
          icon="mso-edit"
          aria-label="Edit item"
          @click="emit('edit-item', rowData as AdminDNAItem)"
        />
      </div>
    </template>
  </VaDataTable>

  <VaAlert v-if="selectable" class="!mt-6" color="info" outline>
    {{ t('dna.select_hint') }}:
    <VaChip v-for="item in selectedItems" :key="item.id" class="ml-2" @click="unselectItem(item)">
      {{ item.name }}
    </VaChip>
  </VaAlert>

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
