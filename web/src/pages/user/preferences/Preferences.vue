<template>
  <div class="flex flex-col space-y-6 md:space-y-4">
    <h1 class="page-title">{{ t('user.preferences') }}</h1>
    <!-- <div class="flex flex-col p-4 space-y-4 bg-backgroundSecondary rounded-lg">
      <h3 class="h3">{{ t('user.theme') }}</h3>
      <ThemeSwitcher />
    </div> -->
    <div class="flex flex-col p-4 space-y-4 bg-backgroundSecondary rounded-lg">
      <!-- <h3 class="h3">{{ t('user.general_preferences') }}</h3> -->
      <LanguageSwitcher />
    </div>
  </div>
</template>
<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import LanguageSwitcher from './language-switcher/LanguageSwitcher.vue'
// import ThemeSwitcher from './theme-switcher/ThemeSwitcher.vue'

const { t } = useI18n()
</script>
