<template>
  <div class="flex flex-col space-y-6 md:space-y-4">
    <h1 class="page-title">{{ t('menu.other_config') }}</h1>
    <VaForm v-slot="{ isValid }" ref="other-config-form">
      <div class="flex flex-col p-4 space-y-4 bg-backgroundSecondary rounded-lg">
        <div class="flex items-center justify-between">
          <p class="w-full sm:w-1/5">样本类型</p>
          <div class="w-full sm:w-4/5">
            <VaInput
              v-model="config.sample_types"
              placeholder="请按顺序输入样本类型，使用逗号分隔"
              class="w-full"
              :rules="[validators.required]"
              required-mark
            />
          </div>
        </div>

        <div class="flex items-center justify-between">
          <p class="w-full sm:w-1/5">检测项目</p>
          <div class="w-full sm:w-4/5">
            <VaInput
              v-model="config.test_items"
              placeholder="请按顺序输入检测项目，使用逗号分隔"
              class="w-full"
              :rules="[validators.required]"
              required-mark
            />
          </div>
        </div>

        <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
          <VaButton :disabled="!isValid" @click="onSave">{{ t('user.save') }}</VaButton>
        </div>
      </div>
    </VaForm>
  </div>

  <VaModal
    v-slot="{ cancel, ok }"
    v-model="doShowSaveModal"
    size="small"
    mobile-fullscreen
    hide-default-actions
    no-outside-dismiss
  >
    <h1 class="va-h5">{{ t('config.preview') }}</h1>
    <div class="flex-col justify-start items-start gap-4 inline-flex w-full">
      <div class="self-stretch flex-col justify-start items-start gap-4 flex">
        <div class="flex gap-4 flex-col sm:flex-row w-full">
          <VaSelect
            label="样本类型"
            class="w-full sm:w-1/2"
            placeholder="请选择"
            :options="parseOptions(config.sample_types)"
          />
          <VaSelect
            label="检测项目"
            class="w-full sm:w-1/2"
            placeholder="请选择"
            :options="parseOptions(config.test_items)"
          />
        </div>
      </div>
      <div v-if="!!error" class="flex gap-4 w-full">
        <VaAlert dense closeable color="danger" class="w-full">
          {{ error }}
        </VaAlert>
      </div>
      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton preset="secondary" color="secondary" @click="cancel">{{ t('vuestic.cancel') }}</VaButton>
        <VaButton @click="onSaveConfirmed(ok)">{{ t('user.save') }}</VaButton>
      </div>
    </div>
  </VaModal>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm, useToast } from 'vuestic-ui'
import { Config } from './types'
import { validators, parseOptions } from '../../../services/utils'
import { getConfig, updateConfig } from '../../../data/pages/config'

const { t } = useI18n()
const { init } = useToast()

const config = ref<Config>({
  sample_types: '加载中...',
  test_items: '加载中...',
})

const doShowSaveModal = ref(false)
const error = ref<string | null>(null)

const form = useForm('other-config-form')

const onSave = () => {
  if (form.validate()) {
    error.value = null
    doShowSaveModal.value = true
  }
}

const onSaveConfirmed = async (ok: () => void) => {
  try {
    error.value = null
    await updateConfig(config.value)
  } catch (err) {
    error.value = err instanceof Error ? `${err.name}: ${err.message}` : String(err)
  }
  if (!error.value) {
    ok()
  }
}

onMounted(async () => {
  try {
    const res = await getConfig()
    config.value = res
  } catch (err) {
    init({ message: `Failed to get config: ${String(err)}`, color: 'danger' })
  }
})
</script>
