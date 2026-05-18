<script setup lang="ts">
import Papa from 'papaparse'
import { PropType, computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm } from 'vuestic-ui'
import {
  ProteinItem,
  ProteinMsInstrument,
  ProteinAcquisitionMethod,
  ProteinLcSystem,
  ProteinLcColumn,
  ProteinFactionation,
  ProteinDigestionEnzyme,
  ProteinAnalysisSoftware,
  ProteinDatabase,
  ProteinNormalization,
  AliOSSToken,
} from '../types'
import { validators, humanFileSize } from '../../../services/utils'
import { qcQuartetProtein } from '../../../services/webr'
import { addNewOption, formatDate, makeAliOssClient, renameFile } from '../composables/useOmics'

const { t } = useI18n()

const prefix = 'protein'

const props = defineProps({
  item: {
    type: Object as PropType<ProteinItem | null>,
    default: null,
  },
  token: {
    type: Object as PropType<AliOSSToken | null>,
    default: null,
  },
  saveButtonLabel: {
    type: String,
    default: 'Save',
  },
})

type editProteinItem = Omit<ProteinItem, 'id' | 'user_id' | 'report_file' | 'notes'>

const defaultNewItem: editProteinItem = {
  name: '',
  ms_instrument: null,
  acquisition_method: null,
  lc_system: null,
  lc_column: null,
  fractionation: null,
  digestion_enzyme: null,
  analysis_software: null,
  database: null,
  normalization: null,
  determinated_at: new Date(),
  meta_file: '',
  exp_file: '',
  features: 0,
  recall: 0,
  snr: 0,
  pcc: 0,
  d5: 0,
  d6: 0,
  f7: 0,
  m8: 0,
}

const newItem = ref<ProteinItem>({ ...defaultNewItem } as ProteinItem)

watch(
  [() => props.item],
  () => {
    if (!props.item) {
      return
    }

    newItem.value = {
      ...props.item,
    }
  },
  { immediate: true },
)

const isFormHasUnsavedChanges = computed(() => {
  return Object.keys(newItem.value).some((key) => {
    return (
      newItem.value[key as keyof editProteinItem] !== (props.item ?? defaultNewItem)?.[key as keyof editProteinItem]
    )
  })
})

defineExpose({
  isFormHasUnsavedChanges,
})

const metaFile = ref<File>()
const expFile = ref<File>()
const qcProcessing = ref(false)
const errorMsg = ref('')

const form = useForm('add-protein-form')

const emit = defineEmits(['close', 'save'])

const onSave = async () => {
  errorMsg.value = ''
  if (form.validate()) {
    try {
      qcProcessing.value = true
      if (props.token) {
        const qc_result = await qcQuartetProtein(metaFile.value!, expFile.value!)
        if (qc_result[6] === Infinity) {
          throw new Error('SNR is Infinity.')
        }
        newItem.value.features = qc_result[3]!
        newItem.value.recall = qc_result[4]!
        newItem.value.snr = qc_result[6]!
        newItem.value.pcc = qc_result[5]!

        const metaData = Papa.parse(await metaFile.value!.text(), {
          header: true,
          transformHeader: function (header) {
            return header.toLowerCase()
          },
        }).data
        const d5 = metaData.filter((row: any) => !!row.sample && row.sample.trim().toLowerCase() === 'd5').length
        const d6 = metaData.filter((row: any) => !!row.sample && row.sample.trim().toLowerCase() === 'd6').length
        const f7 = metaData.filter((row: any) => !!row.sample && row.sample.trim().toLowerCase() === 'f7').length
        const m8 = metaData.filter((row: any) => !!row.sample && row.sample.trim().toLowerCase() === 'm8').length
        newItem.value.d5 = d5
        newItem.value.d6 = d6
        newItem.value.f7 = f7
        newItem.value.m8 = m8

        const oSSClient = makeAliOssClient(props.token)
        let result = await oSSClient.put(renameFile(prefix, metaFile.value!.name), metaFile.value!)
        console.log(`${metaFile.value!.name} uploaded to ${result.url}`)
        newItem.value.meta_file = result.name

        result = await oSSClient.put(renameFile(prefix, expFile.value!.name), expFile.value!)
        console.log(`${expFile.value!.name} uploaded to ${result.url}`)
        newItem.value.exp_file = result.name
      }
      emit('save', newItem.value)
    } catch (e) {
      if (e instanceof Error) {
        errorMsg.value = e.message
      }
    } finally {
      qcProcessing.value = false
    }
  }
}

