<script setup lang="ts">
import { PropType, ref, toRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { defineVaDataTableColumns } from 'vuestic-ui'
import { Receiver } from '../types'

const { t } = useI18n()

const columns = defineVaDataTableColumns([
  { label: '#', key: 'id', sortable: true },
  { label: t('contact.name'), key: 'name', sortable: false },
  { label: t('contact.address'), key: 'address', sortable: false },
  { label: t('contact.phone'), key: 'phone', sortable: false },
  { label: t('contact.email'), key: 'email', sortable: false },
  { label: t('contact.default_receiver'), key: 'default', sortable: false },
  { label: ' ', key: 'actions', align: 'right' },
])

const props = defineProps({
  receivers: {
    type: Array as PropType<Receiver[]>,
    required: true,
  },
  loading: { type: Boolean, default: false },
})

const emit = defineEmits<{
  (event: 'edit-receiver', receiver: Receiver): void
  (event: 'delete-receiver', receiver: Receiver): void
}>()

const receivers = toRef(props, 'receivers')
const yes = ref(true)
</script>

<template>
  <VaDataTable :columns="columns" :items="receivers" :loading="$props.loading">
    <template #cell(id)="{ rowIndex }">
      <div class="max-w-[120px] ellipsis">
        {{ rowIndex + 1 }}
      </div>
    </template>

    <template #cell(name)="{ rowData }">
      <div class="max-w-[230px] ellipsis">
        {{ rowData.name }}
      </div>
    </template>

    <template #cell(address)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ rowData.institution }}
      </div>
    </template>

    <template #cell(email)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ rowData.email }}
      </div>
    </template>

    <template #cell(phone)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ rowData.phone }}
      </div>
    </template>

    <template #cell(default)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        <VaCheckbox v-if="rowData.default" v-model="yes" color="info" readonly />
      </div>
    </template>

    <template #cell(actions)="{ rowData }">
      <div class="flex gap-2 justify-end">
        <VaButton
          preset="primary"
          size="small"
          icon="mso-edit"
          aria-label="Edit receiver"
          @click="emit('edit-receiver', rowData as Receiver)"
        />
        <VaButton
          preset="danger"
          size="small"
          icon="mso-delete"
          aria-label="Delete receiver"
          @click="emit('delete-receiver', rowData as Receiver)"
        />
      </div>
    </template>
  </VaDataTable>
</template>

<style lang="scss" scoped>
.va-data-table {
  ::v-deep(.va-data-table__table-tr) {
    border-bottom: 1px solid var(--va-background-border);
  }
}
</style>
