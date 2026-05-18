<template>
  <h1 class="page-title">{{ t('user.profile') }}</h1>
  <div class="flex flex-col p-4 space-y-10 bg-backgroundSecondary rounded-lg">
    <div class="flex space-x-5">
      <PreferencesHeader />
    </div>
    <div class="space-y-4 md:space-y-6">
      <Settings @openResetPasswordModal="isResetPasswordModalOpen = true" />
    </div>
  </div>
  <ResetPasswordModal v-if="isResetPasswordModalOpen" @cancel="isResetPasswordModalOpen = false" />
</template>
<script lang="ts" setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

import PreferencesHeader from './preferences-header/PreferencesHeader.vue'
import Settings from './settings/Settings.vue'
import ResetPasswordModal from './modals/ResetPasswordModal.vue'

const { t } = useI18n()
const isResetPasswordModalOpen = ref(false)
</script>
