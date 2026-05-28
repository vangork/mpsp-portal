<script setup lang="ts">
import { PropType, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { AdminBill, FeeItem } from '../types'

const { t } = useI18n()

const props = defineProps({
  item: {
    type: Object as PropType<AdminBill>,
    required: true,
  },
})

const cloneFee = (fee: FeeItem): FeeItem => ({ ...fee })

const newItem = ref<AdminBill>({
  ...props.item,
  extractionFee: cloneFee(props.item.extractionFee),
  qualityCheckFee: cloneFee(props.item.qualityCheckFee),
  libraryFee: cloneFee(props.item.libraryFee),
  sequencingFee: cloneFee(props.item.sequencingFee),
  analysisServiceFee: cloneFee(props.item.analysisServiceFee),
})

const emit = defineEmits(['close', 'save'])

const onSave = () => {
  emit('save', newItem.value)
}

const feeRows: {
  label: string
  key: keyof Pick<
    AdminBill,
    'extractionFee' | 'qualityCheckFee' | 'libraryFee' | 'sequencingFee' | 'analysisServiceFee'
  >
}[] = [
  { label: '核算提取费', key: 'extractionFee' },
  { label: '质检费', key: 'qualityCheckFee' },
  { label: '建库费', key: 'libraryFee' },
  { label: '测序费', key: 'sequencingFee' },
  { label: '个性化数据分析费', key: 'analysisServiceFee' },
]

const statusOptions = ref<{ text: string; value: number }[]>([
  { text: '未开始', value: 0 },
  { text: '已发送报价单', value: 1 },
  { text: '已开票', value: 2 },
  { text: '已打款', value: 3 },
])
</script>

<template>
  <VaForm class="flex-col justify-start items-start gap-4 inline-flex w-full">
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput :model-value="newItem.institution" :label="t('rna.name')" class="w-full sm:w-1/2" disabled />
        <VaInput :model-value="newItem.contact" :label="t('rna.name')" class="w-full sm:w-1/2" disabled />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="flex w-full sm:w-1/3">
          <div class="align-bottom font-bold">核算提取费</div>
        </div>
        <VaInput
          :model-value="newItem.extractionFee.unitPrice"
          type="number"
          :label="t('billing.unit_price')"
          class="w-full sm:w-1/3"
        />
        <VaInput
          :model-value="newItem.extractionFee.totalPrice"
          type="number"
          :label="t('billing.total_price')"
          class="w-full sm:w-1/3"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="flex w-full sm:w-1/3">
          <div class="align-bottom font-bold">质检费</div>
        </div>
        <VaInput
          :model-value="newItem.qualityCheckFee.unitPrice"
          type="number"
          :label="t('billing.unit_price')"
          class="w-full sm:w-1/3"
        />
        <VaInput
          :model-value="newItem.qualityCheckFee.totalPrice"
          type="number"
          :label="t('billing.total_price')"
          class="w-full sm:w-1/3"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="flex w-full sm:w-1/3">
          <div class="align-bottom font-bold">建库费</div>
        </div>
        <VaInput
          :model-value="newItem.libraryFee.unitPrice"
          type="number"
          :label="t('billing.unit_price')"
          class="w-full sm:w-1/3"
        />
        <VaInput
          :model-value="newItem.libraryFee.totalPrice"
          type="number"
          :label="t('billing.total_price')"
          class="w-full sm:w-1/3"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="flex w-full sm:w-1/3">
          <div class="align-bottom font-bold">测序费</div>
        </div>
        <VaInput
          :model-value="newItem.sequencingFee.unitPrice"
          type="number"
          :label="t('billing.unit_price')"
          class="w-full sm:w-1/3"
        />
        <VaInput
          :model-value="newItem.sequencingFee.totalPrice"
          type="number"
          :label="t('billing.total_price')"
          class="w-full sm:w-1/3"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="flex w-full sm:w-1/3">
          <div class="align-bottom font-bold">个性化数据分析费</div>
        </div>
        <VaInput
          :model-value="newItem.analysisServiceFee.unitPrice"
          type="number"
          :label="t('billing.unit_price')"
          class="w-full sm:w-1/3"
        />
        <VaInput
          :model-value="newItem.analysisServiceFee.totalPrice"
          type="number"
          :label="t('billing.total_price')"
          class="w-full sm:w-1/3"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="flex w-full sm:w-1/3">
          <div class="align-bottom font-bold">总金额</div>
        </div>
        <VaInput model-value="0" type="number" :label="t('billing.unit_price')" readonly class="w-full sm:w-1/3" />
        <div class="flex w-full sm:w-1/3" style="min-width: 181.33px">
          <div class="align-bottom-0">
            <VaButton preset="primary" @click="">下载报价单</VaButton>
          </div>
        </div>
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.status"
          :options="statusOptions"
          label="财务状态"
          class="w-full sm:w-1/2"
          placeholder="请选择"
        />
      </div>

      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton preset="secondary" color="secondary" @click="emit('close')">{{ t('vuestic.cancel') }} </VaButton>
        <VaButton @click="onSave">{{ t('omics.save') }}</VaButton>
      </div>
    </div>
  </VaForm>
</template>

<style lang="scss" scoped>
.align-bottom {
  display: flex;
  align-items: flex-end;
  margin-bottom: 10px;
}

.align-bottom-0 {
  display: flex;
  align-items: flex-end;
}
</style>
