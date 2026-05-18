export type AliOSSToken = {
  region: string
  access_key_id: string
  access_key_secret: string
  bucket: string
}

export type DNAFamilyMember = 5 | 6 | 7 | 8
export type DNAPlatformModel =
  | 'Illumina NovaSeq 6000/X Series'
  | 'Illumina HiSeq X Ten/2500'
  | 'Illumina NextSeq/MiSeq'
  | 'MGI DNBSEQ-T7 series'
  | 'MGI DNBSEQ T10/T20'
  | 'MGI DNBSEQ-G400 (SEQ2000)'
  | 'PacBio Sequel II/IIe'
  | 'PacBio Revio'
  | 'ONT PromethION'
  | 'ONT GridION'
  | 'ONT MinION'
  | 'Element AVITI'
  | 'Ultima UG100'
export type DNAFragmentation = 'Acoustic Shear' | 'Enzymatic' | 'Transposase' | 'No Fragmentation'
export type DNALibraryProtocol = 'PCR-free' | 'PCR-based' | 'Hybrid Capture'
export type DNAApplication = 'WGS' | 'WES' | 'Targeted Panel' | 'Low-pass WGS'
export type DNAReadMode =
  | 'PE 150bp'
  | 'PE 100bp'
  | 'PE 75bp'
  | 'SE 150bp'
  | 'SE 100bp'
  | 'Long-reads(HiFi/CCS)'
  | 'Long-reads(CLR)'
  | 'ONT Raw Reads'
export type DNAReference = 'GRCh38(hg38)' | 'GRCh37(hg19)' | 'T2T-CHM13'
export type DNAAlignmentSoftware =
  | 'BWA-MEM'
  | 'Sentieon-BWA'
  | 'minimap2'
  | 'pbmm2'
  | 'Bowtie2'
  | 'Isaac'
  | 'HISAT2'
  | 'Dragen Alignment'
export type DNAVariantCaller =
  | 'GATK HaplotypeCaller'
  | 'Sentieon Haplotyper'
  | 'DeepVariant'
  | 'FreeBayes'
  | 'cuteSV'
  | 'Sniffles'
  | 'pbsv'
  | 'SVIM'
export type DNAFiltering = 'Hard Filtration' | 'VQSR' | 'CNNScore' | 'DeepVariant Filter' | 'No Filter'

export type DNAItem = {
  id: number
  user_id: number
  name: string
  family_member: DNAFamilyMember | null
  platform_model: DNAPlatformModel | null
  dna_fragmentation: DNAFragmentation | null
  library_protocol: DNALibraryProtocol | null
  application: DNAApplication | null
  read_mode: DNAReadMode | null
  reference: DNAReference | null
  alignment_software: DNAAlignmentSoftware | null
  variant_caller: DNAVariantCaller | null
  filtering: DNAFiltering | null
  determinated_at: Date
  vcf_file: string
  snv_number: number | null
  indel_number: number | null
  snv_precision: number | null
  indel_precision: number | null
  snv_recall: number | null
  indel_recall: number | null
  notes: string | null
}

export type RNAPlatformModel =
  | 'Illumina NovaSeq 6000/X Series'
  | 'Illumina HiSeq X Ten/2500'
  | 'Illumina NextSeq/MiSeq'
  | 'MGI DNBSEQ-T7 series'
  | 'MGI DNBSEQ T10/T20'
  | 'MGI DNBSEQ-G400 (SEQ2000)'
  | 'PacBio Sequel II/IIe'
  | 'PacBio Revio'
  | 'ONT PromethION'
  | 'ONT GridION'
  | 'ONT MinION'
  | 'Element AVITI'
  | 'Ultima UG100'
export type RNAFragmentation = 'Acoustic Shear' | 'Enzymatic' | 'Chemical/Heat' | 'No Fragmentation'
export type RNAEnrichment =
  | 'Ribosomal RNA Depletion'
  | 'Poly-A Selection'
  | 'Total RNA'
  | 'Globin Depletion'
  | 'Targeted Capture'
export type RNALibraryProtocol =
  | 'Stranded (Antisense/Reverse) 3'
  | 'Stranded (Sense/Forward)'
  | 'Non-stranded 4'
  | 'SMART-Seq'
  | 'Direct RNA (ONT)'
  | 'cDNA (Full-length)'
export type RNAReadMode =
  | 'PE 150bp'
  | 'PE 100bp'
  | 'PE 75bp'
  | 'SE 100bp'
  | 'SE 75bp'
  | 'SE 50bp'
  | 'Long-reads(Iso-Seq)'
  | 'ONT Raw Reads'
