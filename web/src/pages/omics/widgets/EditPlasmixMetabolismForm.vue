<script setup lang="ts">
import Papa from 'papaparse'
import { PropType, computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm } from 'vuestic-ui'
import {
  PlasmixMetabolismItem,
  PlasmixMetabolismMsInstrument,
  PlasmixMetabolismDetectionStrategy,
  PlasmixMetabolismAnalysisPlatform,
  PlasmixMetabolismChromatography,
  PlasmixMetabolismIonization,
  PlasmixMetabolismExtractionMethod,
  PlasmixMetabolismPreprocessingSoftware,
  PlasmixMetabolismNormalization,
  PlasmixMetabolismIdentificationBasis,
  AliOSSToken,
} from '../types'
import { validators, humanFileSize } from '../../../services/utils'
import { qcPlasmixMetabolism } from '../../../services/webr'
import { addNewOption, formatDate, makeAliOssClient, renameFile } from '../composables/useOmics'

const { t } = useI18n()

const prefix = 'plasmix_metabolism'

const props = defineProps({
  item: {
    type: Object as PropType<PlasmixMetabolismItem | null>,
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

type editPlasmixMetabolismItem = Omit<PlasmixMetabolismItem, 'id' | 'user_id' | 'report_file' | 'notes'>

const defaultNewItem: editPlasmixMetabolismItem = {
  name: '',
  ms_instrument: null,
  detection_strategy: null,
  analysis_platform: null,
  chromatography: null,
  ionization: null,
  extraction_method: null,
  preprocessing_software: null,
  normalization: null,
  identification_basis: null,
  database: '',
  determinated_at: new Date(),
  meta_file: '',
  exp_file: '',
  snr: 0,
  pcc: 0,
  m: 0,
  f: 0,
  p: 0,
  x: 0,
  y: 0,
}

const newItem = ref<PlasmixMetabolismItem>({ ...defaultNewItem } as PlasmixMetabolismItem)

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
      newItem.value[key as keyof editPlasmixMetabolismItem] !==
      (props.item ?? defaultNewItem)?.[key as keyof editPlasmixMetabolismItem]
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

const form = useForm('add-plasmix-metabolism-form')

const emit = defineEmits(['close', 'save'])

const onSave = async () => {
  errorMsg.value = ''
  if (form.validate()) {
    try {
      qcProcessing.value = true
      if (props.token) {
        const qc_result = await qcPlasmixMetabolism(metaFile.value!, expFile.value!)
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
        const m = metaData.filter((row: any) => !!row.sample && row.sample.trim().toLowerCase() === 'm').length
        const f = metaData.filter((row: any) => !!row.sample && row.sample.trim().toLowerCase() === 'f').length
        const p = metaData.filter((row: any) => !!row.sample && row.sample.trim().toLowerCase() === 'p').length
        const x = metaData.filter((row: any) => !!row.sample && row.sample.trim().toLowerCase() === 'x').length
        const y = metaData.filter((row: any) => !!row.sample && row.sample.trim().toLowerCase() === 'y').length
        newItem.value.m = m
        newItem.value.f = f
        newItem.value.p = p
        newItem.value.x = x
        newItem.value.y = y

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

const msInstrumentOptions = ref<{ text: PlasmixMetabolismMsInstrument; value: PlasmixMetabolismMsInstrument }[]>([
  { text: 'Thermo Orbitrap Exploris/QE Series', value: 'Thermo Orbitrap Exploris/QE Series' },
  { text: 'Agilent Q-TOF(6500 series)', value: 'Agilent Q-TOF(6500 series)' },
  { text: 'Waters Vion/Synapt G2-Si', value: 'Waters Vion/Synapt G2-Si' },
  { text: 'Bruker impact II/timsTOF', value: 'Bruker impact II/timsTOF' },
  { text: 'Sciex TripleTOF/Triple Quad', value: 'Sciex TripleTOF/Triple Quad' },
  { text: 'AB Sciex QTRAP', value: 'AB Sciex QTRAP' },
])
const detectionStrategyOptions = ref<{ text: string; value: PlasmixMetabolismDetectionStrategy }[]>([
  { text: t('plasmix_metabolism.untargeted'), value: 'Untargeted' },
  { text: t('plasmix_metabolism.targeted'), value: 'Targeted' },
  { text: t('plasmix_metabolism.polar_metabolites'), value: 'Polar Metabolites' },
  { text: t('plasmix_metabolism.lipidomics'), value: 'Lipidomics' },
])
const analysisPlatformOptions = ref<{ text: string; value: PlasmixMetabolismAnalysisPlatform }[]>([
  { text: t('plasmix_metabolism.lc_ms'), value: 'LC-MS' },
  { text: t('plasmix_metabolism.gc_ms'), value: 'GC-MS' },
  { text: t('plasmix_metabolism.nmr'), value: 'NMR' },
  { text: t('plasmix_metabolism.di_ms'), value: 'DI-MS' },
])
const chromatographyOptions = ref<{ text: string; value: PlasmixMetabolismChromatography }[]>([
  { text: t('plasmix_metabolism.reverse_phase'), value: 'Reverse Phase' },
  { text: t('plasmix_metabolism.hilic'), value: 'HILIC' },
  { text: t('plasmix_metabolism.normal_phase'), value: 'Normal Phase' },
  { text: t('plasmix_metabolism.gc_capillary_column'), value: 'GC-Capillary Column' },
  { text: t('plasmix_metabolism.biphasic'), value: 'Biphasic/Mixed-mode' },
])
const ionizationOptions = ref<{ text: string; value: PlasmixMetabolismIonization }[]>([
  { text: t('plasmix_metabolism.esi_positive'), value: 'ESI Positive' },
  { text: t('plasmix_metabolism.esi_negative'), value: 'ESI Negative' },
  { text: t('plasmix_metabolism.dual_polarity'), value: 'Dual Polarity' },
  { text: t('plasmix_metabolism.apci'), value: 'APCI' },
  { text: t('plasmix_metabolism.ei'), value: 'EI' },
])
const extractionMethodOptions = ref<
  { text: PlasmixMetabolismExtractionMethod; value: PlasmixMetabolismExtractionMethod }[]
>([
  { text: 'Methanol', value: 'Methanol' },
  { text: 'Acetonitrile', value: 'Acetonitrile' },
  { text: 'Methanol:Acetonitrile', value: 'Methanol:Acetonitrile' },
  { text: 'Chloroform:Methanol', value: 'Chloroform:Methanol' },
  { text: 'SPE', value: 'SPE' },
  { text: 'None', value: 'None' },
])
const preprocessingSoftwareOptions = ref<
  {
    text: PlasmixMetabolismPreprocessingSoftware
    value: PlasmixMetabolismPreprocessingSoftware
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
const normalizationOptions = ref<{ text: PlasmixMetabolismNormalization; value: PlasmixMetabolismNormalization }[]>([
  { text: 'Internal Standard', value: 'Internal Standard' },
  { text: 'QC-RLSC', value: 'QC-RLSC' },
  { text: 'TIC Normalization', value: 'TIC Normalization' },
  { text: 'Median Normalization', value: 'Median Normalization' },
  { text: 'PQN Normalization', value: 'PQN Normalization' },
  { text: 'None', value: 'None' },
])
const identificationBasisOptions = ref<
  { text: PlasmixMetabolismIdentificationBasis; value: PlasmixMetabolismIdentificationBasis }[]
>([
  { text: 'MS1 & MS2 & RT', value: 'MS1 & MS2 & RT' },
  { text: 'MS1 & MS2', value: 'MS1 & MS2' },
  { text: 'Accurate Mass', value: 'Accurate Mass' },
  { text: 'NMR Chemical Shift', value: 'NMR Chemical Shift' },
])
</script>

<template>
  <VaForm
    v-slot="{ isValid }"
    ref="add-plasmix-metabolism-form"
    class="flex-col justify-start items-start gap-4 inline-flex w-full"
  >
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newItem.name"
          :label="t('plasmix_metabolism.name')"
          class="w-full sm:w-1/2"
          :rules="[validators.required]"
        />
        <VaSelect
          v-model="newItem.ms_instrument"
          :label="t('plasmix_metabolism.ms_instrument')"
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
          :label="t('plasmix_metabolism.detection_strategy')"
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
          :label="t('plasmix_metabolism.analysis_platform')"
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
          :label="t('plasmix_metabolism.chromatography')"
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
          :label="t('plasmix_metabolism.ionization')"
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
          :label="t('plasmix_metabolism.extraction_method')"
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
          :label="t('plasmix_metabolism.preprocessing_software')"
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
          v-model="newItem.normalization"
          :label="t('plasmix_metabolism.normalization')"
          class="w-full sm:w-1/2"
          :options="normalizationOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, normalizationOptions)"
        />
        <VaSelect
          v-model="newItem.identification_basis"
          :label="t('plasmix_metabolism.identification_basis')"
          class="w-full sm:w-1/2"
          :options="identificationBasisOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, identificationBasisOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newItem.database"
          :label="t('plasmix_metabolism.database')"
          class="w-full sm:w-1/2"
          :placeholder="t('plasmix_metabolism.name_version')"
          :rules="[validators.required]"
        />
        <div class="w-full sm:w-1/2">
          <VaDateInput
            v-model="newItem.determinated_at"
            :label="t('plasmix_metabolism.determinated_at')"
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
          <VaButton preset="primary" size="small">{{ t('plasmix_metabolism.meta_file') }}</VaButton>
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
          <VaButton preset="primary" size="small">{{ t('plasmix_metabolism.exp_file') }}</VaButton>
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
            :to="{ name: 'faq', hash: '#plasmix-metabolism' }"
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
