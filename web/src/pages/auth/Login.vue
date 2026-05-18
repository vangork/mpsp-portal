<template>
  <VaForm ref="form" @submit.prevent="submit">
    <h1 class="font-semibold text-4xl mb-4">{{ t('auth.login') }}</h1>
    <p class="text-base mb-4 leading-5">{{ t('auth.welcome') }} <i>Q<sup>4</sup>Omics</i> Portal</p>
    <VaInput
      v-model="formData.email"
      :rules="[validators.required, validators.email]"
      class="mb-4"
      :label="t('auth.email')"
      type="email"
    />
    <VaValue v-slot="isPasswordVisible" :default-value="false">
      <VaInput
        v-model="formData.password"
        :rules="[validators.required]"
        :type="isPasswordVisible.value ? 'text' : 'password'"
        class="mb-4"
        :label="t('auth.password')"
        @clickAppendInner.stop="isPasswordVisible.value = !isPasswordVisible.value"
      >
        <template #appendInner>
          <VaIcon
            :name="isPasswordVisible.value ? 'mso-visibility_off' : 'mso-visibility'"
            class="cursor-pointer"
            color="secondary"
          />
        </template>
      </VaInput>
    </VaValue>
    <VaAlert v-model="errorMessageVisible" color="danger" closeable>
      {{ errorMessage }}
    </VaAlert>
    <div class="flex justify-center mt-4">
      <VaButton class="w-full" :disabled="authStore.isLogining" type="submit">{{ t('auth.login') }}</VaButton>
    </div>
  </VaForm>
</template>

<script lang="ts" setup>
import { computed, reactive, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { useForm } from 'vuestic-ui'
import { validators } from '../../services/utils'
import { useAuthStore } from '../../stores/auth'

const { validate } = useForm('form')
const authStore = useAuthStore()
const { t } = useI18n()
const { push } = useRouter()

const formData = reactive({
  email: '',
  password: '',
})

const errorMessage = ref('')
const errorMessageVisible = computed({
  get() {
    return errorMessage.value.length > 0
  },
  set() {
    errorMessage.value = ''
  },
})

const submit = async () => {
  errorMessage.value = ''
  if (validate()) {
    try {
      await authStore.login(formData.email, formData.password)
      push({ name: 'dashboard' })
    } catch (err) {
      if (err instanceof Error) {
        if (err.name == '401') {
          errorMessage.value = t('auth.error_401')
        } else {
          errorMessage.value = err.toString()
        }
      } else {
        errorMessage.value = 'unknown error'
      }
    }
  }
}
</script>
