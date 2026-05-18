<script setup lang="ts">
import Papa from 'papaparse'
import { PropType, computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm } from 'vuestic-ui'
import {
  PlasmixProteinItem,
  PlasmixProteinTechnology,
  PlasmixProteinModelPanel,
  PlasmixProteinPreTreatment,
  PlasmixProteinReadout,
  PlasmixProteinProtocol,
  PlasmixProteinExpressionUnit,
  PlasmixProteinAnalysisSoftware,
  PlasmixProteinDatabase,
  PlasmixProteinNormalization,
  AliOSSToken,
} from '../types'
import { validators, humanFileSize } from '../../../services/utils'
import { qcPlasmixProtein } from '../../../services/webr'
import { addNewOption, formatDate, makeAliOssClient, renameFile } from '../composables/useOmics'

const { t } = useI18n()

const prefix = 'plasmix_protein'

const props = defineProps({
  item: {
    type: Object as PropType<PlasmixProteinItem | null>,
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

type editPlasmixProteinItem = Omit<PlasmixProteinItem, 'id' | 'user_id' | 'report_file' | 'notes'>

const defaultNewItem: editPlasmixProteinItem = {
  name: '',
  technology: null,
  model_panel: null,
  pre_treatment: null,
  readout: null,
  protocol: null,
  expression_unit: null,
  analysis_software: null,
  database: null,
  normalization: null,
  determinated_at: new Date(),
  meta_file: '',
  exp_file: '',
  identified_proteins: 0,
  recall: 0,
  snr: 0,
  rc: 0,
  m: 0,
  f: 0,
  p: 0,
  x: 0,
  y: 0,
}

const newItem = ref<PlasmixProteinItem>({ ...defaultNewItem } as PlasmixProteinItem)

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
      newItem.value[key as keyof editPlasmixProteinItem] !==
      (props.item ?? defaultNewItem)?.[key as keyof editPlasmixProteinItem]
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

const form = useForm('add-plasmix-protein-form')

const emit = defineEmits(['close', 'save'])

const onSave = async () => {
  errorMsg.value = ''
  if (form.validate()) {
    try {
      qcProcessing.value = true
      if (props.token) {
        const qc_result = await qcPlasmixProtein(metaFile.value!, expFile.value!)
        if (qc_result[2] === Infinity) {
          throw new Error('SNR is Infinity.')
        }
        newItem.value.identified_proteins = qc_result[0]!
        newItem.value.recall = qc_result[1]!
        newItem.value.snr = qc_result[2]!
        newItem.value.rc = qc_result[3]!

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

const technologyOptions = ref<{ text: PlasmixProteinTechnology; value: PlasmixProteinTechnology }[]>([
  { text: 'Olink', value: 'Olink' },
  { text: 'SomaScan', value: 'SomaScan' },
  { text: 'DIA', value: 'DIA' },
  { text: 'Mass Spectrometry', value: 'Mass Spectrometry' },
  { text: 'Luminex', value: 'Luminex' },
  { text: 'ELISA', value: 'ELISA' },
  { text: 'Simoa', value: 'Simoa' },
])
const modelPanelOptions = ref<{ text: PlasmixProteinModelPanel; value: PlasmixProteinModelPanel }[]>([
  { text: 'Thermo Orbitrap Astral', value: 'Thermo Orbitrap Astral' },
  { text: 'Thermo Orbitrap Exploris 480/240', value: 'Thermo Orbitrap Exploris 480/240' },
  { text: 'Thermo Q Exactive HF-X/HF', value: 'Thermo Q Exactive HF-X/HF' },
  { text: 'Bruker timsTOF Pro/Pro 2', value: 'Bruker timsTOF Pro/Pro 2' },
  { text: 'Bruker timsTOF HT', value: 'Bruker timsTOF HT' },
  { text: 'SCIEX ZenoTOF 7600', value: 'SCIEX ZenoTOF 7600' },
  { text: 'SCIEX TripleTOF 6600+', value: 'SCIEX TripleTOF 6600+' },
  { text: 'Olink Explore HT', value: 'Olink Explore HT' },
  { text: 'Olink Explore 3072/1536', value: 'Olink Explore 3072/1536' },
  { text: 'Olink Target 96/48', value: 'Olink Target 96/48' },
  { text: 'SomaScan 7K/11K', value: 'SomaScan 7K/11K' },
  { text: 'Luminex 200/FLEXMAP 3D', value: 'Luminex 200/FLEXMAP 3D' },
  { text: 'Simoa HD-X/HD-1', value: 'Simoa HD-X/HD-1' },
  { text: 'Simoa SR-X', value: 'Simoa SR-X' },
  { text: 'Simoa SP-X', value: 'Simoa SP-X' },
])
const preTreatmentOptions = ref<{ text: string; value: PlasmixProteinPreTreatment }[]>([
  { text: t('plasmix_protein.neat_plasma'), value: 'Neat Plasma' },
  { text: t('plasmix_protein.high_abundance_depletion'), value: 'High-abundance Depletion' },
  { text: t('plasmix_protein.nanoparticle_enrichment'), value: 'Nanoparticle Enrichment' },
  { text: t('plasmix_protein.proximity_extension'), value: 'Proximity Extension' },
  { text: t('plasmix_protein.aptamer_capture'), value: 'Aptamer Capture' },
])
const readoutOptions = ref<{ text: PlasmixProteinReadout; value: PlasmixProteinReadout }[]>([
  { text: 'Mass Spectrometer', value: 'Mass Spectrometer' },
  { text: 'Illumina NGS(Olink Explore)', value: 'Illumina NGS(Olink Explore)' },
  { text: 'qPCR(Olink Target)', value: 'qPCR(Olink Target)' },
  { text: 'DNA Microarray(SomaScan)', value: 'DNA Microarray(SomaScan)' },
  { text: 'Digital Image/AEB', value: 'Digital Image/AEB' },
])
const protocolOptions = ref<{ text: PlasmixProteinProtocol; value: PlasmixProteinProtocol }[]>([
  { text: 'In-solution Trypsin', value: 'In-solution Trypsin' },
  { text: 'S-Trap/iST', value: 'S-Trap/iST' },
  { text: 'PEA Extension & Ligation', value: 'PEA Extension & Ligation' },
  { text: 'Biotin-Aptamer Binding', value: 'Biotin-Aptamer Binding' },
  { text: 'Digital ELISA', value: 'Digital ELISA' },
])
const expressionUnitOptions = ref<{ text: PlasmixProteinExpressionUnit; value: PlasmixProteinExpressionUnit }[]>([
  { text: 'NPX', value: 'NPX' },
  { text: 'RFU', value: 'RFU' },
  { text: 'Log2 Intensity', value: 'Log2 Intensity' },
  { text: 'LFQ Intensity', value: 'LFQ Intensity' },
  { text: 'Concentration(pg/mL)', value: 'Concentration(pg/mL)' },
  { text: 'AEB(Average Enzymes per Bead)', value: 'AEB(Average Enzymes per Bead)' },
])
const analysisSoftwareOptions = ref<{ text: PlasmixProteinAnalysisSoftware; value: PlasmixProteinAnalysisSoftware }[]>([
  { text: 'Olink NPX Signature/Manager', value: 'Olink NPX Signature/Manager' },
  { text: 'DIA-NN', value: 'DIA-NN' },
  { text: 'Spectronaut', value: 'Spectronaut' },
  { text: 'MaxQuant', value: 'MaxQuant' },
  { text: 'FragPipe', value: 'FragPipe' },
  { text: 'SomaLogic Analysis Software', value: 'SomaLogic Analysis Software' },
  { text: 'Skyline', value: 'Skyline' },
  { text: 'Simoa Software(Quanterix)', value: 'Simoa Software(Quanterix)' },
])
const databaseOptions = ref<{ text: PlasmixProteinDatabase; value: PlasmixProteinDatabase }[]>([
  { text: 'UniProt Human (Canonical)', value: 'UniProt Human (Canonical)' },
  { text: 'Olink Internal Annotation', value: 'Olink Internal Annotation' },
  { text: 'Simoa Assay List', value: 'Simoa Assay List' },
  { text: 'SomaScan Target List', value: 'SomaScan Target List' },
  { text: 'RefSeq Human', value: 'RefSeq Human' },
])
const normalizationOptions = ref<{ text: PlasmixProteinNormalization; value: PlasmixProteinNormalization }[]>([
  { text: 'IPC', value: 'IPC' },
  { text: 'Olink Bridge Normalization', value: 'Olink Bridge Normalization' },
  { text: 'Median Normalization', value: 'Median Normalization' },
  { text: 'TIC Normalization', value: 'TIC Normalization' },
  { text: 'ANML(SomaScan)', value: 'ANML(SomaScan)' },
  { text: 'Plate-specific Normalization', value: 'Plate-specific Normalization' },
  { text: 'Calibrator Curve', value: 'Calibrator Curve' },
])
</script>

<template>
  <VaForm
    v-slot="{ isValid }"
    ref="add-plasmix-protein-form"
    class="flex-col justify-start items-start gap-4 inline-flex w-full"
  >
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newItem.name"
          :label="t('plasmix_protein.name')"
          class="w-full sm:w-1/2"
          :rules="[validators.required]"
        />
        <VaSelect
          v-model="newItem.technology"
          :label="t('plasmix_protein.technology')"
          class="w-full sm:w-1/2"
          :options="technologyOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, technologyOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.model_panel"
          :label="t('plasmix_protein.model_panel')"
          class="w-full sm:w-1/2"
          :options="modelPanelOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, modelPanelOptions)"
        />
        <VaSelect
          v-model="newItem.pre_treatment"
          :label="t('plasmix_protein.pre_treatment')"
          class="w-full sm:w-1/2"
          :options="preTreatmentOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, preTreatmentOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.readout"
          :label="t('plasmix_protein.readout')"
          class="w-full sm:w-1/2"
          :options="readoutOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, readoutOptions)"
        />
        <VaSelect
          v-model="newItem.protocol"
          :label="t('plasmix_protein.protocol')"
          class="w-full sm:w-1/2"
          :options="protocolOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, protocolOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.expression_unit"
          :label="t('plasmix_protein.expression_unit')"
          class="w-full sm:w-1/2"
          :options="expressionUnitOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, expressionUnitOptions)"
        />
        <VaSelect
          v-model="newItem.analysis_software"
          :label="t('plasmix_protein.analysis_software')"
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
          :label="t('plasmix_protein.database')"
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
          :label="t('plasmix_protein.normalization')"
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
            :label="t('plasmix_protein.determinated_at')"
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
          <VaButton preset="primary" size="small">{{ t('plasmix_protein.meta_file') }}</VaButton>
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
          <VaButton preset="primary" size="small">{{ t('plasmix_protein.exp_file') }}</VaButton>
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
            :to="{ name: 'faq', hash: '#plasmix-protein' }"
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