const msInstrumentOptions = ref<{ text: ProteinMsInstrument; value: ProteinMsInstrument }[]>([
  { text: 'Thermo Orbitrap Exploris 480/240', value: 'Thermo Orbitrap Exploris 480/240' },
  { text: 'Thermo Orbitrap Astral', value: 'Thermo Orbitrap Astral' },
  { text: 'Thermo Q Exactive HF-X/HF', value: 'Thermo Q Exactive HF-X/HF' },
  { text: 'Bruker timsTOF Pro/Pro 2', value: 'Bruker timsTOF Pro/Pro 2' },
  { text: 'Bruker timsTOF HT', value: 'Bruker timsTOF HT' },
  { text: 'SCIEX ZenoTOF 7600', value: 'SCIEX ZenoTOF 7600' },
  { text: 'SCIEX TripleTOF 6600+', value: 'SCIEX TripleTOF 6600+' },
  { text: 'Waters SELECT SERIES Cyclic IMS', value: 'Waters SELECT SERIES Cyclic IMS' },
])
const acquisitionMethodOptions = ref<{ text: ProteinAcquisitionMethod; value: ProteinAcquisitionMethod }[]>([
  { text: 'DIA', value: 'DIA' },
  { text: 'DDA', value: 'DDA' },
  { text: 'diaPASEF', value: 'diaPASEF' },
  { text: 'ddaPASEF', value: 'ddaPASEF' },
  { text: 'PRM', value: 'PRM' },
  { text: 'BoxCar', value: 'BoxCar' },
  { text: 'SureQuant', value: 'SureQuant' },
])
const lcSystemOptions = ref<{ text: ProteinLcSystem; value: ProteinLcSystem }[]>([
  { text: 'Thermo Vanquish Neo', value: 'Thermo Vanquish Neo' },
  { text: 'Waters ACQUITY UPLC M-Class', value: 'Waters ACQUITY UPLC M-Class' },
  { text: 'Evosep One', value: 'Evosep One' },
  { text: 'Agilent 1290 Infinity II', value: 'Agilent 1290 Infinity II' },
  { text: 'Shimadzu Nexera', value: 'Shimadzu Nexera' },
])
const lcColumnOptions = ref<{ text: ProteinLcColumn; value: ProteinLcColumn }[]>([
  { text: 'C18 Reverse Phase', value: 'C18 Reverse Phase' },
  { text: 'Ion Exchange', value: 'Ion Exchange' },
  { text: 'Integrated Spray Tip Column', value: 'Integrated Spray Tip Column' },
  { text: 'Performance Column(Evosep)', value: 'Performance Column(Evosep)' },
])
const fractionationOptions = ref<{ text: string; value: ProteinFactionation }[]>([
  { text: 'No Fractionation', value: 'No Fractionation' },
  { text: 'High-pH Reverse Phase', value: 'High-pH Reverse Phase' },
  { text: 'SCX', value: 'SCX' },
  { text: 'GeLC-MS', value: 'GeLC-MS' },
])
const digestionEnzymeOptions = ref<{ text: ProteinDigestionEnzyme; value: ProteinDigestionEnzyme }[]>([
  { text: 'Trypsin', value: 'Trypsin' },
  { text: 'Lys-C', value: 'Lys-C' },
  { text: 'Trypsin/Lys-C Mix', value: 'Trypsin/Lys-C Mix' },
  { text: 'Chymotrypsin', value: 'Chymotrypsin' },
  { text: 'Glu-C', value: 'Glu-C' },
  { text: 'Pepsin', value: 'Pepsin' },
])
const analysisSoftwareOptions = ref<{ text: ProteinAnalysisSoftware; value: ProteinAnalysisSoftware }[]>([
  { text: 'DIA-NN', value: 'DIA-NN' },
  { text: 'Spectronaut', value: 'Spectronaut' },
  { text: 'MaxQuant', value: 'MaxQuant' },
  { text: 'Proteome Discoverer', value: 'Proteome Discoverer' },
  { text: 'FragPipe', value: 'FragPipe' },
  { text: 'Skyline', value: 'Skyline' },
  { text: 'PEAKS Online', value: 'PEAKS Online' },
  { text: 'PaSER', value: 'PaSER' },
])
const databaseOptions = ref<{ text: ProteinDatabase; value: ProteinDatabase }[]>([
  { text: 'UniProt Human(Canonical)', value: 'UniProt Human(Canonical)' },
  { text: 'UniProt Human(Canonical & Isoforms)', value: 'UniProt Human(Canonical & Isoforms)' },
  { text: 'RefSeq Human', value: 'RefSeq Human' },
  { text: 'Swiss-Prot', value: 'Swiss-Prot' },
])
const normalizationOptions = ref<{ text: ProteinNormalization; value: ProteinNormalization }[]>([
  { text: 'Median Normalization', value: 'Median Normalization' },
  { text: 'Quantile Normalization', value: 'Quantile Normalization' },
  { text: 'TIC Normalization', value: 'TIC Normalization' },
  { text: 'Loess Normalization', value: 'Loess Normalization' },
  { text: 'Total MS1 Intensity', value: 'Total MS1 Intensity' },
  { text: 'None', value: 'None' },
])
</script>

