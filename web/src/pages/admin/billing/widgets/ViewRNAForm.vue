<script setup lang="ts">
import { PropType, toRef } from 'vue'
import { useI18n } from 'vue-i18n'
import { AdminRNAItem } from '../types'
import { formatDate } from '../../../omics/composables/useOmics'
const { t } = useI18n()

const props = defineProps({
  item: {
    type: Object as PropType<AdminRNAItem>,
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
        <VaInput :model-value="item.name" :label="t('rna.name')" class="w-full sm:w-1/2" readonly />
        <VaInput :model-value="item.platform_model" :label="t('rna.platform_model')" class="w-full sm:w-1/2" readonly />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.rna_fragmentation"
          :label="t('rna.rna_fragmentation')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput :model-value="item.enrichment" :label="t('rna.enrichment')" class="w-full sm:w-1/2" readonly />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.library_protocol"
          :label="t('rna.library_protocol')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput :model-value="item.read_mode" :label="t('rna.read_mode')" class="w-full sm:w-1/2" readonly />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="item.reference" :label="t('rna.reference')" class="w-full sm:w-1/2" readonly />
        <VaInput
          :model-value="item.alignment_software"
          :label="t('rna.alignment_software')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="item.quantification_tool"
          :label="t('rna.quantification_tool')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput
          :model-value="item.expression_unit"
          :label="t('rna.expression_unit')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="w-full sm:w-1/2">
          <VaDateInput
            :model-value="item.determinated_at"
            :label="t('rna.determinated_at')"
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
