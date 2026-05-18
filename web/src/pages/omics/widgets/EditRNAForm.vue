<script setup lang="ts">
import Papa from 'papaparse'
import { PropType, computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm } from 'vuestic-ui'
import {
  RNAItem,
  RNAPlatformModel,
  RNAFragmentation,
  RNAEnrichment,
  RNALibraryProtocol,
  RNAReadMode,
  RNAReference,
  RNAAlignmentSoftware,
  RNAQuantificationTool,
  RNAExpressionUnit,
  AliOSSToken,
} from '../types'
import { validators, humanFileSize } from '../../../services/utils'
import { qcQuartetRNA } from '../../../services/webr'
import { addNewOption, formatDate, makeAliOssClient, renameFile } from '../composables/useOmics'

const { t } = useI18n()

const prefix = 'rna'

const props = defineProps({
  item: {
    type: Object as PropType<RNAItem | null>,
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

type editRNAItem = Omit<RNAItem, 'id' | 'user_id' | 'report_file' | 'notes'>

const defaultNewItem: editRNAItem = {
  name: '',
  platform_model: null,
  rna_fragmentation: null,
  enrichment: null,
  library_protocol: null,
  read_mode: null,
  reference: null,
  alignment_software: null,
  quantification_tool: null,
  expression_unit: null,
  determinated_at: new Date(),
  meta_file: '',
  exp_file: '',
  count_file: '',
  snr: 0,
  pcc: 0,
  d5: 0,
  d6: 0,
  f7: 0,
  m8: 0,
}

const newItem = ref<RNAItem>({ ...defaultNewItem } as RNAItem)

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
    return newItem.value[key as keyof editRNAItem] !== (props.item ?? defaultNewItem)?.[key as keyof editRNAItem]
  })
})

defineExpose({
  isFormHasUnsavedChanges,
})

const metaFile = ref<File>()
const expFile = ref<File>()
const countFile = ref<File>()
const qcProcessing = ref(false)
const errorMsg = ref('')

const form = useForm('add-rna-form')

const emit = defineEmits(['close', 'save'])

