<template>
  <VaAvatar size="large"
    ><span class="text-4xl"> <i class="fas fa-user"></i></span
  ></VaAvatar>
  <div class="flex flex-col justify-center">
    <h2 class="text-[28px] md:text-[32px] leading-10 font-bold">{{ store.profile.username }}</h2>
    <div class="flex space-x-1 text-[13px] leading-4">
      <p>{{ t('dashboard.member_since') }}:</p>
      <p>{{ store.profile.created_at.toLocaleString('en-CA') }}</p>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { useAuthStore } from '../../../../stores/auth'

const { t } = useI18n()
const store = useAuthStore()
</script>
