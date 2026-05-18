<script setup lang="ts">
import { PropType, computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useForm } from 'vuestic-ui'
import {
  AliOSSToken,
  DNAItem,
  DNAFamilyMember,
  DNAPlatformModel,
  DNAFragmentation,
  DNALibraryProtocol,
  DNAApplication,
  DNAReadMode,
  DNAReference,
  DNAAlignmentSoftware,
  DNAVariantCaller,
  DNAFiltering,
} from '../types'
import { validators, humanFileSize } from '../../../services/utils'
import { addNewOption, formatDate, makeAliOssClient, renameFile } from '../composables/useOmics'

const { t } = useI18n()

const prefix = 'dna'

const props = defineProps({
  item: {
    type: Object as PropType<DNAItem | null>,
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

// prettier-ignore
type editDNAItem = Omit<DNAItem, 'id' | 'user_id' | 'snv_number' | 'indel_number' | 'snv_precision' | 'indel_precision' | 'snv_recall' | 'indel_recall' | 'notes'>

const defaultNewItem: editDNAItem = {
  name: '',
  family_member: null,
  platform_model: null,
  dna_fragmentation: null,
  library_protocol: null,
  application: null,
  read_mode: null,
  reference: null,
  alignment_software: null,
  variant_caller: null,
  filtering: null,
  determinated_at: new Date(),
  vcf_file: '',
}

const newItem = ref<DNAItem>({ ...defaultNewItem } as DNAItem)

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
    return newItem.value[key as keyof editDNAItem] !== (props.item ?? defaultNewItem)?.[key as keyof editDNAItem]
  })
})

defineExpose({
  isFormHasUnsavedChanges,
})

const vcfFile = ref<File>()
const qcProcessing = ref(false)
const errorMsg = ref('')

const form = useForm('add-dna-form')

const emit = defineEmits(['close', 'save'])

