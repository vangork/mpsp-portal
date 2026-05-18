<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useModal } from 'vuestic-ui'
import { useUsers } from './composables/useUsers'
import { User } from './types'
import UsersTable from './widgets/UsersTable.vue'
import EditUserForm from './widgets/EditUserForm.vue'

const { t } = useI18n()

const doShowEditUserModal = ref(false)

const { users, isLoading, filters, pagination, error, ...usersApi } = useUsers()

const userToEdit = ref<User | null>(null)

const showEditUserModal = (user: User) => {
  // user password undefined and convert it to empty string
  user.password = ''
  error.value = null
  userToEdit.value = user
  doShowEditUserModal.value = true
}

const showAddUserModal = () => {
  error.value = null
  userToEdit.value = null
  doShowEditUserModal.value = true
}

const onUserSaved = async (user: User, ok: () => void) => {
  if (userToEdit.value) {
    await usersApi.update(user)
  } else {
    await usersApi.add(user)
  }
  if (!error.value) {
    ok()
  }
}

const editFormRef = ref()

const { confirm } = useModal()

const beforeEditFormModalClose = async (hide: () => unknown) => {
  if (editFormRef.value.isFormHasUnsavedChanges) {
    const agreed = await confirm({
      maxWidth: '380px',
      message: t('user.confirm_form_change_disregard'),
      size: 'small',
    })
    if (agreed) {
      hide()
    }
  } else {
    hide()
  }
}
</script>

<template>
  <h1 class="page-title">{{ t('menu.users') }}</h1>

  <VaCard>
    <VaCardContent>
      <div class="flex flex-col md:flex-row gap-2 mb-2 justify-between">
        <div class="flex flex-col md:flex-row gap-2 justify-start">
          <VaButtonToggle
            v-model="filters.isActive"
            color="background-element"
            border-color="background-element"
            :options="[
              { label: 'Active', value: true },
              { label: 'Inactive', value: false },
            ]"
          />
          <VaInput v-model="filters.search" :placeholder="t('vuestic.search')" clearable>
            <template #prependInner>
              <VaIcon name="search" color="secondary" size="small" />
            </template>
          </VaInput>
        </div>
        <VaButton @click="showAddUserModal">{{ t('user.add_user') }}</VaButton>
      </div>

      <UsersTable :users="users" :loading="isLoading" :pagination="pagination" @editUser="showEditUserModal" />
    </VaCardContent>
  </VaCard>

  <VaModal
    v-slot="{ cancel, ok }"
    v-model="doShowEditUserModal"
    size="small"
    mobile-fullscreen
    close-button
    hide-default-actions
    :before-cancel="beforeEditFormModalClose"
  >
    <h1 class="va-h5">{{ userToEdit ? t('user.edit_user') : t('user.add_user') }}</h1>
    <EditUserForm
      ref="editFormRef"
      :user="userToEdit"
      :save-button-label="userToEdit ? t('user.save') : t('user.add')"
      @close="cancel"
      @save="
        (user) => {
          onUserSaved(user, ok)
        }
      "
    />
    <div v-if="!!error" class="flex gap-4 w-full">
      <VaAlert dense closeable color="danger" class="w-full">
        {{ error }}
      </VaAlert>
    </div>
  </VaModal>
</template>
