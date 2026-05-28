<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useModal } from 'vuestic-ui'
import { useReceiversStore } from '../../../stores/receivers'
import { Receiver } from './types'
import EditReceiverForm from './widgets/EditReceiverForm.vue'
import ReceiversTable from './widgets/ReceiversTable.vue'

const { t } = useI18n()
const receiversStore = useReceiversStore()

const error = ref<string | null>(null)
const doShowEditReceiverModal = ref(false)
const doShowDeleteReceiverModal = ref(false)
const receiverToEdit = ref<Receiver | null>(null)

const showEditReceiverModal = (receiver: Receiver) => {
  error.value = null
  receiverToEdit.value = receiver
  doShowEditReceiverModal.value = true
}

const showAddReceiverModal = () => {
  error.value = null
  receiverToEdit.value = null
  doShowEditReceiverModal.value = true
}

const showDeleteReceiverModal = (receiver: Receiver) => {
  error.value = null
  receiverToEdit.value = receiver
  doShowDeleteReceiverModal.value = true
}

const onReceiverSaved = async (receiver: Receiver, ok: () => void) => {
  error.value = null
  if (receiverToEdit.value) {
    await receiversStore.update(receiver)
  } else {
    await receiversStore.add(receiver)
  }
  if (!error.value) {
    ok()
  }
}

const onReceiverDeleted = async (ok: () => void) => {
  error.value = null
  if (receiverToEdit.value) {
    await receiversStore.delete(receiverToEdit.value.id)
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

onMounted(() => {
  receiversStore.getAll()
})
</script>

<template>
  <h1 class="page-title">{{ t('menu.receivers') }}</h1>

  <VaCard>
    <VaCardContent>
      <div class="flex flex-col md:flex-row gap-2 mb-2 justify-between">
        <div class="flex flex-col md:flex-row gap-2 justify-start"></div>
        <VaButton @click="showAddReceiverModal">{{ t('contact.add_receiver') }}</VaButton>
      </div>

      <ReceiversTable
        :receivers="receiversStore.items"
        :loading="receiversStore.loading"
        @editReceiver="showEditReceiverModal"
        @deleteReceiver="showDeleteReceiverModal"
      />
    </VaCardContent>
  </VaCard>

  <VaModal
    v-slot="{ cancel, ok }"
    v-model="doShowEditReceiverModal"
    size="small"
    mobile-fullscreen
    hide-default-actions
    no-outside-dismiss
    :before-cancel="beforeEditFormModalClose"
  >
    <h1 class="va-h5">{{ receiverToEdit ? t('contact.edit_receiver') : t('contact.add_receiver') }}</h1>
    <EditReceiverForm
      ref="editFormRef"
      :receiver="receiverToEdit"
      :save-button-label="receiverToEdit ? t('user.save') : t('user.add')"
      @close="cancel"
      @save="
        (receiver) => {
          onReceiverSaved(receiver, ok)
        }
      "
    />
    <div v-if="!!error" class="flex gap-4 w-full">
      <VaAlert dense closeable color="danger" class="w-full">
        {{ error }}
      </VaAlert>
    </div>
  </VaModal>

  <VaModal
    v-slot="{ cancel, ok }"
    v-model="doShowDeleteReceiverModal"
    size="small"
    mobile-fullscreen
    hide-default-actions
    no-outside-dismiss
  >
    <h1 class="va-h5">{{ t('contact.delete_receiver') }}</h1>
    <p>{{ t('contact.confirm_delete', { name: receiverToEdit?.name }) }}</p>
    <div v-if="!!error" class="flex gap-4 w-full">
      <VaAlert dense closeable color="danger" class="w-full">
        {{ error }}
      </VaAlert>
    </div>
    <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
      <VaButton preset="secondary" color="secondary" @click="cancel">{{ t('vuestic.cancel') }}</VaButton>
      <VaButton @click="onReceiverDeleted(ok)">{{ t('contact.delete') }}</VaButton>
    </div>
  </VaModal>
</template>
