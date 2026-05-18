<script setup lang="ts">
import Papa from 'papaparse'
import { PropType, computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm } from 'vuestic-ui'
import {
  MetabolismItem,
  MetabolismMsInstrument,
  MetabolismDetectionStrategy,
  MetabolismAnalysisPlatform,
  MetabolismChromatography,
  MetabolismIonization,
  MetabolismExtractionMethod,
  MetabolismPreprocessingSoftware,
  MetabolismIdentificationLevel,
  MetabolismUnit,
  AliOSSToken,
} from '../types'
import { validators, humanFileSize } from '../../../services/utils'
import { qcQuartetMetabolism } from '../../../services/webr'
import { addNewOption, formatDate, makeAliOssClient, renameFile } from '../composables/useOmics'

const { t } = useI18n()

const prefix = 'metabolism'

const props = defineProps({
  item: {
    type: Object as PropType<MetabolismItem | null>,
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

type editMetabolismItem = Omit<MetabolismItem, 'id' | 'user_id' | 'report_file' | 'notes'>

const defaultNewItem: editMetabolismItem = {
  name: '',
  ms_instrument: null,
  detection_strategy: null,
  analysis_platform: null,
  chromatography: null,
  ionization: null,
  extraction_method: null,
  preprocessing_software: null,
  identification_level: null,
  unit: null,
  database: '',
  determinated_at: new Date(),
  meta_file: '',
  exp_file: '',
  snr: 0,
  pcc: 0,
  d5: 0,
  d6: 0,
  f7: 0,
  m8: 0,
}

const newItem = ref<MetabolismItem>({ ...defaultNewItem } as MetabolismItem)

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
      newItem.value[key as keyof editMetabolismItem] !==
      (props.item ?? defaultNewItem)?.[key as keyof editMetabolismItem]
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

const form = useForm('add-metabolism-form')

const emit = defineEmits(['close', 'save'])

const onSave = async () => {
  errorMsg.value = ''
  if (form.validate()) {
    try {
      qcProcessing.value = true
      if (props.token) {
        const qc_result = await qcQuartetMetabolism(metaFile.value!, expFile.value!)
        if (qc_result[0] === Infinity) {
          throw new Error('SNR is Infinity.')
        }
        newItem.value.snr = qc_result[0]!
        newItem.value.pcc = qc_result[1]!

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

const msInstrumentOptions = ref<{ text: MetabolismMsInstrument; value: MetabolismMsInstrument }[]>([
  { text: 'Thermo Orbitrap Exploris/QE Series', value: 'Thermo Orbitrap Exploris/QE Series' },
  { text: 'Agilent Q-TOF(6500 series)', value: 'Agilent Q-TOF(6500 series)' },
  { text: 'Waters Vion/Synapt G2-Si', value: 'Waters Vion/Synapt G2-Si' },
  { text: 'Bruker impact II/timsTOF', value: 'Bruker impact II/timsTOF' },
  { text: 'Sciex TripleTOF/Triple Quad', value: 'Sciex TripleTOF/Triple Quad' },
  { text: 'AB Sciex QTRAP', value: 'AB Sciex QTRAP' },
])
const detectionStrategyOptions = ref<{ text: string; value: MetabolismDetectionStrategy }[]>([
  { text: t('metabolism.untargeted'), value: 'Untargeted' },
  { text: t('metabolism.targeted'), value: 'Targeted' },
  { text: t('metabolism.semi_targeted'), value: 'Semi-targeted' },
  { text: t('metabolism.lipidomics'), value: 'Lipidomics' },
])
const analysisPlatformOptions = ref<{ text: string; value: MetabolismAnalysisPlatform }[]>([
  { text: t('metabolism.lc_ms'), value: 'LC-MS' },
  { text: t('metabolism.gc_ms'), value: 'GC-MS' },
  { text: t('metabolism.nmr'), value: 'NMR' },
  { text: t('metabolism.ce_ms'), value: 'CE-MS' },
  { text: t('metabolism.di_ms'), value: 'DI-MS' },
])
const chromatographyOptions = ref<{ text: string; value: MetabolismChromatography }[]>([
  { text: t('metabolism.reverse_phase'), value: 'Reverse Phase' },
  { text: t('metabolism.hilic'), value: 'HILIC' },
  { text: t('metabolism.normal_phase'), value: 'Normal Phase' },
  { text: t('metabolism.gc_capillary_column'), value: 'GC-Capillary Column' },
  { text: t('metabolism.ion_exchange'), value: 'Ion Exchange' },
])
const ionizationOptions = ref<{ text: string; value: MetabolismIonization }[]>([
  { text: t('metabolism.esi_positive'), value: 'ESI Positive' },
  { text: t('metabolism.esi_negative'), value: 'ESI Negative' },
  { text: t('metabolism.apci'), value: 'APCI' },
  { text: t('metabolism.ei'), value: 'EI' },
])
const extractionMethodOptions = ref<{ text: MetabolismExtractionMethod; value: MetabolismExtractionMethod }[]>([
  { text: 'Methanol', value: 'Methanol' },
  { text: 'Acetonitrile', value: 'Acetonitrile' },
  { text: 'Methanol/Water', value: 'Methanol/Water' },
  { text: 'Chloroform/Methanol(Bligh-Dyer)', value: 'Chloroform/Methanol(Bligh-Dyer)' },
  { text: 'Protein Precipitation', value: 'Protein Precipitation' },
])
const preprocessingSoftwareOptions = ref<
  {
    text: MetabolismPreprocessingSoftware
    value: MetabolismPreprocessingSoftware
  }[]
>([
  { text: 'Compound Discoverer', value: 'Compound Discoverer' },
  { text: 'MS-DIAL', value: 'MS-DIAL' },
  { text: 'XCMS', value: 'XCMS' },
  { text: 'MZmine', value: 'MZmine' },
  { text: 'MetaboAnalyst', value: 'MetaboAnalyst' },
  { text: 'TraceFinder', value: 'TraceFinder' },
  { text: 'MassHunter', value: 'MassHunter' },
  { text: 'LabSolutions', value: 'LabSolutions' },
])
const identificationLevelOptions = ref<{ text: MetabolismIdentificationLevel; value: MetabolismIdentificationLevel }[]>(
  [
    { text: 'MS1 & MS2(Secondary standard)', value: 'MS1 & MS2(Secondary standard)' },
    { text: 'MS1 & RT(Accurate mass)', value: 'MS1 & RT(Accurate mass)' },
    { text: 'MS2 Library Match', value: 'MS2 Library Match' },
    { text: 'IDMS', value: 'IDMS' },
  ],
)
const unitOptions = ref<{ text: MetabolismUnit; value: MetabolismUnit }[]>([
  { text: 'Peak Intensity', value: 'Peak Intensity' },
  { text: 'Peak Area', value: 'Peak Area' },
  { text: 'Concentration(μmol/L or mg/dL)', value: 'Concentration(μmol/L or mg/dL)' },
  { text: 'Internal Standard Normalized', value: 'Internal Standard Normalized' },
  { text: 'TIC Normalized', value: 'TIC Normalized' },
  { text: 'PQN Normalized', value: 'PQN Normalized' },
])
</script>

<template>
  <VaForm
    v-slot="{ isValid }"
    ref="add-metabolism-form"
    class="flex-col justify-start items-start gap-4 inline-flex w-full"
  >
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newItem.name"
          :label="t('metabolism.name')"
          class="w-full sm:w-1/2"
          :rules="[validators.required]"
        />
        <VaSelect
          v-model="newItem.ms_instrument"
          :label="t('metabolism.ms_instrument')"
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
          v-model="newItem.detection_strategy"
          :label="t('metabolism.detection_strategy')"
          class="w-full sm:w-1/2"
          :options="detectionStrategyOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, detectionStrategyOptions)"
        />
        <VaSelect
          v-model="newItem.analysis_platform"
          :label="t('metabolism.analysis_platform')"
          class="w-full sm:w-1/2"
          :options="analysisPlatformOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, analysisPlatformOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.chromatography"
          :label="t('metabolism.chromatography')"
          class="w-full sm:w-1/2"
          :options="chromatographyOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, chromatographyOptions)"
        />
        <VaSelect
          v-model="newItem.ionization"
          :label="t('metabolism.ionization')"
          class="w-full sm:w-1/2"
          :options="ionizationOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, ionizationOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.extraction_method"
          :label="t('metabolism.extraction_method')"
          class="w-full sm:w-1/2"
          :options="extractionMethodOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, extractionMethodOptions)"
        />
        <VaSelect
          v-model="newItem.preprocessing_software"
          :label="t('metabolism.preprocessing_software')"
          class="w-full sm:w-1/2"
          :options="preprocessingSoftwareOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, preprocessingSoftwareOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.identification_level"
          :label="t('metabolism.identification_level')"
          class="w-full sm:w-1/2"
          :options="identificationLevelOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, identificationLevelOptions)"
        />
        <VaSelect
          v-model="newItem.unit"
          :label="t('metabolism.unit')"
          class="w-full sm:w-1/2"
          :options="unitOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, unitOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newItem.database"
          :label="t('metabolism.database')"
          class="w-full sm:w-1/2"
          :placeholder="t('metabolism.name_version')"
          :rules="[validators.required]"
        />
        <div class="w-full sm:w-1/2">
          <VaDateInput
            v-model="newItem.determinated_at"
            :label="t('metabolism.determinated_at')"
            class="w-full"
            :format-date="formatDate"
            :rules="[validators.required]"
          />
        </div>
      </div>

      <div v-if="!item" class="flex gap-4 w-full">
        <VaFileUpload
          v-model="metaFile"
          type="single"
          hide-file-list
          class="self-stretch justify-start items-center gap-4 inline-flex"
          file-types="csv"
        >
          <VaButton preset="primary" size="small">{{ t('metabolism.meta_file') }}</VaButton>
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
          <VaButton preset="primary" size="small">{{ t('metabolism.exp_file') }}</VaButton>
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
            :to="{ name: 'faq', hash: '#quartet-metabolism' }"
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
