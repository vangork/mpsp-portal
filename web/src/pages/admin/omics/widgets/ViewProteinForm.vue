<script setup lang="ts">
import { PropType, toRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { AdminProteinItem } from '../types'
import { formatDate } from '../../../omics/composables/useOmics'
const { t } = useI18n()

const props = defineProps({
  item: {
    type: Object as PropType<AdminProteinItem>,
    required: true,
  },
})

const item = toRef(props, 'item')

const emit = defineEmits(['close'])
</script>

<template>
  <VaForm class="flex-col justify-start items-start gap-4 inline-flex w-full">
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="item.name" :label="t('protein.name')" class="w-full sm:w-1/2" readonly />
        <VaInput
          :model-value="item.ms_instrument"
          :label="t('protein.ms_instrument')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.acquisition_method"
          :label="t('protein.acquisition_method')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput :model-value="item.lc_system" :label="t('protein.lc_system')" class="w-full sm:w-1/2" readonly />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="item.lc_column" :label="t('protein.lc_column')" class="w-full sm:w-1/2" readonly />
        <VaInput
          :model-value="item.fractionation"
          :label="t('protein.fractionation')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.digestion_enzyme"
          :label="t('protein.digestion_enzyme')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput
          :model-value="item.analysis_software"
          :label="t('protein.analysis_software')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="item.database" :label="t('protein.database')" class="w-full sm:w-1/2" readonly />
        <VaInput
          :model-value="item.normalization"
          :label="t('protein.normalization')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="w-full sm:w-1/2">
          <VaDateInput
            :model-value="item.determinated_at"
            :label="t('protein.determinated_at')"
            class="w-full"
            :format-date="formatDate"
            readonly
          />
        </div>
      </div>

      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton @click="emit('close')">{{ t('vuestic.close') }}</VaButton>
      </div>
    </div>
  </VaForm>
</template>
