<script setup lang="ts">
import { PropType } from 'vue'
import { useI18n } from 'vue-i18n'
import { AdminDNAItem } from '../../omics/types'
import { formatDate } from '../../../omics/composables/useOmics'
import { DNAFamilyMember } from '../../../omics/types'

const { t } = useI18n()

const props = defineProps({
  item: {
    type: Object as PropType<AdminDNAItem | null>,
    default: null,
  },
})

const familyMemberSelectOptions: { text: string; value: DNAFamilyMember }[] = [
  { text: t('dna.d5'), value: 5 },
  { text: t('dna.d6'), value: 6 },
  { text: t('dna.f7'), value: 7 },
  { text: t('dna.m8'), value: 8 },
]

const emit = defineEmits(['close'])
</script>

<template>
  <VaForm class="flex-col justify-start items-start gap-4 inline-flex w-full">
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="props.item?.name" :label="t('dna.name')" class="w-full sm:w-1/2" readonly />
        <VaInput
          :model-value="familyMemberSelectOptions.find((option) => option.value === props.item?.family_member)?.text"
          :label="t('dna.sample_id')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="props.item?.platform_model"
          :label="t('dna.platform_model')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput
          :model-value="props.item?.dna_fragmentation"
          :label="t('dna.dna_fragmentation')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="props.item?.library_protocol"
          :label="t('dna.library_protocol')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput
          :model-value="props.item?.application"
          :label="t('dna.application')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="props.item?.read_mode" :label="t('dna.read_mode')" class="w-full sm:w-1/2" readonly />
        <VaInput :model-value="props.item?.reference" :label="t('dna.reference')" class="w-full sm:w-1/2" readonly />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          :model-value="props.item?.alignment_software"
          :label="t('dna.alignment_software')"
          class="w-full sm:w-1/2"
          readonly
        />
        <VaInput
          :model-value="props.item?.variant_caller"
          :label="t('dna.variant_caller')"
          class="w-full sm:w-1/2"
          readonly
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="props.item?.filtering" :label="t('dna.filtering')" class="w-full sm:w-1/2" readonly />
        <div class="w-full sm:w-1/2">
          <VaDateInput
            :model-value="props.item?.determinated_at"
            :label="t('dna.determinated_at')"
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
