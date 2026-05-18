<script setup lang="ts">
import { PropType, computed, toRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { defineVaDataTableColumns } from 'vuestic-ui'
import { Pagination } from '../../../../data/pages/users'
import { role, User, UserRoleKeys } from '../types'

const { t } = useI18n()

const columns = defineVaDataTableColumns([
  { label: 'ID', key: 'id', sortable: true },
  { label: t('auth.username'), key: 'username', sortable: true },
  { label: t('auth.email'), key: 'email', sortable: true },
  { label: t('user.role'), key: 'role', sortable: true },
  { label: t('user.notes'), key: 'notes', sortable: false },
  { label: ' ', key: 'actions', align: 'right' },
])

const props = defineProps({
  users: {
    type: Array as PropType<User[]>,
    required: true,
  },
  loading: { type: Boolean, default: false },
  pagination: { type: Object as PropType<Pagination>, required: true },
})

const emit = defineEmits<{
  (event: 'edit-user', user: User): void
}>()

const users = toRef(props, 'users')

const roleColors: Record<UserRoleKeys, string> = {
  2: 'danger',
  1: 'warning',
  0: 'background-element',
}

const totalPages = computed(() => Math.ceil(props.pagination.total / props.pagination.perPage))
</script>

<template>
  <VaDataTable :columns="columns" :items="users" :loading="$props.loading">
    <template #cell(id)="{ rowData }">
      <div class="items-center gap-2 max-w-[120px]">
        {{ rowData.id }}
      </div>
    </template>

    <template #cell(username)="{ rowData }">
      <div class="max-w-[230px] ellipsis">
        {{ rowData.username }}
      </div>
    </template>

    <template #cell(email)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ rowData.email }}
      </div>
    </template>

    <template #cell(role)="{ rowData }">
      <VaBadge :text="role(rowData.role)" :color="roleColors[rowData.role as UserRoleKeys]" />
    </template>

    <template #cell(notes)="{ rowData }">
      <div class="ellipsis max-w-[230px]">
        {{ rowData.notes }}
      </div>
    </template>

    <template #cell(actions)="{ rowData }">
      <div class="flex gap-2 justify-end">
        <VaButton
          preset="primary"
          size="small"
          icon="mso-edit"
          aria-label="Edit user"
          @click="emit('edit-user', rowData as User)"
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
