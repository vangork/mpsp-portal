<script setup lang="ts">
import { PropType, toRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { AdminMetabolismItem } from '../types'
import { formatDate } from '../../../omics/composables/useOmics'
const { t } = useI18n()

const props = defineProps({
  item: {
    type: Object as PropType<AdminMetabolismItem>,
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
        <VaInput :model-value="item.name" :label="t('metabolism.name')" class="w-full sm:w-1/2" readonly />
        <VaInput
          :model-value="item.ms_instrument"
          :label="t('metabolism.ms_instrument')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.detection_strategy"
          :label="t('metabolism.detection_strategy')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput
          :model-value="item.analysis_platform"
          :label="t('metabolism.analysis_platform')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.chromatography"
          :label="t('metabolism.chromatography')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput :model-value="item.ionization" :label="t('metabolism.ionization')" class="w-full sm:w-1/2" readonly />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.extraction_method"
          :label="t('metabolism.extraction_method')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput
          :model-value="item.preprocessing_software"
          :label="t('metabolism.preprocessing_software')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.identification_level"
          :label="t('metabolism.identification_level')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput :model-value="item.unit" :label="t('metabolism.unit')" class="w-full sm:w-1/2" readonly />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="item.database" :label="t('metabolism.database')" class="w-full sm:w-1/2" readonly />
        <div class="w-full sm:w-1/2">
          <VaDateInput
            :model-value="item.determinated_at"
            :label="t('metabolism.determinated_at')"
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
