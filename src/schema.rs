// @generated automatically by Diesel CLI.

diesel::table! {
    dna (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 128]
        name -> Varchar,
        family_member -> Integer,
        #[max_length = 128]
        platform_model -> Varchar,
        #[max_length = 128]
        dna_fragmentation -> Varchar,
        #[max_length = 128]
        library_protocol -> Varchar,
        #[max_length = 128]
        application -> Varchar,
        #[max_length = 128]
        read_mode -> Varchar,
        #[max_length = 128]
        reference -> Varchar,
        #[max_length = 128]
        alignment_software -> Varchar,
        #[max_length = 128]
        variant_caller -> Varchar,
        #[max_length = 128]
        filtering -> Varchar,
        determinated_at -> Timestamp,
        #[max_length = 255]
        vcf_file -> Varchar,
        snv_number -> Nullable<Integer>,
        indel_number -> Nullable<Integer>,
        snv_precision -> Nullable<Float>,
        indel_precision -> Nullable<Float>,
        snv_recall -> Nullable<Float>,
        indel_recall -> Nullable<Float>,
        #[max_length = 255]
        notes -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    metabolism (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        ms_instrument -> Varchar,
        #[max_length = 128]
        detection_strategy -> Varchar,
        #[max_length = 128]
        analysis_platform -> Varchar,
        #[max_length = 128]
        chromatography -> Varchar,
        #[max_length = 128]
        ionization -> Varchar,
        #[max_length = 128]
        extraction_method -> Varchar,
        #[max_length = 128]
        preprocessing_software -> Varchar,
        #[max_length = 128]
        identification_level -> Varchar,
        #[max_length = 128]
        unit -> Varchar,
        #[max_length = 128]
        database -> Varchar,
        determinated_at -> Timestamp,
        #[max_length = 255]
        meta_file -> Varchar,
        #[max_length = 255]
        exp_file -> Varchar,
        snr -> Float,
        pcc -> Float,
        d5 -> Integer,
        d6 -> Integer,
        f7 -> Integer,
        m8 -> Integer,
        #[max_length = 255]
        report_file -> Nullable<Varchar>,
        #[max_length = 255]
        notes -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    plasmix_metabolism (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        ms_instrument -> Varchar,
        #[max_length = 128]
        detection_strategy -> Varchar,
        #[max_length = 128]
        analysis_platform -> Varchar,
        #[max_length = 128]
        chromatography -> Varchar,
        #[max_length = 128]
        ionization -> Varchar,
        #[max_length = 128]
        extraction_method -> Varchar,
        #[max_length = 128]
        preprocessing_software -> Varchar,
        #[max_length = 128]
        normalization -> Varchar,
        #[max_length = 128]
        identification_basis -> Varchar,
        #[max_length = 128]
        database -> Varchar,
        determinated_at -> Timestamp,
        #[max_length = 255]
        meta_file -> Varchar,
        #[max_length = 255]
        exp_file -> Varchar,
        snr -> Float,
        pcc -> Float,
        m -> Integer,
        f -> Integer,
        p -> Integer,
        x -> Integer,
        y -> Integer,
        #[max_length = 255]
        report_file -> Nullable<Varchar>,
        #[max_length = 255]
        notes -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    plasmix_protein (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        technology -> Varchar,
        #[max_length = 128]
        model_panel -> Varchar,
        #[max_length = 128]
        pre_treatment -> Varchar,
        #[max_length = 128]
        readout -> Varchar,
        #[max_length = 128]
        protocol -> Varchar,
        #[max_length = 128]
        expression_unit -> Varchar,
        #[max_length = 128]
        analysis_software -> Varchar,
        #[max_length = 128]
        database -> Varchar,
        #[max_length = 128]
        normalization -> Varchar,
        determinated_at -> Timestamp,
        #[max_length = 255]
        meta_file -> Varchar,
        #[max_length = 255]
        exp_file -> Varchar,
        identified_proteins -> Integer,
        recall -> Float,
        snr -> Float,
        rc -> Float,
        m -> Integer,
        f -> Integer,
        p -> Integer,
        x -> Integer,
        y -> Integer,
        #[max_length = 255]
        report_file -> Nullable<Varchar>,
        #[max_length = 255]
        notes -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    protein (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        ms_instrument -> Varchar,
        #[max_length = 128]
        acquisition_method -> Varchar,
        #[max_length = 128]
        lc_system -> Varchar,
        #[max_length = 128]
        lc_column -> Varchar,
        #[max_length = 128]
        fractionation -> Varchar,
        #[max_length = 128]
        digestion_enzyme -> Varchar,
        #[max_length = 128]
        analysis_software -> Varchar,
        #[max_length = 128]
        database -> Varchar,
        #[max_length = 128]
        normalization -> Varchar,
        determinated_at -> Timestamp,
        #[max_length = 255]
        meta_file -> Varchar,
        #[max_length = 255]
        exp_file -> Varchar,
        features -> Integer,
        recall -> Float,
        snr -> Float,
        pcc -> Float,
        d5 -> Integer,
        d6 -> Integer,
        f7 -> Integer,
        m8 -> Integer,
        #[max_length = 255]
        report_file -> Nullable<Varchar>,
        #[max_length = 255]
        notes -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rna (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        platform_model -> Varchar,
        #[max_length = 128]
        rna_fragmentation -> Varchar,
        #[max_length = 128]
        enrichment -> Varchar,
        #[max_length = 128]
        library_protocol -> Varchar,
        #[max_length = 128]
        read_mode -> Varchar,
        #[max_length = 128]
        reference -> Varchar,
        #[max_length = 128]
        alignment_software -> Varchar,
        #[max_length = 128]
        quantification_tool -> Varchar,
        #[max_length = 128]
        expression_unit -> Varchar,
        determinated_at -> Timestamp,
        #[max_length = 255]
        meta_file -> Varchar,
        #[max_length = 255]
        exp_file -> Varchar,
        #[max_length = 255]
        count_file -> Varchar,
        snr -> Float,
        pcc -> Float,
        d5 -> Integer,
        d6 -> Integer,
        f7 -> Integer,
        m8 -> Integer,
        #[max_length = 255]
        report_file -> Nullable<Varchar>,
        #[max_length = 255]
        notes -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_scope (user_id, managed_user_id) {
        user_id -> Integer,
        managed_user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 128]
        username -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        role -> Integer,
        active -> Bool,
        #[max_length = 255]
        notes -> Varchar,
        last_seen -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    dna,
    metabolism,
    plasmix_metabolism,
    plasmix_protein,
    protein,
    rna,
    user_scope,
    users,
);
