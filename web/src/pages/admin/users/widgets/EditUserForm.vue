<script setup lang="ts">
import { PropType, computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm } from 'vuestic-ui'
import { validators } from '../../../../services/utils'
import { User, UserRole, UserRoleKeys } from '../types'

const { t } = useI18n()

const props = defineProps({
  user: {
    type: Object as PropType<User | null>,
    default: null,
  },
  saveButtonLabel: {
    type: String,
    default: 'Save',
  },
})

type editUser = Omit<User, 'id' | 'last_seen'>

const defaultNewUser: editUser = {
  username: '',
  email: '',
  password: '',
  role: 0,
  active: true,
  notes: '',
}

const newUser = ref<User>({ ...defaultNewUser } as User)
const confirmPassword = ref('')

const isFormHasUnsavedChanges = computed(() => {
  return Object.keys(newUser.value).some((key) => {
    return newUser.value[key as keyof editUser] !== (props.user ?? defaultNewUser)?.[key as keyof editUser]
  })
})

defineExpose({
  isFormHasUnsavedChanges,
})

watch(
  [() => props.user],
  () => {
    if (!props.user) {
      return
    }

    newUser.value = {
      ...props.user,
    }
  },
  { immediate: true },
)

const form = useForm('add-user-form')

const emit = defineEmits(['close', 'save'])

const onSave = () => {
  if (form.validate()) {
    emit('save', newUser.value)
  }
}

const roleSelectOptions: { text: Capitalize<Lowercase<UserRole>>; value: UserRoleKeys }[] = [
  { text: 'Admin', value: 2 },
  { text: 'Owner', value: 1 },
  { text: 'User', value: 0 },
]

const repeatNewPasswordRules = [(v: string) => v === newUser.value.password || t('auth.passwords_do_not_match')]
</script>

<template>
  <VaForm v-slot="{ isValid }" ref="add-user-form" class="flex-col justify-start items-start gap-4 inline-flex w-full">
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newUser.username"
          :label="t('auth.username')"
          class="w-full sm:w-1/2"
          :rules="[validators.required]"
          required-mark
        />
        <VaInput
          v-model="newUser.email"
          :label="t('auth.email')"
          class="w-full sm:w-1/2"
          :rules="[validators.required, validators.email]"
          required-mark
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newUser.password"
          :label="t('auth.new_password')"
          :placeholder="t('user.leave_empty_unchanged')"
          class="w-full sm:w-1/2"
          :rules="!props.user ? [validators.required] : []"
          type="password"
          :required-mark="!props.user"
        />
        <VaInput
          v-model="confirmPassword"
          :label="t('auth.repeat_new_password')"
          :placeholder="t('user.leave_empty_unchanged')"
          class="w-full sm:w-1/2"
          :rules="repeatNewPasswordRules"
          type="password"
          :required-mark="!props.user"
        />
      </div>

      <div class="flex gap-4 w-full">
        <div class="w-1/2">
          <VaSelect
            v-model="newUser.role"
            :label="t('user.role')"
            class="w-full"
            :options="roleSelectOptions"
            :rules="[validators.required]"
            name="role"
            value-by="value"
          />
        </div>

        <div class="flex items-center w-1/2 mt-4">
          <VaCheckbox v-model="newUser.active" label="Active" class="w-full" name="active" />
        </div>
      </div>

      <VaTextarea v-model="newUser.notes" :label="t('user.notes')" class="w-full" name="notes" />
      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton preset="secondary" color="secondary" @click="$emit('close')">{{ t('vuestic.cancel') }}</VaButton>
        <VaButton :disabled="!isValid" @click="onSave">{{ saveButtonLabel }}</VaButton>
      </div>
    </div>
  </VaForm>
</template>
