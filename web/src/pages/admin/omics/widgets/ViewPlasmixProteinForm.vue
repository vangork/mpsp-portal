<script setup lang="ts">
import { PropType, toRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { AdminPlasmixProteinItem } from '../types'
import { formatDate } from '../../../omics/composables/useOmics'
const { t } = useI18n()

const props = defineProps({
  item: {
    type: Object as PropType<AdminPlasmixProteinItem>,
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
        <VaInput :model-value="item.name" :label="t('plasmix_protein.name')" class="w-full sm:w-1/2" readonly />
        <VaInput
          :model-value="item.technology"
          :label="t('plasmix_protein.technology')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.model_panel"
          :label="t('plasmix_protein.model_panel')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput
          :model-value="item.pre_treatment"
          :label="t('plasmix_protein.pre_treatment')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="item.readout" :label="t('plasmix_protein.readout')" class="w-full sm:w-1/2" readonly />
        <VaInput :model-value="item.protocol" :label="t('plasmix_protein.protocol')" class="w-full sm:w-1/2" readonly />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.expression_unit"
          :label="t('plasmix_protein.expression_unit')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput
          :model-value="item.analysis_software"
          :label="t('plasmix_protein.analysis_software')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="item.database" :label="t('plasmix_protein.database')" class="w-full sm:w-1/2" readonly />
        <VaInput
          :model-value="item.normalization"
          :label="t('plasmix_protein.normalization')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="w-full sm:w-1/2">
          <VaDateInput
            :model-value="item.determinated_at"
            :label="t('plasmix_protein.determinated_at')"
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
