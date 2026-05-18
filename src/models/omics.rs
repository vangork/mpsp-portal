use crate::db::Connection;
use crate::schema::{dna, metabolism, plasmix_metabolism, plasmix_protein, protein, rna, users};
use crate::utils::js_date_to_naive_datetime;
use chrono::NaiveDateTime;
use diesel::sql_types::Integer;
use diesel::{Identifiable, Insertable, Queryable, prelude::*};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::{AsyncConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = dna)]
pub struct DNA {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub family_member: i32,
    pub platform_model: String,
    pub dna_fragmentation: String,
    pub library_protocol: String,
    pub application: String,
    pub read_mode: String,
    pub reference: String,
    pub alignment_software: String,
    pub variant_caller: String,
    pub filtering: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub vcf_file: String,
    pub snv_number: Option<i32>,
    pub indel_number: Option<i32>,
    pub snv_precision: Option<f32>,
    pub indel_precision: Option<f32>,
    pub snv_recall: Option<f32>,
    pub indel_recall: Option<f32>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = dna)]
pub struct NewDNA {
    pub name: String,
    pub user_id: Option<i32>,
    pub family_member: i32,
    pub platform_model: String,
    pub dna_fragmentation: String,
    pub library_protocol: String,
    pub application: String,
    pub read_mode: String,
    pub reference: String,
    pub alignment_software: String,
    pub variant_caller: String,
    pub filtering: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub vcf_file: String,
}

#[derive(Serialize, Deserialize)]
pub struct DNAScore {
    pub snv_number: Option<i32>,
    pub indel_number: Option<i32>,
    pub snv_precision: Option<f32>,
    pub indel_precision: Option<f32>,
    pub snv_recall: Option<f32>,
    pub indel_recall: Option<f32>,
    pub notes: Option<String>,
}

#[derive(QueryableByName)]
pub struct Sequence {
    #[diesel(sql_type = Integer)]
    pub id: i32,
}

impl DNA {
    pub async fn get_all(conn: &mut Connection) -> QueryResult<Vec<DNA>> {
        dna::dsl::dna
            .order(dna::dsl::id.desc())
            .load::<DNA>(conn)
            .await
    }

    pub async fn get_all_with_username(conn: &mut Connection) -> QueryResult<Vec<(DNA, String)>> {
        dna::dsl::dna
            .inner_join(users::table.on(dna::dsl::user_id.eq(users::dsl::id)))
            .select((dna::all_columns, users::dsl::username))
            .order(dna::dsl::id.desc())
            .load::<(DNA, String)>(conn)
            .await
    }

    pub async fn get_all_by_user_id(user_id: i32, conn: &mut Connection) -> QueryResult<Vec<DNA>> {
        dna::dsl::dna
            .filter(dna::dsl::user_id.eq(user_id))
            .order(dna::dsl::id.desc())
            .load::<DNA>(conn)
            .await
    }

    pub async fn find_by_id(dna_id: i32, conn: &mut Connection) -> QueryResult<DNA> {
        dna::dsl::dna.find(dna_id).get_result::<DNA>(conn).await
    }

    pub async fn add(
        mut item: NewDNA,
        user_id: i32,
        conn: &mut Connection,
    ) -> Result<i32, diesel::result::Error> {
        item.user_id = Some(user_id);

        let rows = conn
            .transaction::<_, diesel::result::Error, _>(|connection| {
                async move {
                    // Step 1: Execute the insert
                    diesel::insert_into(dna::table)
                        .values(item)
                        .execute(connection)
                        .await?;

                    // Step 2: Get the ID within the same transaction/connection
                    let value = diesel::sql_query("SELECT LAST_INSERT_ID() as id")
                        .load::<Sequence>(connection)
                        .await?;
                    Ok(value)
                }
                .scope_boxed()
            })
            .await?;
        Ok(rows[0].id)
    }

