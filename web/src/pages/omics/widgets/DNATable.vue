<script setup lang="ts">
import { PropType, computed, ref, toRef, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { defineVaDataTableColumns } from 'vuestic-ui'
import { DNAItem, DNAApplication, DNAFamilyMember } from '../types'
import { Pagination } from '../../../data/pages/omics'
import { formatDate, recoverFileName } from '../composables/useOmics'

const { t } = useI18n()

const columns = defineVaDataTableColumns([
  { label: 'ID', key: 'id', sortable: true },
  { label: t('dna.name'), key: 'name', sortable: true },
  { label: t('dna.sample_id'), key: 'family_member', sortable: true },
  { label: t('dna.application'), key: 'application', sortable: true },
  { label: t('dna.vcf_file'), key: 'vcf_file' },
  { label: t('dna.determinated_at'), key: 'determinated_at', sortable: true },
  { label: t('dna.status'), key: 'status', sortable: false },
  { label: ' ', key: 'actions', align: 'right' },
])

const familyMemberSelectOptions: { text: string; value: DNAFamilyMember }[] = [
  { text: t('dna.d5'), value: 5 },
  { text: t('dna.d6'), value: 6 },
  { text: t('dna.f7'), value: 7 },
  { text: t('dna.m8'), value: 8 },
]

const applicationSelectOptions: { text: string; value: DNAApplication }[] = [
  { text: t('dna.wgs'), value: 'WGS' },
  { text: t('dna.wes'), value: 'WES' },
  { text: t('dna.targeted_panel'), value: 'Targeted Panel' },
  { text: t('dna.low_pass_wgs'), value: 'Low-pass WGS' },
]

const props = defineProps({
  items: {
    type: Array as PropType<DNAItem[]>,
    required: true,
  },
  selectable: { type: Boolean, default: false },
  loading: { type: Boolean, default: false },
  pagination: { type: Object as PropType<Pagination>, required: true },
})

const emit = defineEmits<{
  (event: 'edit-item', item: DNAItem): void
}>()

const items = toRef(props, 'items')
const selectedItems = ref<DNAItem[]>([])
const unselectItem = (item: DNAItem) => {
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

    <template #cell(name)="{ rowData }">
      <div class="max-w-[120px] ellipsis">
        {{ rowData.name }}
      </div>
    </template>

    <template #cell(family_member)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ familyMemberSelectOptions.find((option) => option.value === rowData.family_member)?.text }}
      </div>
    </template>

    <template #cell(application)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ applicationSelectOptions.find((option) => option.value === rowData.application)?.text }}
      </div>
    </template>

    <template #cell(vcf_file)="{ rowData }">
      <div>
        {{ recoverFileName(rowData.vcf_file) }}
      </div>
    </template>

    <template #cell(determinated_at)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ formatDate(rowData.determinated_at) }}
      </div>
    </template>

    <template #cell(status)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        <VaBadge v-if="rowData.indel_number || rowData.snv_number" color="success" :text="t('dna.done')"></VaBadge>
        <VaBadge v-else-if="!!rowData.notes" color="warning" :text="rowData.notes"></VaBadge>
        <VaBadge v-else color="secondary" :text="t('dna.processing')"></VaBadge>
      </div>
    </template>

    <template #cell(actions)="{ rowData }">
      <div class="flex gap-2 justify-end">
        <VaButton
          preset="primary"
          size="small"
          icon="mso-edit"
          aria-label="Edit item"
          @click="emit('edit-item', rowData as DNAItem)"
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