const onSave = async () => {
  errorMsg.value = ''
  if (form.validate()) {
    try {
      qcProcessing.value = true
      if (props.token) {
        const oSSClient = makeAliOssClient(props.token)
        const result = await oSSClient.put(renameFile(prefix, vcfFile.value!.name), vcfFile.value!)
        console.log(`${vcfFile.value!.name} uploaded to ${result.url}`)
        newItem.value.vcf_file = result.name
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

const familyMemberSelectOptions: { text: string; value: DNAFamilyMember }[] = [
  { text: t('dna.d5'), value: 5 },
  { text: t('dna.d6'), value: 6 },
  { text: t('dna.f7'), value: 7 },
  { text: t('dna.m8'), value: 8 },
]

const platformSelectOptions = ref<{ text: DNAPlatformModel; value: DNAPlatformModel }[]>([
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

const fragmentationSelectOptions = ref<{ text: string; value: DNAFragmentation }[]>([
  { text: t('dna.acoustic_shear'), value: 'Acoustic Shear' },
  { text: t('dna.enzymatic'), value: 'Enzymatic' },
  { text: t('dna.transposase'), value: 'Transposase' },
  { text: t('dna.no_fragmentation'), value: 'No Fragmentation' },
])

const protocolSelectOptions = ref<{ text: string; value: DNALibraryProtocol }[]>([
  { text: t('dna.pcr_free'), value: 'PCR-free' },
  { text: t('dna.pcr_based'), value: 'PCR-based' },
  { text: t('dna.hybrid_capture'), value: 'Hybrid Capture' },
])

const applicationSelectOptions = ref<{ text: string; value: DNAApplication }[]>([
  { text: t('dna.wgs'), value: 'WGS' },
  { text: t('dna.wes'), value: 'WES' },
  { text: t('dna.targeted_panel'), value: 'Targeted Panel' },
  { text: t('dna.low_pass_wgs'), value: 'Low-pass WGS' },
])

const readModeSelectOptions = ref<{ text: DNAReadMode; value: DNAReadMode }[]>([
  { text: 'PE 150bp', value: 'PE 150bp' },
  { text: 'PE 100bp', value: 'PE 100bp' },
  { text: 'PE 75bp', value: 'PE 75bp' },
  { text: 'SE 150bp', value: 'SE 150bp' },
  { text: 'SE 100bp', value: 'SE 100bp' },
  { text: 'Long-reads(HiFi/CCS)', value: 'Long-reads(HiFi/CCS)' },
  { text: 'Long-reads(CLR)', value: 'Long-reads(CLR)' },
  { text: 'ONT Raw Reads', value: 'ONT Raw Reads' },
])

const referenceSelectOptions = ref<{ text: DNAReference; value: DNAReference }[]>([
  { text: 'GRCh38(hg38)', value: 'GRCh38(hg38)' },
  { text: 'GRCh37(hg19)', value: 'GRCh37(hg19)' },
  { text: 'T2T-CHM13', value: 'T2T-CHM13' },
])

const alignmentSelectOptions = ref<{ text: DNAAlignmentSoftware; value: DNAAlignmentSoftware }[]>([
  { text: 'BWA-MEM', value: 'BWA-MEM' },
  { text: 'Sentieon-BWA', value: 'Sentieon-BWA' },
  { text: 'minimap2', value: 'minimap2' },
  { text: 'pbmm2', value: 'pbmm2' },
  { text: 'Bowtie2', value: 'Bowtie2' },
  { text: 'Isaac', value: 'Isaac' },
  { text: 'HISAT2', value: 'HISAT2' },
  { text: 'Dragen Alignment', value: 'Dragen Alignment' },
])

const vcSelectOptions = ref<{ text: DNAVariantCaller; value: DNAVariantCaller }[]>([
  { text: 'GATK HaplotypeCaller', value: 'GATK HaplotypeCaller' },
  { text: 'Sentieon Haplotyper', value: 'Sentieon Haplotyper' },
  { text: 'DeepVariant', value: 'DeepVariant' },
  { text: 'FreeBayes', value: 'FreeBayes' },
  { text: 'cuteSV', value: 'cuteSV' },
  { text: 'Sniffles', value: 'Sniffles' },
  { text: 'pbsv', value: 'pbsv' },
  { text: 'SVIM', value: 'SVIM' },
])

const filtermingSelectOptions = ref<{ text: DNAFiltering; value: DNAFiltering }[]>([
  { text: 'Hard Filtration', value: 'Hard Filtration' },
  { text: 'VQSR', value: 'VQSR' },
  { text: 'CNNScore', value: 'CNNScore' },
  { text: 'DeepVariant Filter', value: 'DeepVariant Filter' },
  { text: 'No Filter', value: 'No Filter' },
])
</script>

<template>
  <VaForm v-slot="{ isValid }" ref="add-dna-form" class="flex-col justify-start items-start gap-4 inline-flex w-full">
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput
          v-model="newItem.name"
          :label="t('dna.name')"
          class="w-full sm:w-1/2"
          placeholder="格式：批次号_其他描述性信息"
          :rules="[validators.required]"
        />
        <VaSelect
          v-model="newItem.family_member"
          :label="t('dna.sample_id')"
          class="w-full sm:w-1/2"
          :disabled="!!item"
          :options="familyMemberSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.platform_model"
          :label="t('dna.platform_model')"
          class="w-full sm:w-1/2"
          :options="platformSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, platformSelectOptions)"
        />
        <VaSelect
          v-model="newItem.dna_fragmentation"
          :label="t('dna.dna_fragmentation')"
          class="w-full sm:w-1/2"
          :options="fragmentationSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, fragmentationSelectOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.library_protocol"
          :label="t('dna.library_protocol')"
          class="w-full sm:w-1/2"
          :options="protocolSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, protocolSelectOptions)"
        />
        <VaSelect
          v-model="newItem.application"
          :label="t('dna.application')"
          class="w-full sm:w-1/2"
          :options="applicationSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.read_mode"
          :label="t('dna.read_mode')"
          class="w-full sm:w-1/2"
          :options="readModeSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, readModeSelectOptions)"
        />
        <VaSelect
          v-model="newItem.reference"
          :label="t('dna.reference')"
          class="w-full sm:w-1/2"
          :options="referenceSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, referenceSelectOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.alignment_software"
          :label="t('dna.alignment_software')"
          class="w-full sm:w-1/2"
          :options="alignmentSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, alignmentSelectOptions)"
        />
        <VaSelect
          v-model="newItem.variant_caller"
          :label="t('dna.variant_caller')"
          class="w-full sm:w-1/2"
          :options="vcSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, vcSelectOptions)"
        />
      </div>

      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaSelect
          v-model="newItem.filtering"
          :label="t('dna.filtering')"
          class="w-full sm:w-1/2"
          :options="filtermingSelectOptions"
          :rules="[validators.required]"
          value-by="value"
          :placeholder="t('vuestic.select')"
          allow-create="unique"
          :search-placeholder-text="t('search.placeholder')"
          @createNew="(option: any) => addNewOption(option, filtermingSelectOptions)"
        />
        <div class="w-full sm:w-1/2">
          <VaDateInput
            v-model="newItem.determinated_at"
            :label="t('dna.determinated_at')"
            class="w-full"
            :format-date="formatDate"
            :rules="[validators.required]"
          />
        </div>
      </div>

      <div v-if="!item">
        <div class="flex gap-4 w-full">
          <VaFileUpload
            v-model="vcfFile"
            type="single"
            hide-file-list
            class="self-stretch justify-start items-center gap-4 inline-flex"
            file-types="vcf,vcf.gz"
          >
            <VaButton preset="primary" size="small">{{ t('dna.vcf_file') }}</VaButton>
            <div v-if="vcfFile" class="va-title">
              {{ `${vcfFile.name} ${humanFileSize(vcfFile.size)}` }}
            </div>
            <VaButton
              v-if="vcfFile"
              preset="primary"
              color="danger"
              size="small"
              icon="delete"
              class="z-10"
              @click.stop="vcfFile = undefined"
            />
          </VaFileUpload>
        </div>
        <p class="va-input-label">{{ t('dna.vcf_only') }}</p>
      </div>

      <div v-if="!item && !!errorMsg" class="flex gap-4 w-full">
        <VaAlert dense closeable color="danger" class="w-full">
          {{ errorMsg }}
        </VaAlert>
      </div>

      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton preset="secondary" color="secondary" @click="$emit('close')">{{ t('omics.cancel') }}</VaButton>
        <VaButton :disabled="!isValid || (!item && !vcfFile?.name)" :loading="qcProcessing" @click="onSave">{{
          saveButtonLabel
        }}</VaButton>
      </div>
    </div>
  </VaForm>
</template>