export type RNAReference = 'GRCh38+Ensembl' | 'GRCh38+GENCODE' | 'GRCh37(hg19)' | 'T2T-CHM13'
export type RNAAlignmentSoftware =
  | 'STAR'
  | 'HISAT2'
  | 'Bowtie2'
  | 'TopHat2'
  | 'Salmon(Mapping-based)'
  | 'Kallisto(Pseudo-alignment)'
  | 'minimap2(Long-reads)'
export type RNAQuantificationTool = 'FeatureCounts' | 'HTSeq-count' | 'RSEM' | 'Salmon (Quasi-mapping)' | 'Kallisto'
export type RNAExpressionUnit = 'TPM' | 'FPKM' | 'RPKM' | 'CPM' | 'Normalized Counts'

export type RNAItem = {
  id: number
  user_id: number
  name: string
  platform_model: RNAPlatformModel | null
  rna_fragmentation: RNAFragmentation | null
  enrichment: RNAEnrichment | null
  library_protocol: RNALibraryProtocol | null
  read_mode: RNAReadMode | null
  reference: RNAReference | null
  alignment_software: RNAAlignmentSoftware | null
  quantification_tool: RNAQuantificationTool | null
  expression_unit: RNAExpressionUnit | null
  determinated_at: Date
  meta_file: string
  exp_file: string
  count_file: string
  snr: number
  pcc: number
  d5: number
  d6: number
  f7: number
  m8: number
  report_file: string | null
  notes: string | null
}

export type ProteinMsInstrument =
  | 'Thermo Orbitrap Exploris 480/240'
  | 'Thermo Orbitrap Astral'
  | 'Thermo Q Exactive HF-X/HF'
  | 'Bruker timsTOF Pro/Pro 2'
  | 'Bruker timsTOF HT'
  | 'SCIEX ZenoTOF 7600'
  | 'SCIEX TripleTOF 6600+'
  | 'Waters SELECT SERIES Cyclic IMS'
export type ProteinAcquisitionMethod = 'DIA' | 'DDA' | 'diaPASEF' | 'ddaPASEF' | 'PRM' | 'BoxCar' | 'SureQuant'
export type ProteinLcSystem =
  | 'Thermo Vanquish Neo'
  | 'Waters ACQUITY UPLC M-Class'
  | 'Evosep One'
  | 'Agilent 1290 Infinity II'
  | 'Shimadzu Nexera'
  | 'Nanoelute'
export type ProteinLcColumn =
  | 'C18 Reverse Phase'
  | 'Ion Exchange'
  | 'Integrated Spray Tip Column'
  | 'Performance Column(Evosep)'
export type ProteinFactionation = 'No Fractionation' | 'High-pH Reverse Phase' | 'SCX' | 'GeLC-MS'
export type ProteinDigestionEnzyme = 'Trypsin' | 'Lys-C' | 'Trypsin/Lys-C Mix' | 'Chymotrypsin' | 'Glu-C' | 'Pepsin'
export type ProteinAnalysisSoftware =
  | 'DIA-NN'
  | 'Spectronaut'
  | 'MaxQuant'
  | 'Proteome Discoverer'
  | 'FragPipe'
  | 'Skyline'
  | 'PEAKS Online'
  | 'PaSER'
export type ProteinDatabase =
  | 'UniProt Human(Canonical)'
  | 'UniProt Human(Canonical & Isoforms)'
  | 'RefSeq Human'
  | 'Swiss-Prot'
export type ProteinNormalization =
  | 'Median Normalization'
  | 'Quantile Normalization'
  | 'TIC Normalization'
  | 'Loess Normalization'
  | 'Total MS1 Intensity'
  | 'None'

export type ProteinItem = {
  id: number
  user_id: number
  name: string
  ms_instrument: ProteinMsInstrument | null
  acquisition_method: ProteinAcquisitionMethod | null
  lc_system: ProteinLcSystem | null
  lc_column: ProteinLcColumn | null
  fractionation: ProteinFactionation | null
  digestion_enzyme: ProteinDigestionEnzyme | null
  analysis_software: ProteinAnalysisSoftware | null
  database: ProteinDatabase | null
  normalization: ProteinNormalization | null
  determinated_at: Date
  meta_file: string
  exp_file: string
  features: number
  recall: number
  snr: number
  pcc: number
  d5: number
  d6: number
  f7: number
  m8: number
  report_file: string | null
  notes: string | null
}