<template>
  <VaForm
    v-slot="{ isValid }"
    ref="add-protein-form"
    class="flex-col justify-start items-start gap-4 inline-flex w-full"
  >
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newItem.name"
          :label="t('protein.name')"
          class="w-full sm:w-1/2"
          :rules="[validators.required]"
        />
        <VaSelect
          v-model="newItem.ms_instrument"
          :label="t('protein.ms_instrument')"
          class="w-full sm:w-1/2"
          :options="msInstrumentOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, msInstrumentOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.acquisition_method"
          :label="t('protein.acquisition_method')"
          class="w-full sm:w-1/2"
          :options="acquisitionMethodOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, acquisitionMethodOptions)"
        />
        <VaSelect
          v-model="newItem.lc_system"
          :label="t('protein.lc_system')"
          class="w-full sm:w-1/2"
          :options="lcSystemOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, lcSystemOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.lc_column"
          :label="t('protein.lc_column')"
          class="w-full sm:w-1/2"
          :options="lcColumnOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, lcColumnOptions)"
        />
        <VaSelect
          v-model="newItem.fractionation"
          :label="t('protein.fractionation')"
          class="w-full sm:w-1/2"
          :options="fractionationOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, fractionationOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.digestion_enzyme"
          :label="t('protein.digestion_enzyme')"
          class="w-full sm:w-1/2"
          :options="digestionEnzymeOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, digestionEnzymeOptions)"
        />
        <VaSelect
          v-model="newItem.analysis_software"
          :label="t('protein.analysis_software')"
          class="w-full sm:w-1/2"
          :options="analysisSoftwareOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, analysisSoftwareOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.database"
          :label="t('protein.database')"
          class="w-full sm:w-1/2"
          :options="databaseOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, databaseOptions)"
        />
        <VaSelect
          v-model="newItem.normalization"
          :label="t('protein.normalization')"
          class="w-full sm:w-1/2"
          :options="normalizationOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, normalizationOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="w-full sm:w-1/2">
          <VaDateInput
            v-model="newItem.determinated_at"
            :label="t('protein.determinated_at')"
            class="w-full"
            :format-date="formatDate"
            :rules="[validators.required]"
          />
        </div>
        <div class="w-full sm:w-1/2"></div>
      </div>

      <div v-if="!item" class="flex gap-4 w-full">
        <VaFileUpload
          v-model="metaFile"
          type="single"
          hide-file-list
          class="self-stretch justify-start items-center gap-4 inline-flex"
          file-types="csv"
        >
          <VaButton preset="primary" size="small">{{ t('protein.meta_file') }}</VaButton>
          <div v-if="metaFile" class="va-title">
            {{ `${metaFile.name} ${humanFileSize(metaFile.size)}` }}
          </div>
          <VaButton
            v-if="metaFile"
            preset="primary"
            color="danger"
            size="small"
            icon="delete"
            class="z-10"
            @click.stop="metaFile = undefined"
          />
        </VaFileUpload>
      </div>

      <div v-if="!item" class="flex gap-4 w-full">
        <VaFileUpload
          v-model="expFile"
          type="single"
          hide-file-list
          class="self-stretch justify-start items-center gap-4 inline-flex"
          file-types="csv"
        >
          <VaButton preset="primary" size="small">{{ t('protein.exp_file') }}</VaButton>
          <div v-if="expFile" class="va-title">
            {{ `${expFile.name} ${humanFileSize(expFile.size)}` }}
          </div>
          <VaButton
            v-if="expFile"
            preset="primary"
            color="danger"
            size="small"
            icon="delete"
            class="z-10"
            @click.stop="expFile = undefined"
          />
        </VaFileUpload>
      </div>

      <div v-if="!item" class="flex gap-4 w-full">
        <p class="va-input-label">
          {{ t('omics.csv_only') }},
          <RouterLink
            style="text-decoration: underline"
            target="_blank"
            :to="{ name: 'faq', hash: '#quartet-protein' }"
          >
            {{ t('omics.refer_sample') }}
          </RouterLink>
        </p>
      </div>

      <div v-if="!item && !!errorMsg" class="flex gap-4 w-full">
        <VaAlert dense closeable color="danger" class="w-full">
          {{ errorMsg }}
        </VaAlert>
      </div>

      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton preset="secondary" color="secondary" @click="$emit('close')">{{ t('omics.cancel') }}</VaButton>
        <VaButton
          :disabled="!isValid || (!item && (!metaFile?.name || !expFile?.name))"
          :loading="qcProcessing"
          @click="onSave"
          >{{ saveButtonLabel }}</VaButton
        >
      </div>
    </div>
  </VaForm>
</template>
