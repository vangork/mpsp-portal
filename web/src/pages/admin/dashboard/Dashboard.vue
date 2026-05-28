<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import RevenueUpdates from './cards/RevenueReport.vue'
import DataSection from './DataSection.vue'
import YearlyBreakup from './cards/YearlyBreakup.vue'
import MonthlyEarnings from './cards/MonthlyEarnings.vue'

const { t } = useI18n()
</script>

<template>
  <h1 class="page-title font-bold">{{ t('menu.overview') }}</h1>
  <section class="flex flex-col gap-4">
    <div class="flex flex-col sm:flex-row gap-4">
      <RevenueUpdates class="w-full sm:w-[70%]" />
      <div class="flex flex-col gap-4 w-full sm:w-[30%]">
        <YearlyBreakup class="h-full" />
        <MonthlyEarnings />
      </div>
    </div>
    <DataSection />
  </section>
</template>