export type MetabolismMsInstrument =
  | 'Thermo Orbitrap Exploris/QE Series'
  | 'Agilent Q-TOF(6500 series)'
  | 'Waters Vion/Synapt G2-Si'
  | 'Bruker impact II/timsTOF'
  | 'Sciex TripleTOF/Triple Quad'
  | 'AB Sciex QTRAP'
export type MetabolismDetectionStrategy = 'Untargeted' | 'Targeted' | 'Semi-targeted' | 'Lipidomics'
export type MetabolismAnalysisPlatform = 'LC-MS' | 'GC-MS' | 'NMR' | 'CE-MS' | 'DI-MS'
export type MetabolismChromatography =
  | 'Reverse Phase'
  | 'HILIC'
  | 'Normal Phase'
  | 'GC-Capillary Column'
  | 'Ion Exchange'
export type MetabolismIonization = 'ESI Positive' | 'ESI Negative' | 'APCI' | 'EI'
export type MetabolismExtractionMethod =
  | 'Methanol'
  | 'Acetonitrile'
  | 'Methanol/Water'
  | 'Chloroform/Methanol(Bligh-Dyer)'
  | 'Protein Precipitation'
export type MetabolismPreprocessingSoftware =
  | 'Compound Discoverer'
  | 'MS-DIAL'
  | 'XCMS'
  | 'MZmine'
  | 'MetaboAnalyst'
  | 'TraceFinder'
  | 'MassHunter'
  | 'LabSolutions'
export type MetabolismIdentificationLevel =
  | 'MS1 & MS2(Secondary standard)'
  | 'MS1 & RT(Accurate mass)'
  | 'MS2 Library Match'
  | 'IDMS'
export type MetabolismUnit =
  | 'Peak Intensity'
  | 'Peak Area'
  | 'Concentration(μmol/L or mg/dL)'
  | 'Internal Standard Normalized'
  | 'TIC Normalized'
  | 'PQN Normalized'

export type MetabolismItem = {
  id: number
  user_id: number
  name: string
  ms_instrument: MetabolismMsInstrument | null
  detection_strategy: MetabolismDetectionStrategy | null
  analysis_platform: MetabolismAnalysisPlatform | null
  chromatography: MetabolismChromatography | null
  ionization: MetabolismIonization | null
  extraction_method: MetabolismExtractionMethod | null
  preprocessing_software: MetabolismPreprocessingSoftware | null
  identification_level: MetabolismIdentificationLevel | null
  unit: MetabolismUnit | null
  database: string
  determinated_at: Date
  meta_file: string
  exp_file: string
  snr: number
  pcc: number
  d5: number
  d6: number
  f7: number
  m8: number
  report_file: string | null
  notes: string | null
}

export type PlasmixProteinTechnology =
  | 'Olink'
  | 'SomaScan'
  | 'DIA'
  | 'Mass Spectrometry'
  | 'Luminex'
  | 'ELISA'
  | 'Simoa'
export type PlasmixProteinModelPanel =
  | 'Thermo Orbitrap Astral'
  | 'Thermo Orbitrap Exploris 480/240'
  | 'Thermo Q Exactive HF-X/HF'
  | 'Bruker timsTOF Pro/Pro 2'
  | 'Bruker timsTOF HT'
  | 'SCIEX ZenoTOF 7600'
  | 'SCIEX TripleTOF 6600+'
  | 'Olink Explore HT'
  | 'Olink Explore 3072/1536'
  | 'Olink Target 96/48'
  | 'SomaScan 7K/11K'
  | 'Luminex 200/FLEXMAP 3D'
  | 'Simoa HD-X/HD-1'
  | 'Simoa SR-X'
  | 'Simoa SP-X'
export type PlasmixProteinPreTreatment =
  | 'Neat Plasma'
  | 'High-abundance Depletion'
  | 'Nanoparticle Enrichment'
  | 'Proximity Extension'
  | 'Aptamer Capture'
export type PlasmixProteinReadout =
  | 'Mass Spectrometer'
  | 'Illumina NGS(Olink Explore)'
  | 'qPCR(Olink Target)'
  | 'DNA Microarray(SomaScan)'
  | 'Digital Image/AEB'
export type PlasmixProteinProtocol =
  | 'In-solution Trypsin'
  | 'S-Trap/iST'
  | 'PEA Extension & Ligation'
  | 'Biotin-Aptamer Binding'
  | 'Digital ELISA'
export type PlasmixProteinExpressionUnit =
  | 'NPX'
  | 'RFU'
  | 'Log2 Intensity'
  | 'LFQ Intensity'
  | 'Concentration(pg/mL)'
  | 'AEB(Average Enzymes per Bead)'