const onSave = async () => {
  errorMsg.value = ''
  if (form.validate()) {
    try {
      qcProcessing.value = true
      if (props.token) {
        const qc_result = await qcQuartetRNA(metaFile.value!, expFile.value!, countFile.value!)
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

        result = await oSSClient.put(renameFile(prefix, countFile.value!.name), countFile.value!)
        console.log(`${countFile.value!.name} uploaded to ${result.url}`)
        newItem.value.count_file = result.name
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
const platformSelectOptions = ref<{ text: RNAPlatformModel; value: RNAPlatformModel }[]>([
  { text: 'Illumina NovaSeq 6000/X Series', value: 'Illumina NovaSeq 6000/X Series' },
  { text: 'Illumina HiSeq X Ten/2500', value: 'Illumina HiSeq X Ten/2500' },
  { text: 'Illumina NextSeq/MiSeq', value: 'Illumina NextSeq/MiSeq' },
  { text: 'MGI DNBSEQ-T7 series', value: 'MGI DNBSEQ-T7 series' },
  { text: 'MGI DNBSEQ T10/T20', value: 'MGI DNBSEQ T10/T20' },
  { text: 'MGI DNBSEQ-G400 (SEQ2000)', value: 'MGI DNBSEQ-G400 (SEQ2000)' },
  { text: 'PacBio Sequel II/IIe', value: 'PacBio Sequel II/IIe' },
  { text: 'PacBio Revio', value: 'PacBio Revio' },
  { text: 'ONT PromethION', value: 'ONT PromethION' },
  { text: 'ONT GridION', value: 'ONT GridION' },
  { text: 'ONT MinION', value: 'ONT MinION' },
  { text: 'Element AVITI', value: 'Element AVITI' },
  { text: 'Ultima UG100', value: 'Ultima UG100' },
])
const fragmentationSelectOptions = ref<{ text: string; value: RNAFragmentation }[]>([
  { text: t('rna.acoustic_shear'), value: 'Acoustic Shear' },
  { text: t('rna.enzymatic'), value: 'Enzymatic' },
  { text: t('rna.chemical'), value: 'Chemical/Heat' },
  { text: t('rna.no_fragmentation'), value: 'No Fragmentation' },
])
const enrichmentSelectOptions = ref<{ text: string; value: RNAEnrichment }[]>([
  { text: t('rna.ribosomal_depletion'), value: 'Ribosomal RNA Depletion' },
  { text: t('rna.polyA_selection'), value: 'Poly-A Selection' },
  { text: t('rna.total_rna'), value: 'Total RNA' },
  { text: t('rna.globin_depletion'), value: 'Globin Depletion' },
  { text: t('rna.targeted_capture'), value: 'Targeted Capture' },
])
const libraryProtocolSelectOptions = ref<{ text: RNALibraryProtocol; value: RNALibraryProtocol }[]>([
  { text: 'Stranded (Antisense/Reverse) 3', value: 'Stranded (Antisense/Reverse) 3' },
  { text: 'Stranded (Sense/Forward)', value: 'Stranded (Sense/Forward)' },
  { text: 'Non-stranded 4', value: 'Non-stranded 4' },
  { text: 'SMART-Seq', value: 'SMART-Seq' },
  { text: 'Direct RNA (ONT)', value: 'Direct RNA (ONT)' },
  { text: 'cDNA (Full-length)', value: 'cDNA (Full-length)' },
])
const readModeSelectOptions = ref<{ text: RNAReadMode; value: RNAReadMode }[]>([
  { text: 'PE 150bp', value: 'PE 150bp' },
  { text: 'PE 100bp', value: 'PE 100bp' },
  { text: 'PE 75bp', value: 'PE 75bp' },
  { text: 'SE 100bp', value: 'SE 100bp' },
  { text: 'SE 75bp', value: 'SE 75bp' },
  { text: 'SE 50bp', value: 'SE 50bp' },
  { text: 'Long-reads(Iso-Seq)', value: 'Long-reads(Iso-Seq)' },
  { text: 'ONT Raw Reads', value: 'ONT Raw Reads' },
])
const referenceSelectOptions = ref<{ text: RNAReference; value: RNAReference }[]>([
  { text: 'GRCh38+Ensembl', value: 'GRCh38+Ensembl' },
  { text: 'GRCh38+GENCODE', value: 'GRCh38+GENCODE' },
  { text: 'GRCh37(hg19)', value: 'GRCh37(hg19)' },
  { text: 'T2T-CHM13', value: 'T2T-CHM13' },
])
const alignmentSoftwareSelectOptions = ref<{ text: RNAAlignmentSoftware; value: RNAAlignmentSoftware }[]>([
  { text: 'STAR', value: 'STAR' },
  { text: 'HISAT2', value: 'HISAT2' },
  { text: 'Bowtie2', value: 'Bowtie2' },
  { text: 'TopHat2', value: 'TopHat2' },
  { text: 'Salmon(Mapping-based)', value: 'Salmon(Mapping-based)' },
  { text: 'Kallisto(Pseudo-alignment)', value: 'Kallisto(Pseudo-alignment)' },
  { text: 'minimap2(Long-reads)', value: 'minimap2(Long-reads)' },
])
const quantificationToolSelectOptions = ref<{ text: RNAQuantificationTool; value: RNAQuantificationTool }[]>([
  { text: 'FeatureCounts', value: 'FeatureCounts' },
  { text: 'HTSeq-count', value: 'HTSeq-count' },
  { text: 'RSEM', value: 'RSEM' },
  { text: 'Salmon (Quasi-mapping)', value: 'Salmon (Quasi-mapping)' },
  { text: 'Kallisto', value: 'Kallisto' },
])
const expressionUnitSelectOptions = ref<{ text: RNAExpressionUnit; value: RNAExpressionUnit }[]>([
  { text: 'TPM', value: 'TPM' },
  { text: 'FPKM', value: 'FPKM' },
  { text: 'RPKM', value: 'RPKM' },
  { text: 'CPM', value: 'CPM' },
  { text: 'Normalized Counts', value: 'Normalized Counts' },
])
</script>

<template>
  <VaForm v-slot="{ isValid }" ref="add-rna-form" class="flex-col justify-start items-start gap-4 inline-flex w-full">
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput v-model="newItem.name" :label="t('rna.name')" class="w-full sm:w-1/2" :rules="[validators.required]" />
        <VaSelect
          v-model="newItem.platform_model"
          :label="t('rna.platform_model')"
          class="w-full sm:w-1/2"
          :options="platformSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, platformSelectOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.rna_fragmentation"
          :label="t('rna.rna_fragmentation')"
          class="w-full sm:w-1/2"
          :options="fragmentationSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, fragmentationSelectOptions)"
        />
        <VaSelect
          v-model="newItem.enrichment"
          :label="t('rna.enrichment')"
          class="w-full sm:w-1/2"
          :options="enrichmentSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, enrichmentSelectOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.library_protocol"
          :label="t('rna.library_protocol')"
          class="w-full sm:w-1/2"
          :options="libraryProtocolSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, libraryProtocolSelectOptions)"
        />
        <VaSelect
          v-model="newItem.read_mode"
          :label="t('rna.read_mode')"
          class="w-full sm:w-1/2"
          :options="readModeSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, readModeSelectOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.reference"
          :label="t('rna.reference')"
          class="w-full sm:w-1/2"
          :options="referenceSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, referenceSelectOptions)"
        />
        <VaSelect
          v-model="newItem.alignment_software"
          :label="t('rna.alignment_software')"
          class="w-full sm:w-1/2"
          :options="alignmentSoftwareSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, alignmentSoftwareSelectOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.quantification_tool"
          :label="t('rna.quantification_tool')"
          class="w-full sm:w-1/2"
          :options="quantificationToolSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, quantificationToolSelectOptions)"
        />
        <VaSelect
          v-model="newItem.expression_unit"
          :label="t('rna.expression_unit')"
          class="w-full sm:w-1/2"
          :options="expressionUnitSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, expressionUnitSelectOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <div class="w-full sm:w-1/2">
          <VaDateInput
            v-model="newItem.determinated_at"
            :label="t('rna.determinated_at')"
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
          <VaButton preset="primary" size="small">{{ t('rna.meta_file') }}</VaButton>
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
          <VaButton preset="primary" size="small">{{ t('rna.exp_file') }}</VaButton>
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
        <VaFileUpload
          v-model="countFile"
          type="single"
          hide-file-list
          class="self-stretch justify-start items-center gap-4 inline-flex"
          file-types="csv"
        >
          <VaButton preset="primary" size="small">{{ t('rna.count_file') }}</VaButton>
          <div v-if="countFile" class="va-title">
            {{ `${countFile.name} ${humanFileSize(countFile.size)}` }}
          </div>
          <VaButton
            v-if="countFile"
            preset="primary"
            color="danger"
            size="small"
            icon="delete"
            class="z-10"
            @click.stop="countFile = undefined"
          />
        </VaFileUpload>
      </div>

      <div v-if="!item" class="flex gap-4 w-full">
        <p class="va-input-label">
          {{ t('omics.csv_only') }},
          <RouterLink style="text-decoration: underline" target="_blank" :to="{ name: 'faq', hash: '#quartet-rna' }">
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
          :disabled="!isValid || (!item && (!metaFile?.name || !expFile?.name || !countFile?.name))"
          :loading="qcProcessing"
          @click="onSave"
          >{{ saveButtonLabel }}</VaButton
        >
      </div>
    </div>
  </VaForm>
</template>