    pub async fn update(
        &self,
        item: Self,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(dna::table.find(self.id))
            .set((
                dna::dsl::name.eq(item.name),
                dna::dsl::platform_model.eq(item.platform_model),
                dna::dsl::dna_fragmentation.eq(item.dna_fragmentation),
                dna::dsl::library_protocol.eq(item.library_protocol),
                dna::dsl::application.eq(item.application),
                dna::dsl::read_mode.eq(item.read_mode),
                dna::dsl::reference.eq(item.reference),
                dna::dsl::alignment_software.eq(item.alignment_software),
                dna::dsl::variant_caller.eq(item.variant_caller),
                dna::dsl::filtering.eq(item.filtering),
                dna::dsl::determinated_at.eq(item.determinated_at),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_score(
        dna_id: i32,
        score: DNAScore,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(dna::table.find(dna_id))
            .set((
                dna::dsl::snv_number.eq(score.snv_number),
                dna::dsl::indel_number.eq(score.indel_number),
                dna::dsl::snv_precision.eq(score.snv_precision),
                dna::dsl::indel_precision.eq(score.indel_precision),
                dna::dsl::snv_recall.eq(score.snv_recall),
                dna::dsl::indel_recall.eq(score.indel_recall),
                dna::dsl::notes.eq(score.notes),
            ))
            .execute(conn)
            .await
    }
}

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = rna)]
pub struct RNA {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub platform_model: String,
    pub rna_fragmentation: String,
    pub enrichment: String,
    pub library_protocol: String,
    pub read_mode: String,
    pub reference: String,
    pub alignment_software: String,
    pub quantification_tool: String,
    pub expression_unit: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub count_file: String,
    pub snr: f32,
    pub pcc: f32,
    pub d5: i32,
    pub d6: i32,
    pub f7: i32,
    pub m8: i32,
    pub report_file: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = rna)]
pub struct NewRNA {
    pub name: String,
    pub user_id: Option<i32>,
    pub platform_model: String,
    pub rna_fragmentation: String,
    pub enrichment: String,
    pub library_protocol: String,
    pub read_mode: String,
    pub reference: String,
    pub alignment_software: String,
    pub quantification_tool: String,
    pub expression_unit: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub count_file: String,
    pub snr: f32,
    pub pcc: f32,
    pub d5: i32,
    pub d6: i32,
    pub f7: i32,
    pub m8: i32,
}

impl RNA {
    pub async fn get_all(conn: &mut Connection) -> QueryResult<Vec<RNA>> {
        rna::dsl::rna
            .order(rna::dsl::id.desc())
            .load::<RNA>(conn)
            .await
    }

    pub async fn get_all_with_username(conn: &mut Connection) -> QueryResult<Vec<(RNA, String)>> {
        rna::dsl::rna
            .inner_join(users::table.on(rna::dsl::user_id.eq(users::dsl::id)))
            .select((rna::all_columns, users::dsl::username))
            .order(rna::dsl::id.desc())
            .load::<(RNA, String)>(conn)
            .await
    }

    pub async fn get_all_by_user_id(user_id: i32, conn: &mut Connection) -> QueryResult<Vec<RNA>> {
        rna::dsl::rna
            .filter(rna::dsl::user_id.eq(user_id))
            .order(rna::dsl::id.desc())
            .load::<RNA>(conn)
            .await
    }

    pub async fn find_by_id(rna_id: i32, conn: &mut Connection) -> QueryResult<RNA> {
        rna::dsl::rna.find(rna_id).get_result::<RNA>(conn).await
    }

    pub async fn add(
        mut item: NewRNA,
        user_id: i32,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        item.user_id = Some(user_id);
        diesel::insert_into(rna::table)
            .values(item)
            .execute(conn)
            .await
    }

