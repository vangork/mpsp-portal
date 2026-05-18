<script setup lang="ts">
import { PropType, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { AdminPlasmixMetabolismItem } from '../types'

const { t } = useI18n()

const props = defineProps({
  item: {
    type: Object as PropType<AdminPlasmixMetabolismItem>,
    required: true,
  },
})

const newItem = ref<AdminPlasmixMetabolismItem>({ ...props.item })

const emit = defineEmits(['close', 'save'])

const onSave = () => {
  emit('save', newItem.value)
}
</script>

<template>
  <VaForm class="flex-col justify-start items-start gap-4 inline-flex w-full">
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="newItem.name" :label="t('plasmix_metabolism.name')" class="w-full sm:w-1/2" disabled />
      </div>

      <VaTextarea v-model="newItem.notes!" :label="t('omics.notes')" class="w-full" name="notes" />

      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton preset="secondary" color="secondary" @click="emit('close')">{{ t('vuestic.cancel') }} </VaButton>
        <VaButton @click="onSave">{{ t('omics.save') }}</VaButton>
      </div>
    </div>
  </VaForm>
</template>