export type PlasmixProteinAnalysisSoftware =
  | 'Olink NPX Signature/Manager'
  | 'DIA-NN'
  | 'Spectronaut'
  | 'MaxQuant'
  | 'FragPipe'
  | 'SomaLogic Analysis Software'
  | 'Skyline'
  | 'Simoa Software(Quanterix)'
export type PlasmixProteinDatabase =
  | 'UniProt Human (Canonical)'
  | 'Olink Internal Annotation'
  | 'Simoa Assay List'
  | 'SomaScan Target List'
  | 'RefSeq Human'
export type PlasmixProteinNormalization =
  | 'IPC'
  | 'Olink Bridge Normalization'
  | 'Median Normalization'
  | 'TIC Normalization'
  | 'ANML(SomaScan)'
  | 'Plate-specific Normalization'
  | 'Calibrator Curve'

export type PlasmixProteinItem = {
  id: number
  user_id: number
  name: string
  technology: PlasmixProteinTechnology | null
  model_panel: PlasmixProteinModelPanel | null
  pre_treatment: PlasmixProteinPreTreatment | null
  readout: PlasmixProteinReadout | null
  protocol: PlasmixProteinProtocol | null
  expression_unit: PlasmixProteinExpressionUnit | null
  analysis_software: PlasmixProteinAnalysisSoftware | null
  database: PlasmixProteinDatabase | null
  normalization: PlasmixProteinNormalization | null
  determinated_at: Date
  meta_file: string
  exp_file: string
  identified_proteins: number
  recall: number
  snr: number
  rc: number
  m: number
  f: number
  p: number
  x: number
  y: number
  report_file: string | null
  notes: string | null
}

export type PlasmixMetabolismMsInstrument =
  | 'Thermo Orbitrap Exploris/QE Series'
  | 'Agilent Q-TOF(6500 series)'
  | 'Waters Vion/Synapt G2-Si'
  | 'Bruker impact II/timsTOF'
  | 'Sciex TripleTOF/Triple Quad'
  | 'AB Sciex QTRAP'
export type PlasmixMetabolismDetectionStrategy = 'Untargeted' | 'Targeted' | 'Polar Metabolites' | 'Lipidomics'
export type PlasmixMetabolismAnalysisPlatform = 'LC-MS' | 'GC-MS' | 'NMR' | 'DI-MS'
export type PlasmixMetabolismChromatography =
  | 'Reverse Phase'
  | 'HILIC'
  | 'Normal Phase'
  | 'GC-Capillary Column'
  | 'Biphasic/Mixed-mode'
export type PlasmixMetabolismIonization = 'ESI Positive' | 'ESI Negative' | 'Dual Polarity' | 'APCI' | 'EI'
export type PlasmixMetabolismExtractionMethod =
  | 'Methanol'
  | 'Acetonitrile'
  | 'Methanol:Acetonitrile'
  | 'Chloroform:Methanol'
  | 'SPE'
  | 'None'
export type PlasmixMetabolismPreprocessingSoftware =
  | 'Compound Discoverer'
  | 'MS-DIAL'
  | 'XCMS'
  | 'MZmine'
  | 'MetaboAnalyst'
  | 'TraceFinder'
  | 'MassHunter'
  | 'LabSolutions'
export type PlasmixMetabolismNormalization =
  | 'Internal Standard'
  | 'QC-RLSC'
  | 'TIC Normalization'
  | 'Median Normalization'
  | 'PQN Normalization'
  | 'None'
export type PlasmixMetabolismIdentificationBasis =
  | 'MS1 & MS2 & RT'
  | 'MS1 & MS2'
  | 'Accurate Mass'
  | 'NMR Chemical Shift'

export type PlasmixMetabolismItem = {
  id: number
  user_id: number
  name: string
  ms_instrument: PlasmixMetabolismMsInstrument | null
  detection_strategy: PlasmixMetabolismDetectionStrategy | null
  analysis_platform: PlasmixMetabolismAnalysisPlatform | null
  chromatography: PlasmixMetabolismChromatography | null
  ionization: PlasmixMetabolismIonization | null
  extraction_method: PlasmixMetabolismExtractionMethod | null
  preprocessing_software: PlasmixMetabolismPreprocessingSoftware | null
  normalization: PlasmixMetabolismNormalization | null
  identification_basis: PlasmixMetabolismIdentificationBasis | null
  database: string
  determinated_at: Date
  meta_file: string
  exp_file: string
  snr: number
  pcc: number
  m: number
  f: number
  p: number
  x: number
  y: number
  report_file: string | null
  notes: string | null
}
