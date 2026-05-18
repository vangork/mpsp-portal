<template>
  <div class="flex flex-col md:flex-row md:items-center space-y-2 md:space-y-0 md:space-x-6 min-h-[36px] leading-5">
    <p class="font-bold w-[200px]">{{ t('auth.username') }}</p>
    <div class="flex-1">
      <div class="max-w-[748px]">
        {{ store.profile.username }}
      </div>
    </div>
  </div>
  <VaDivider />
  <div class="flex flex-col md:flex-row space-y-2 md:space-y-0 md:space-x-6 min-h-[36px] leading-5">
    <p class="font-bold w-[200px]">{{ t('auth.email') }}</p>
    <div class="flex-1">
      <div class="max-w-[748px]">
        {{ store.profile.email }}
      </div>
    </div>
  </div>
  <div class="flex flex-col md:flex-row space-y-2 md:space-y-0 md:space-x-6 min-h-[36px] leading-5">
    <p class="font-bold w-[200px]">{{ t('auth.password') }}</p>
    <div class="flex-1">
      <div class="max-w-[748px]">•••••••••••••</div>
    </div>
    <VaButton :style="buttonStyles" class="w-fit h-fit" preset="primary" @click="emits('openResetPasswordModal')">
      {{ t('auth.reset_password') }}
    </VaButton>
  </div>
</template>
<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '../../../../stores/auth'
import { buttonStyles } from '../styles'

const store = useAuthStore()
const { t } = useI18n()

const emits = defineEmits(['openResetPasswordModal'])
</script>
