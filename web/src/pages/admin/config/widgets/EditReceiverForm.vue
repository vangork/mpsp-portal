<script setup lang="ts">
import { PropType, computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm } from 'vuestic-ui'
import { validators } from '../../../../services/utils'
import { Receiver } from '../types'

const { t } = useI18n()

const props = defineProps({
  receiver: {
    type: Object as PropType<Receiver | null>,
    default: null,
  },
  saveButtonLabel: {
    type: String,
    default: 'Save',
  },
})

type editReceiver = Omit<Receiver, 'id'>

const defaultNewReceiver: editReceiver = {
  name: '',
  email: '',
  phone: '',
  institution: '',
  default: false,
}

const newReceiver = ref<Receiver>({ ...defaultNewReceiver } as Receiver)

const isFormHasUnsavedChanges = computed(() => {
  return Object.keys(newReceiver.value).some((key) => {
    return (
      newReceiver.value[key as keyof editReceiver] !==
      (props.receiver ?? defaultNewReceiver)?.[key as keyof editReceiver]
    )
  })
})

defineExpose({
  isFormHasUnsavedChanges,
})

watch(
  [() => props.receiver],
  () => {
    if (!props.receiver) {
      return
    }

    newReceiver.value = {
      ...props.receiver,
    }
  },
  { immediate: true },
)

const form = useForm('add-receiver-form')

const emit = defineEmits(['close', 'save'])

const onSave = () => {
  if (form.validate()) {
    emit('save', newReceiver.value)
  }
}
</script>

<template>
  <VaForm
    v-slot="{ isValid }"
    ref="add-receiver-form"
    class="flex-col justify-start items-start gap-4 inline-flex w-full"
  >
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newReceiver.name"
          :label="t('contact.name')"
          class="w-full sm:w-1/2"
          :rules="[validators.required]"
          required-mark
        />
        <VaInput
          v-model="newReceiver.institution"
          :label="t('contact.address')"
          class="w-full sm:w-1/2"
          :rules="[validators.required]"
          required-mark
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newReceiver.phone"
          :label="t('contact.phone')"
          class="w-full sm:w-1/2"
          :rules="[validators.required]"
          required-mark
        />
        <VaInput
          v-model="newReceiver.email"
          :label="t('contact.email')"
          class="w-full sm:w-1/2"
          :rules="[validators.required, validators.email]"
          required-mark
        />
      </div>

      <div class="flex gap-4 w-full">
        <div class="flex items-center w-1/2 mt-4">
          <VaCheckbox
            v-model="newReceiver.default"
            :label="t('contact.default_receiver')"
            class="w-full"
            name="default"
          />
        </div>
      </div>

      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton preset="secondary" color="secondary" @click="$emit('close')">{{ t('vuestic.cancel') }}</VaButton>
        <VaButton :disabled="!isValid" @click="onSave">{{ saveButtonLabel }}</VaButton>
      </div>
    </div>
  </VaForm>
</template>
