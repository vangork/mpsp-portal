<template>
  <VaModal
    max-width="530px"
    :mobile-fullscreen="false"
    hide-default-actions
    model-value
    close-button
    @update:modelValue="emits('cancel')"
  >
    <h1 class="va-h5 mb-4">{{ t('auth.reset_password') }}</h1>
    <VaForm ref="form" class="space-y-6" @submit.prevent="submit">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <VaInput
          v-model="oldPassowrd"
          :rules="oldPasswordRules"
          :label="t('auth.old_password')"
          :placeholder="t('auth.old_password')"
          required-mark
          type="password"
        />
        <div class="hidden md:block" />
        <VaInput
          v-model="newPassword"
          :rules="newPasswordRules"
          :label="t('auth.new_password')"
          :placeholder="t('auth.new_password')"
          required-mark
          type="password"
        />
        <VaInput
          v-model="repeatNewPassword"
          :rules="repeatNewPasswordRules"
          :label="t('auth.repeat_new_password')"
          :placeholder="t('auth.repeat_new_password')"
          required-mark
          type="password"
        />
      </div>
      <div class="flex flex-col space-y-2">
        <div class="flex space-x-2 items-center">
          <div>
            <VaIcon :name="newPassword?.length! >= 8 ? 'mso-check' : 'mso-close'" color="secondary" size="20px" />
          </div>
          <p>{{ t('auth.at_least_8_long') }}</p>
        </div>
        <div class="flex space-x-2 items-center">
          <div>
            <VaIcon :name="new Set(newPassword).size >= 6 ? 'mso-check' : 'mso-close'" color="secondary" size="20px" />
          </div>
          <p>{{ t('auth.at_least_6_unique') }}</p>
        </div>
        <div v-if="!!errorMessage" class="flex space-x-2 items-center">
          <div>
            <VaIcon name="mso-close" color="danger" size="20px" />
          </div>
          <p class="error">{{ errorMessage }}</p>
        </div>
      </div>
      <div class="flex flex-col-reverse md:justify-end md:flex-row md:space-x-4">
        <VaButton :style="buttonStyles" preset="secondary" color="secondary" @click="emits('cancel')">{{
          t('vuestic.cancel')
        }}</VaButton>
        <VaButton :style="buttonStyles" class="mb-4 md:mb-0" @click="submit">{{ t('auth.update_password') }}</VaButton>
      </div>
    </VaForm>
  </VaModal>
</template>
<script lang="ts" setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm } from 'vuestic-ui'
import { Password } from '../../../auth/types'
import { resetPassword } from '../../../../data/pages/auth'
import { buttonStyles } from '../styles'

const { t } = useI18n()

const oldPassowrd = ref<string>()
const newPassword = ref<string>()
const repeatNewPassword = ref<string>()
const errorMessage = ref<string>()

const { validate } = useForm('form')

const emits = defineEmits(['cancel'])

const submit = async () => {
  if (validate()) {
    const password: Password = {
      current_password: oldPassowrd.value!,
      new_password: newPassword.value!,
    }
    try {
      errorMessage.value = ''
      await resetPassword(password)
      emits('cancel')
    } catch (error) {
      errorMessage.value = `${error}`
    }
  }
}

const oldPasswordRules = [(v: string) => !!v || 'Old password field is required']

const newPasswordRules = [
  (v: string) => !!v || 'New password field is required',
  (v: string) => v?.length >= 8 || 'Must be at least 8 characters long',
  (v: string) => new Set(v).size >= 6 || 'Must contain at least 6 unique characters',
  (v: string) => v !== oldPassowrd.value || 'New password cannot be the same',
]

const repeatNewPasswordRules = [
  (v: string) => !!v || 'Repeat new password field is required',
  (v: string) => v === newPassword.value || 'Confirm password does not match new password',
]
</script>

<style lang="scss">
// TODO temporary before https://github.com/epicmaxco/vuestic-ui/issues/4020 fix
.va-modal__inner {
  min-width: 326px;
}
.error {
  color: var(--va-danger);
}
</style>
