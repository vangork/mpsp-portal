<script setup lang="ts">
import { PropType, computed, ref, toRef, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { defineVaDataTableColumns } from 'vuestic-ui'
import { formatDate, roundTo } from '../../../../services/utils'
import { Pagination } from '../../../../data/pages/projects'
import { AdminProject } from '../types'
import { STAGES } from '../../../user/projects/types'

const { t } = useI18n()

const columns = defineVaDataTableColumns([
  { label: '#', key: 'code', sortable: false },
  { label: 'ID', key: 'id', sortable: true },
  { label: '送样单位', key: 'institution', sortable: true },
  { label: '合作者姓名', key: 'contact', sortable: true },
  { label: '样本数量', key: 'sampleCount', sortable: false },
  { label: '订单时间', key: 'createdAt', sortable: false },
  { label: '状态', key: 'stage', sortable: false },
  { label: ' ', key: 'actions', align: 'right' },
])

const props = defineProps({
  items: {
    type: Array as PropType<AdminProject[]>,
    required: true,
  },
  loading: { type: Boolean, default: false },
  pagination: { type: Object as PropType<Pagination>, required: true },
})

const emit = defineEmits<{
  (event: 'edit-item', item: AdminProject): void
  (event: 'view-item', item: AdminProject): void
}>()

const items = toRef(props, 'items')


const totalPages = computed(() => Math.ceil(props.pagination.total / props.pagination.perPage))
</script>

<template>
  <VaDataTable
    sort-by="code"
    sorting-order="desc"
    :columns="columns"
    :items="items"
    :loading="$props.loading"
    :no-data-html="t('vuestic.no_items')"
  >
    <template #cell(code)="{ rowIndex }">
      <div class="max-w-[120px] ellipsis">
        {{ rowIndex + 1 + ($props.pagination.page - 1) * $props.pagination.perPage }}
      </div>
    </template>

    <template #cell(id)="{ rowData }">
      <div class="max-w-[120px] ellipsis">
        {{ rowData.id }}
      </div>
    </template>

    <template #cell(institution)="{ rowData }">
      <div class="max-w-[200px] ellipsis">
        {{ rowData.institution }}
      </div>
    </template>

    <template #cell(contact)="{ rowData }">
      <div class="max-w-[120px] ellipsis">
        {{ rowData.contact }}
      </div>
    </template>

    <template #cell(sampleCount)="{ rowData }">
      <div>
        {{ rowData.sampleCount }}
      </div>
    </template>

    <template #cell(createdAt)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        <!-- {{ formatDate(rowData.createdAt) }} -->
        {{ rowData.createdAt }}
      </div>
    </template>

    <template #cell(stage)="{ rowData }">
      <VaBadge :text="STAGES[rowData.stage]?.label || '未知'" :color="STAGES[rowData.stage]?.color || 'dark'" />
    </template>

    <template #cell(actions)="{ rowData }">
      <div class="flex gap-2 justify-end">
        <VaButton
          preset="primary"
          size="small"
          icon="mso-visibility"
          aria-label="View item"
          @click="emit('view-item', rowData as AdminProject)"
        />
        <VaButton
          preset="primary"
          size="small"
          icon="mso-edit"
          aria-label="Edit item"
          @click="emit('edit-item', rowData as AdminProject)"
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
