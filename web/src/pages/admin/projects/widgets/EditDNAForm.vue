<script setup lang="ts">
import { PropType, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { AdminDNAItem } from '../types'
import { DNAFamilyMember } from '../../../omics/types'

const { t } = useI18n()

const props = defineProps({
  item: {
    type: Object as PropType<AdminDNAItem>,
    required: true,
  },
})

const newItem = ref<AdminDNAItem>({ ...props.item })

const emit = defineEmits(['close', 'save'])

const onSave = () => {
  emit('save', newItem.value)
}

const familyMemberSelectOptions: { text: string; value: DNAFamilyMember }[] = [
  { text: t('dna.d5'), value: 5 },
  { text: t('dna.d6'), value: 6 },
  { text: t('dna.f7'), value: 7 },
  { text: t('dna.m8'), value: 8 },
]
</script>

<template>
  <VaForm class="flex-col justify-start items-start gap-4 inline-flex w-full">
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="newItem.name" :label="t('dna.name')" class="w-full sm:w-1/2" disabled />
        <VaInput
          :model-value="familyMemberSelectOptions.find((option) => option.value === newItem.family_member)?.text"
          :label="t('dna.sample_id')"
          class="w-full sm:w-1/2"
          disabled
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput v-model="newItem.snv_number" label="SNV No." class="w-full sm:w-1/2" type="number" />
        <VaInput v-model="newItem.indel_number" label="INDEL No." class="w-full sm:w-1/2" type="number" />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput v-model="newItem.snv_precision" label="SNV Precision" class="w-full sm:w-1/2" type="number" />
        <VaInput v-model="newItem.indel_precision" label="INDEL Precision" class="w-full sm:w-1/2" type="number" />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput v-model="newItem.snv_recall" label="SNV Recall" class="w-full sm:w-1/2" type="number" />
        <VaInput v-model="newItem.indel_recall" label="INDEL Recall" class="w-full sm:w-1/2" type="number" />
      </div>

      <VaTextarea v-model="newItem.notes!" :label="t('omics.notes')" class="w-full" name="notes" />

      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton preset="secondary" color="secondary" @click="emit('close')">{{ t('vuestic.cancel') }} </VaButton>
        <VaButton @click="onSave">{{ t('omics.save') }}</VaButton>
      </div>
    </div>
  </VaForm>
</template>