    pub async fn update(
        &self,
        item: Self,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(rna::table.find(self.id))
            .set((
                rna::dsl::name.eq(item.name),
                rna::dsl::platform_model.eq(item.platform_model),
                rna::dsl::rna_fragmentation.eq(item.rna_fragmentation),
                rna::dsl::enrichment.eq(item.enrichment),
                rna::dsl::library_protocol.eq(item.library_protocol),
                rna::dsl::read_mode.eq(item.read_mode),
                rna::dsl::reference.eq(item.reference),
                rna::dsl::alignment_software.eq(item.alignment_software),
                rna::dsl::quantification_tool.eq(item.quantification_tool),
                rna::dsl::expression_unit.eq(item.expression_unit),
                rna::dsl::determinated_at.eq(item.determinated_at),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_score(
        &self,
        score: (f32, f32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(rna::table.find(self.id))
            .set((rna::dsl::snr.eq(score.0), rna::dsl::pcc.eq(score.1)))
            .execute(conn)
            .await
    }

    pub async fn update_samples(
        &self,
        number: (i32, i32, i32, i32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(rna::table.find(self.id))
            .set((
                rna::dsl::d5.eq(number.0),
                rna::dsl::d6.eq(number.1),
                rna::dsl::f7.eq(number.2),
                rna::dsl::m8.eq(number.3),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_report(
        &self,
        report_file: &String,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(rna::table.find(self.id))
            .set(rna::dsl::report_file.eq(report_file))
            .execute(conn)
            .await
    }

    pub async fn update_notes(
        rna_id: i32,
        notes: Option<String>,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(rna::table.find(rna_id))
            .set(rna::dsl::notes.eq(notes))
            .execute(conn)
            .await
    }
}

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = protein)]
pub struct Protein {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub ms_instrument: String,
    pub acquisition_method: String,
    pub lc_system: String,
    pub lc_column: String,
    pub fractionation: String,
    pub digestion_enzyme: String,
    pub analysis_software: String,
    pub database: String,
    pub normalization: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub features: i32,
    pub recall: f32,
    pub snr: f32,
    pub pcc: f32,
    pub d5: i32,
    pub d6: i32,
    pub f7: i32,
    pub m8: i32,
    pub report_file: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = protein)]
pub struct NewProtein {
    pub name: String,
    pub user_id: Option<i32>,
    pub ms_instrument: String,
    pub acquisition_method: String,
    pub lc_system: String,
    pub lc_column: String,
    pub fractionation: String,
    pub digestion_enzyme: String,
    pub analysis_software: String,
    pub database: String,
    pub normalization: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub features: i32,
    pub recall: f32,
    pub snr: f32,
    pub pcc: f32,
    pub d5: i32,
    pub d6: i32,
    pub f7: i32,
    pub m8: i32,
}

impl Protein {
    pub async fn get_all(conn: &mut Connection) -> QueryResult<Vec<Protein>> {
        protein::dsl::protein
            .order(protein::dsl::id.desc())
            .load::<Protein>(conn)
            .await
    }

    pub async fn get_all_by_user_id(
        user_id: i32,
        conn: &mut Connection,
    ) -> QueryResult<Vec<Protein>> {
        protein::dsl::protein
            .filter(protein::dsl::user_id.eq(user_id))
            .order(protein::dsl::id.desc())
            .load::<Protein>(conn)
            .await
    }

    pub async fn get_all_with_username(
        conn: &mut Connection,
    ) -> QueryResult<Vec<(Protein, String)>> {
        protein::dsl::protein
            .inner_join(users::table.on(protein::dsl::user_id.eq(users::dsl::id)))
            .select((protein::all_columns, users::dsl::username))
            .order(protein::dsl::id.desc())
            .load::<(Protein, String)>(conn)
            .await
    }

    pub async fn find_by_id(protein_id: i32, conn: &mut Connection) -> QueryResult<Protein> {
        protein::dsl::protein
            .find(protein_id)
            .get_result::<Protein>(conn)
            .await
    }

    pub async fn add(
        mut item: NewProtein,
        user_id: i32,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        item.user_id = Some(user_id);
        diesel::insert_into(protein::table)
            .values(item)
            .execute(conn)
            .await
    }

    pub async fn update(
        &self,
        item: Self,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(protein::table.find(self.id))
            .set((
                protein::dsl::name.eq(item.name),
                protein::dsl::ms_instrument.eq(item.ms_instrument),
                protein::dsl::acquisition_method.eq(item.acquisition_method),
                protein::dsl::lc_system.eq(item.lc_system),
                protein::dsl::lc_column.eq(item.lc_column),
                protein::dsl::fractionation.eq(item.fractionation),
                protein::dsl::digestion_enzyme.eq(item.digestion_enzyme),
                protein::dsl::analysis_software.eq(item.analysis_software),
                protein::dsl::database.eq(item.database),
                protein::dsl::normalization.eq(item.normalization),
                protein::dsl::determinated_at.eq(item.determinated_at),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_score(
        &self,
        score: (i32, f32, f32, f32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(protein::table.find(self.id))
            .set((
                protein::dsl::features.eq(score.0),
                protein::dsl::recall.eq(score.1),
                protein::dsl::snr.eq(score.2),
                protein::dsl::pcc.eq(score.3),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_samples(
        &self,
        number: (i32, i32, i32, i32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(protein::table.find(self.id))
            .set((
                protein::dsl::d5.eq(number.0),
                protein::dsl::d6.eq(number.1),
                protein::dsl::f7.eq(number.2),
                protein::dsl::m8.eq(number.3),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_report(
        &self,
        report_file: &String,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(protein::table.find(self.id))
            .set(protein::dsl::report_file.eq(report_file))
            .execute(conn)
            .await
    }

    pub async fn update_notes(
        protein_id: i32,
        notes: Option<String>,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(protein::table.find(protein_id))
            .set(protein::dsl::notes.eq(notes))
            .execute(conn)
            .await
    }
}

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = metabolism)]
pub struct Metabolism {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub ms_instrument: String,
    pub detection_strategy: String,
    pub analysis_platform: String,
    pub chromatography: String,
    pub ionization: String,
    pub extraction_method: String,
    pub preprocessing_software: String,
    pub identification_level: String,
    pub unit: String,
    pub database: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub snr: f32,
    pub pcc: f32,
    pub d5: i32,
    pub d6: i32,
    pub f7: i32,
    pub m8: i32,
    pub report_file: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = metabolism)]
pub struct NewMetabolism {
    pub name: String,
    pub user_id: Option<i32>,
    pub ms_instrument: String,
    pub detection_strategy: String,
    pub analysis_platform: String,
    pub chromatography: String,
    pub ionization: String,
    pub extraction_method: String,
    pub preprocessing_software: String,
    pub identification_level: String,
    pub unit: String,
    pub database: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub snr: f32,
    pub pcc: f32,
    pub d5: i32,
    pub d6: i32,
    pub f7: i32,
    pub m8: i32,
}

impl Metabolism {
    pub async fn get_all(conn: &mut Connection) -> QueryResult<Vec<Metabolism>> {
        metabolism::dsl::metabolism
            .order(metabolism::dsl::id.desc())
            .load::<Metabolism>(conn)
            .await
    }

    pub async fn get_all_with_username(
        conn: &mut Connection,
    ) -> QueryResult<Vec<(Metabolism, String)>> {
        metabolism::dsl::metabolism
            .inner_join(users::table.on(metabolism::dsl::user_id.eq(users::dsl::id)))
            .select((metabolism::all_columns, users::dsl::username))
            .order(metabolism::dsl::id.desc())
            .load::<(Metabolism, String)>(conn)
            .await
    }

    pub async fn get_all_by_user_id(
        user_id: i32,
        conn: &mut Connection,
    ) -> QueryResult<Vec<Metabolism>> {
        metabolism::dsl::metabolism
            .filter(metabolism::dsl::user_id.eq(user_id))
            .order(metabolism::dsl::id.desc())
            .load::<Metabolism>(conn)
            .await
    }

    pub async fn find_by_id(metabolism_id: i32, conn: &mut Connection) -> QueryResult<Metabolism> {
        metabolism::dsl::metabolism
            .find(metabolism_id)
            .get_result::<Metabolism>(conn)
            .await
    }

    pub async fn add(
        mut item: NewMetabolism,
        user_id: i32,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        item.user_id = Some(user_id);
        diesel::insert_into(metabolism::table)
            .values(item)
            .execute(conn)
            .await
    }

    pub async fn update(
        &self,
        item: Self,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(metabolism::table.find(self.id))
            .set((
                metabolism::dsl::name.eq(item.name),
                metabolism::dsl::ms_instrument.eq(item.ms_instrument),
                metabolism::dsl::detection_strategy.eq(item.detection_strategy),
                metabolism::dsl::analysis_platform.eq(item.analysis_platform),
                metabolism::dsl::chromatography.eq(item.chromatography),
                metabolism::dsl::ionization.eq(item.ionization),
                metabolism::dsl::extraction_method.eq(item.extraction_method),
                metabolism::dsl::preprocessing_software.eq(item.preprocessing_software),
                metabolism::dsl::identification_level.eq(item.identification_level),
                metabolism::dsl::unit.eq(item.unit),
                metabolism::dsl::database.eq(item.database),
                metabolism::dsl::determinated_at.eq(item.determinated_at),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_score(
        &self,
        score: (f32, f32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(metabolism::table.find(self.id))
            .set((
                metabolism::dsl::snr.eq(score.0),
                metabolism::dsl::pcc.eq(score.1),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_samples(
        &self,
        number: (i32, i32, i32, i32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(metabolism::table.find(self.id))
            .set((
                metabolism::dsl::d5.eq(number.0),
                metabolism::dsl::d6.eq(number.1),
                metabolism::dsl::f7.eq(number.2),
                metabolism::dsl::m8.eq(number.3),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_report(
        &self,
        report_file: &String,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(metabolism::table.find(self.id))
            .set(metabolism::dsl::report_file.eq(report_file))
            .execute(conn)
            .await
    }

    pub async fn update_notes(
        metabolism_id: i32,
        notes: Option<String>,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(metabolism::table.find(metabolism_id))
            .set(metabolism::dsl::notes.eq(notes))
            .execute(conn)
            .await
    }
}

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = plasmix_protein)]
pub struct PlasmixProtein {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub technology: String,
    pub model_panel: String,
    pub pre_treatment: String,
    pub readout: String,
    pub protocol: String,
    pub expression_unit: String,
    pub analysis_software: String,
    pub database: String,
    pub normalization: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub identified_proteins: i32,
    pub recall: f32,
    pub snr: f32,
    pub rc: f32,
    pub m: i32,
    pub f: i32,
    pub p: i32,
    pub x: i32,
    pub y: i32,
    pub report_file: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = plasmix_protein)]
pub struct NewPlasmixProtein {
    pub name: String,
    pub user_id: Option<i32>,
    pub technology: String,
    pub model_panel: String,
    pub pre_treatment: String,
    pub readout: String,
    pub protocol: String,
    pub expression_unit: String,
    pub analysis_software: String,
    pub database: String,
    pub normalization: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub identified_proteins: i32,
    pub recall: f32,
    pub snr: f32,
    pub rc: f32,
    pub m: i32,
    pub f: i32,
    pub p: i32,
    pub x: i32,
    pub y: i32,
}

impl PlasmixProtein {
    pub async fn get_all(conn: &mut Connection) -> QueryResult<Vec<PlasmixProtein>> {
        plasmix_protein::dsl::plasmix_protein
            .order(plasmix_protein::dsl::id.desc())
            .load::<PlasmixProtein>(conn)
            .await
    }

    pub async fn get_all_with_username(
        conn: &mut Connection,
    ) -> QueryResult<Vec<(PlasmixProtein, String)>> {
        plasmix_protein::dsl::plasmix_protein
            .inner_join(users::table.on(plasmix_protein::dsl::user_id.eq(users::dsl::id)))
            .select((plasmix_protein::all_columns, users::dsl::username))
            .order(plasmix_protein::dsl::id.desc())
            .load::<(PlasmixProtein, String)>(conn)
            .await
    }

    pub async fn get_all_by_user_id(
        user_id: i32,
        conn: &mut Connection,
    ) -> QueryResult<Vec<PlasmixProtein>> {
        plasmix_protein::dsl::plasmix_protein
            .filter(plasmix_protein::dsl::user_id.eq(user_id))
            .order(plasmix_protein::dsl::id.desc())
            .load::<PlasmixProtein>(conn)
            .await
    }

    pub async fn find_by_id(protein_id: i32, conn: &mut Connection) -> QueryResult<PlasmixProtein> {
        plasmix_protein::dsl::plasmix_protein
            .find(protein_id)
            .get_result::<PlasmixProtein>(conn)
            .await
    }

    pub async fn add(
        mut item: NewPlasmixProtein,
        user_id: i32,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        item.user_id = Some(user_id);
        diesel::insert_into(plasmix_protein::table)
            .values(item)
            .execute(conn)
            .await
    }

    pub async fn update(
        &self,
        item: Self,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_protein::table.find(self.id))
            .set((
                plasmix_protein::dsl::name.eq(item.name),
                plasmix_protein::dsl::technology.eq(item.technology),
                plasmix_protein::dsl::model_panel.eq(item.model_panel),
                plasmix_protein::dsl::pre_treatment.eq(item.pre_treatment),
                plasmix_protein::dsl::readout.eq(item.readout),
                plasmix_protein::dsl::protocol.eq(item.protocol),
                plasmix_protein::dsl::expression_unit.eq(item.expression_unit),
                plasmix_protein::dsl::analysis_software.eq(item.analysis_software),
                plasmix_protein::dsl::database.eq(item.database),
                plasmix_protein::dsl::normalization.eq(item.normalization),
                plasmix_protein::dsl::determinated_at.eq(item.determinated_at),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_score(
        &self,
        score: (i32, f32, f32, f32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_protein::table.find(self.id))
            .set((
                plasmix_protein::dsl::identified_proteins.eq(score.0),
                plasmix_protein::dsl::recall.eq(score.1),
                plasmix_protein::dsl::snr.eq(score.2),
                plasmix_protein::dsl::rc.eq(score.3),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_samples(
        &self,
        number: (i32, i32, i32, i32, i32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_protein::table.find(self.id))
            .set((
                plasmix_protein::dsl::f.eq(number.0),
                plasmix_protein::dsl::m.eq(number.1),
                plasmix_protein::dsl::p.eq(number.2),
                plasmix_protein::dsl::x.eq(number.3),
                plasmix_protein::dsl::y.eq(number.4),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_report(
        &self,
        report_file: &String,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_protein::table.find(self.id))
            .set(plasmix_protein::dsl::report_file.eq(report_file))
            .execute(conn)
            .await
    }

    pub async fn update_notes(
        protein_id: i32,
        notes: Option<String>,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_protein::table.find(protein_id))
            .set(plasmix_protein::dsl::notes.eq(notes))
            .execute(conn)
            .await
    }
}

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = plasmix_metabolism)]
pub struct PlasmixMetabolism {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub ms_instrument: String,
    pub detection_strategy: String,
    pub analysis_platform: String,
    pub chromatography: String,
    pub ionization: String,
    pub extraction_method: String,
    pub preprocessing_software: String,
    pub normalization: String,
    pub identification_basis: String,
    pub database: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub snr: f32,
    pub pcc: f32,
    pub m: i32,
    pub f: i32,
    pub p: i32,
    pub x: i32,
    pub y: i32,
    pub report_file: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = plasmix_metabolism)]
pub struct NewPlasmixMetabolism {
    pub name: String,
    pub user_id: Option<i32>,
    pub ms_instrument: String,
    pub detection_strategy: String,
    pub analysis_platform: String,
    pub chromatography: String,
    pub ionization: String,
    pub extraction_method: String,
    pub preprocessing_software: String,
    pub normalization: String,
    pub identification_basis: String,
    pub database: String,
    #[serde(deserialize_with = "js_date_to_naive_datetime")]
    pub determinated_at: NaiveDateTime,
    pub meta_file: String,
    pub exp_file: String,
    pub snr: f32,
    pub pcc: f32,
    pub m: i32,
    pub f: i32,
    pub p: i32,
    pub x: i32,
    pub y: i32,
}

impl PlasmixMetabolism {
    pub async fn get_all(conn: &mut Connection) -> QueryResult<Vec<PlasmixMetabolism>> {
        plasmix_metabolism::dsl::plasmix_metabolism
            .order(plasmix_metabolism::dsl::id.desc())
            .load::<PlasmixMetabolism>(conn)
            .await
    }

    pub async fn get_all_with_username(
        conn: &mut Connection,
    ) -> QueryResult<Vec<(PlasmixMetabolism, String)>> {
        plasmix_metabolism::dsl::plasmix_metabolism
            .inner_join(users::table.on(plasmix_metabolism::dsl::user_id.eq(users::dsl::id)))
            .select((plasmix_metabolism::all_columns, users::dsl::username))
            .order(plasmix_metabolism::dsl::id.desc())
            .load::<(PlasmixMetabolism, String)>(conn)
            .await
    }

    pub async fn get_all_by_user_id(
        user_id: i32,
        conn: &mut Connection,
    ) -> QueryResult<Vec<PlasmixMetabolism>> {
        plasmix_metabolism::dsl::plasmix_metabolism
            .filter(plasmix_metabolism::dsl::user_id.eq(user_id))
            .order(plasmix_metabolism::dsl::id.desc())
            .load::<PlasmixMetabolism>(conn)
            .await
    }

    pub async fn find_by_id(
        metabolism_id: i32,
        conn: &mut Connection,
    ) -> QueryResult<PlasmixMetabolism> {
        plasmix_metabolism::dsl::plasmix_metabolism
            .find(metabolism_id)
            .get_result::<PlasmixMetabolism>(conn)
            .await
    }

    pub async fn add(
        mut item: NewPlasmixMetabolism,
        user_id: i32,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        item.user_id = Some(user_id);
        diesel::insert_into(plasmix_metabolism::table)
            .values(item)
            .execute(conn)
            .await
    }

    pub async fn update(
        &self,
        item: Self,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_metabolism::table.find(self.id))
            .set((
                plasmix_metabolism::dsl::name.eq(item.name),
                plasmix_metabolism::dsl::ms_instrument.eq(item.ms_instrument),
                plasmix_metabolism::dsl::detection_strategy.eq(item.detection_strategy),
                plasmix_metabolism::dsl::analysis_platform.eq(item.analysis_platform),
                plasmix_metabolism::dsl::chromatography.eq(item.chromatography),
                plasmix_metabolism::dsl::ionization.eq(item.ionization),
                plasmix_metabolism::dsl::extraction_method.eq(item.extraction_method),
                plasmix_metabolism::dsl::preprocessing_software.eq(item.preprocessing_software),
                plasmix_metabolism::dsl::normalization.eq(item.normalization),
                plasmix_metabolism::dsl::identification_basis.eq(item.identification_basis),
                plasmix_metabolism::dsl::database.eq(item.database),
                plasmix_metabolism::dsl::determinated_at.eq(item.determinated_at),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_score(
        &self,
        score: (f32, f32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_metabolism::table.find(self.id))
            .set((
                plasmix_metabolism::dsl::snr.eq(score.0),
                plasmix_metabolism::dsl::pcc.eq(score.1),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_samples(
        &self,
        number: (i32, i32, i32, i32, i32),
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_metabolism::table.find(self.id))
            .set((
                plasmix_metabolism::dsl::f.eq(number.0),
                plasmix_metabolism::dsl::m.eq(number.1),
                plasmix_metabolism::dsl::p.eq(number.2),
                plasmix_metabolism::dsl::x.eq(number.3),
                plasmix_metabolism::dsl::y.eq(number.4),
            ))
            .execute(conn)
            .await
    }

    pub async fn update_report(
        &self,
        report_file: &String,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_metabolism::table.find(self.id))
            .set(plasmix_metabolism::dsl::report_file.eq(report_file))
            .execute(conn)
            .await
    }

    pub async fn update_notes(
        metabolism_id: i32,
        notes: Option<String>,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::update(plasmix_metabolism::table.find(metabolism_id))
            .set(plasmix_metabolism::dsl::notes.eq(notes))
            .execute(conn)
            .await
    }
}
