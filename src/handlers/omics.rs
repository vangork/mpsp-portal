use super::auth::get_user_id;
use crate::aliyun::OssClient;
use crate::db::Pool;
use crate::error::ServiceError;
use crate::models::omics::{
    DNA, DNAScore, Metabolism, NewDNA, NewMetabolism, NewPlasmixMetabolism, NewPlasmixProtein,
    NewProtein, NewRNA, PlasmixMetabolism, PlasmixProtein, Protein, RNA,
};
use crate::models::user::User;
use actix_session::Session;
use actix_web::{
    HttpResponse, Result,
    web::{Data, Json, Path},
};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::{fs, io};

pub async fn oss_token() -> Result<HttpResponse> {
    let client = OssClient::new();
    Ok(HttpResponse::Ok().json(client))
}

// dna
pub async fn list_dna(pool: Data<Pool>, session: Session) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    let items = DNA::get_all_by_user_id(user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn add_dna(
    Json(item): Json<NewDNA>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    let dna_id = DNA::add(item, user_id, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    let _item = DNA::find_by_id(dna_id, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    Ok(HttpResponse::Ok().json("dna added"))
}

pub async fn update_dna(
    dna_id: Path<i32>,
    Json(item): Json<DNA>,
    session: Session,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let dna_id = dna_id.into_inner();
    let dna_item = DNA::find_by_id(dna_id, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    if dna_item.user_id != get_user_id(&session) {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    dna_item
        .update(item, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("dna updated"))
}

pub async fn generate_dna_report(
    Json(items): Json<Vec<DNA>>,
    _pool: Data<Pool>,
) -> Result<HttpResponse> {
    let oss_client = OssClient::new();
    let report_file =
        run_dna_qc(&items, &oss_client)
            .await
            .map_err(|e| ServiceError::InternalServerError {
                error_message: e.to_string(),
            })?;
    let download_url = oss_client.signature_download_file(report_file);
    Ok(HttpResponse::Ok().json(download_url))
}

pub async fn run_dna_qc(_: &[DNA], _: &OssClient) -> io::Result<String> {
    Ok("".to_string())
}

pub async fn run_dna_qc_1(_items: &[DNA], _name: &str, _dest: &str) -> io::Result<()> {
    Ok(())
}

// rna
pub async fn list_rna(pool: Data<Pool>, session: Session) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    let items = RNA::get_all_by_user_id(user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn add_rna(
    Json(item): Json<NewRNA>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    RNA::add(item, user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("rna added"))
}

pub async fn update_rna(
    rna_id: Path<i32>,
    Json(item): Json<RNA>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let rna_id = rna_id.into_inner();
    let rna_item = RNA::find_by_id(rna_id, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    if rna_item.user_id != get_user_id(&session) {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    rna_item
        .update(item, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("rna updated"))
}

pub async fn get_rna_report(
    rna_id: Path<i32>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let rna_id = rna_id.into_inner();
    let item = RNA::find_by_id(rna_id, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    let user_id = get_user_id(&session);
    if item.user_id != user_id && !User::is_active_admin(user_id, &mut conn).await {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    let oss_client = OssClient::new();
    let download_file = if let Some(ref report_file) = item.report_file
        && !report_file.is_empty()
    {
        report_file.clone()
    } else {
        let report_file = run_rna_qc(&item, &oss_client).await.map_err(|e| {
            ServiceError::InternalServerError {
                error_message: e.to_string(),
            }
        })?;
        item.update_report(&report_file, &mut conn)
            .await
            .map_err(|e| ServiceError::InternalServerError {
                error_message: e.to_string(),
            })?;
        report_file
    };
    let download_url = oss_client.signature_download_file(download_file);
    Ok(HttpResponse::Ok().json(download_url))
}

pub async fn run_rna_qc(_: &RNA, _: &OssClient) -> io::Result<String> {
    Ok("".to_string())
}

pub async fn run_rna_qc_score(_item: &RNA, _client: &OssClient) -> io::Result<(f32, f32)> {
    Ok((0.0, 0.0))
}

pub async fn get_rna_qc_samples(
    item: &RNA,
    client: &OssClient,
) -> io::Result<(i32, i32, i32, i32)> {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::other("epoch error"))?;

    let timestamp_nanos = duration_since_epoch.as_nanos();
    let dir_path = format!("/tmp/omis-portal-rna-{}", timestamp_nanos);
    let meta_file = format!("{}/meta_file.csv", dir_path);
    fs::create_dir_all(&dir_path).await?;
    client.download_file(&item.meta_file, &meta_file).await?;
    let mut rdr = csv::Reader::from_path(&meta_file)?;
    let mut samples = Vec::<String>::new();
    for result in rdr.deserialize() {
        let record: Result<HashMap<String, String>, csv::Error> = result;
        match record {
            Ok(record) => {
                record.iter().for_each(|(k, v)| {
                    if k.to_lowercase() == "sample" {
                        samples.push(v.trim().to_lowercase());
                    }
                });
            }
            Err(e) => println!("Failed to parse metadata record, error: {}", e),
        }
    }
    fs::remove_dir_all(&dir_path).await?;
    Ok((
        samples.iter().filter(|s| *s == "d5").count() as i32,
        samples.iter().filter(|s| *s == "d6").count() as i32,
        samples.iter().filter(|s| *s == "f7").count() as i32,
        samples.iter().filter(|s| *s == "m8").count() as i32,
    ))
}

// protein
pub async fn list_protein(pool: Data<Pool>, session: Session) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    let items = Protein::get_all_by_user_id(user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn add_protein(
    Json(item): Json<NewProtein>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    Protein::add(item, user_id, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    Ok(HttpResponse::Ok().json("protein added"))
}

pub async fn update_protein(
    protein_id: Path<i32>,
    Json(item): Json<Protein>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let protein_id = protein_id.into_inner();
    let protein_item = Protein::find_by_id(protein_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    if protein_item.user_id != get_user_id(&session) {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    protein_item
        .update(item, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("protein updated"))
}

pub async fn get_protein_report(
    protein_id: Path<i32>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let protein_id = protein_id.into_inner();
    let item = Protein::find_by_id(protein_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    if item.user_id != user_id && !User::is_active_admin(user_id, &mut conn).await {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    let oss_client = OssClient::new();
    let download_file = if let Some(ref report_file) = item.report_file
        && !report_file.is_empty()
    {
        report_file.clone()
    } else {
        let report_file = run_protein_qc(&item, &oss_client).await.map_err(|e| {
            ServiceError::InternalServerError {
                error_message: e.to_string(),
            }
        })?;
        item.update_report(&report_file, &mut conn)
            .await
            .map_err(|e| ServiceError::InternalServerError {
                error_message: e.to_string(),
            })?;
        report_file
    };
    let download_url = oss_client.signature_download_file(download_file);
    Ok(HttpResponse::Ok().json(download_url))
}

pub async fn run_protein_qc(_item: &Protein, _client: &OssClient) -> io::Result<String> {
    Ok("".to_string())
}

pub async fn run_protein_qc_score(
    _item: &Protein,
    _client: &OssClient,
) -> io::Result<(i32, f32, f32, f32)> {
    Ok((0, 0.0, 0.0, 0.0))
}

pub async fn get_protein_qc_samples(
    item: &Protein,
    client: &OssClient,
) -> io::Result<(i32, i32, i32, i32)> {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::other("epoch error"))?;

    let timestamp_nanos = duration_since_epoch.as_nanos();
    let dir_path = format!("/tmp/omis-portal-protein-{}", timestamp_nanos);
    let meta_file = format!("{}/meta_file.csv", dir_path);
    fs::create_dir_all(&dir_path).await?;
    client.download_file(&item.meta_file, &meta_file).await?;
    let mut rdr = csv::Reader::from_path(&meta_file)?;
    let mut samples = Vec::<String>::new();
    for result in rdr.deserialize() {
        let record: Result<HashMap<String, String>, csv::Error> = result;
        match record {
            Ok(record) => {
                record.iter().for_each(|(k, v)| {
                    if k.to_lowercase() == "sample" {
                        samples.push(v.trim().to_lowercase());
                    }
                });
            }
            Err(e) => println!("Failed to parse metadata record, error: {}", e),
        }
    }
    fs::remove_dir_all(&dir_path).await?;
    Ok((
        samples.iter().filter(|s| *s == "d5").count() as i32,
        samples.iter().filter(|s| *s == "d6").count() as i32,
        samples.iter().filter(|s| *s == "f7").count() as i32,
        samples.iter().filter(|s| *s == "m8").count() as i32,
    ))
}

// metabolism
pub async fn list_metabolism(pool: Data<Pool>, session: Session) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    let items = Metabolism::get_all_by_user_id(user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn add_metabolism(
    Json(item): Json<NewMetabolism>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    Metabolism::add(item, user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("metabolism added"))
}

pub async fn update_metabolism(
    metabolism_id: Path<i32>,
    Json(item): Json<Metabolism>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let metabolism_id = metabolism_id.into_inner();
    let metabolism_item = Metabolism::find_by_id(metabolism_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    if metabolism_item.user_id != get_user_id(&session) {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    metabolism_item.update(item, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    Ok(HttpResponse::Ok().json("metabolism updated"))
}

pub async fn get_metabolism_report(
    metabolism_id: Path<i32>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let metabolism_id = metabolism_id.into_inner();
    let item = Metabolism::find_by_id(metabolism_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    if item.user_id != user_id && !User::is_active_admin(user_id, &mut conn).await {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    let oss_client = OssClient::new();
    let download_file = if let Some(ref report_file) = item.report_file
        && !report_file.is_empty()
    {
        report_file.clone()
    } else {
        let report_file = run_metabolism_qc(&item, &oss_client).await.map_err(|e| {
            ServiceError::InternalServerError {
                error_message: e.to_string(),
            }
        })?;
        item.update_report(&report_file, &mut conn)
            .await
            .map_err(|e| ServiceError::InternalServerError {
                error_message: e.to_string(),
            })?;
        report_file
    };
    let download_url = oss_client.signature_download_file(download_file);
    Ok(HttpResponse::Ok().json(download_url))
}

pub async fn run_metabolism_qc(_item: &Metabolism, _client: &OssClient) -> io::Result<String> {
    Ok("".to_string())
}

pub async fn run_metabolism_qc_score(
    _item: &Metabolism,
    _client: &OssClient,
) -> io::Result<(f32, f32)> {
    Ok((0.0, 0.0))
}

pub async fn get_metabolism_qc_samples(
    item: &Metabolism,
    client: &OssClient,
) -> io::Result<(i32, i32, i32, i32)> {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::other("epoch error"))?;

    let timestamp_nanos = duration_since_epoch.as_nanos();
    let dir_path = format!("/tmp/omis-portal-metabolism-{}", timestamp_nanos);
    let meta_file = format!("{}/meta_file.csv", dir_path);
    fs::create_dir_all(&dir_path).await?;
    client.download_file(&item.meta_file, &meta_file).await?;
    let mut rdr = csv::Reader::from_path(&meta_file)?;
    let mut samples = Vec::<String>::new();
    for result in rdr.deserialize() {
        let record: Result<HashMap<String, String>, csv::Error> = result;
        match record {
            Ok(record) => {
                record.iter().for_each(|(k, v)| {
                    if k.to_lowercase() == "sample" {
                        samples.push(v.trim().to_lowercase());
                    }
                });
            }
            Err(e) => println!("Failed to parse metadata record, error: {}", e),
        }
    }
    fs::remove_dir_all(&dir_path).await?;
    Ok((
        samples.iter().filter(|s| *s == "d5").count() as i32,
        samples.iter().filter(|s| *s == "d6").count() as i32,
        samples.iter().filter(|s| *s == "f7").count() as i32,
        samples.iter().filter(|s| *s == "m8").count() as i32,
    ))
}

// plasmix protein
pub async fn list_plasmix_protein(pool: Data<Pool>, session: Session) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    let items = PlasmixProtein::get_all_by_user_id(user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn add_plasmix_protein(
    Json(item): Json<NewPlasmixProtein>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    PlasmixProtein::add(item, user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("plasmix protein added"))
}

pub async fn update_plasmix_protein(
    plasmix_protein_id: Path<i32>,
    Json(item): Json<PlasmixProtein>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let plasmix_protein_id = plasmix_protein_id.into_inner();
    let plasmix_protein_item = PlasmixProtein::find_by_id(plasmix_protein_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    if plasmix_protein_item.user_id != get_user_id(&session) {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    plasmix_protein_item
        .update(item, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("plasmix protein updated"))
}

pub async fn get_plasmix_protein_report(
    plasmix_protein_id: Path<i32>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let plasmix_protein_id = plasmix_protein_id.into_inner();
    let item = PlasmixProtein::find_by_id(plasmix_protein_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    if item.user_id != user_id && !User::is_active_admin(user_id, &mut conn).await {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    let oss_client = OssClient::new();
    let download_file = if let Some(ref report_file) = item.report_file
        && !report_file.is_empty()
    {
        report_file.clone()
    } else {
        let report_file = run_plasmix_protein_qc(&item, &oss_client)
            .await
            .map_err(|e| ServiceError::InternalServerError {
                error_message: e.to_string(),
            })?;
        item.update_report(&report_file, &mut conn)
            .await
            .map_err(|e| ServiceError::InternalServerError {
                error_message: e.to_string(),
            })?;
        report_file
    };
    let download_url = oss_client.signature_download_file(download_file);
    Ok(HttpResponse::Ok().json(download_url))
}

pub async fn run_plasmix_protein_qc(
    _item: &PlasmixProtein,
    _client: &OssClient,
) -> io::Result<String> {
    Ok("".to_string())
}

pub async fn run_plasmix_protein_qc_score(
    _item: &PlasmixProtein,
    _client: &OssClient,
) -> io::Result<(i32, f32, f32, f32)> {
    Ok((0, 0.0, 0.0, 0.0))
}

pub async fn get_plasmix_protein_qc_samples(
    item: &PlasmixProtein,
    client: &OssClient,
) -> io::Result<(i32, i32, i32, i32, i32)> {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::other("epoch error"))?;

    let timestamp_nanos = duration_since_epoch.as_nanos();
    let dir_path = format!("/tmp/omis-portal-plasmix-protein-{}", timestamp_nanos);
    let meta_file = format!("{}/meta_file.csv", dir_path);
    fs::create_dir_all(&dir_path).await?;
    client.download_file(&item.meta_file, &meta_file).await?;
    let mut rdr = csv::Reader::from_path(&meta_file)?;
    let mut samples = Vec::<String>::new();
    for result in rdr.deserialize() {
        let record: Result<HashMap<String, String>, csv::Error> = result;
        match record {
            Ok(record) => {
                record.iter().for_each(|(k, v)| {
                    if k.to_lowercase() == "sample" {
                        samples.push(v.trim().to_lowercase());
                    }
                });
            }
            Err(e) => println!("Failed to parse metadata record, error: {}", e),
        }
    }
    fs::remove_dir_all(&dir_path).await?;
    Ok((
        samples.iter().filter(|s| *s == "f").count() as i32,
        samples.iter().filter(|s| *s == "m").count() as i32,
        samples.iter().filter(|s| *s == "p").count() as i32,
        samples.iter().filter(|s| *s == "x").count() as i32,
        samples.iter().filter(|s| *s == "y").count() as i32,
    ))
}

// plasmix metabolism
pub async fn list_plasmix_metabolism(pool: Data<Pool>, session: Session) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    let items = PlasmixMetabolism::get_all_by_user_id(user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn add_plasmix_metabolism(
    Json(item): Json<NewPlasmixMetabolism>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    PlasmixMetabolism::add(item, user_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("plasmix metabolism added"))
}

pub async fn update_plasmix_metabolism(
    metabolism_id: Path<i32>,
    Json(item): Json<PlasmixMetabolism>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let metabolism_id = metabolism_id.into_inner();
    let metabolism_item = PlasmixMetabolism::find_by_id(metabolism_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    if metabolism_item.user_id != get_user_id(&session) {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    metabolism_item.update(item, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    Ok(HttpResponse::Ok().json("plasmix metabolism updated"))
}

pub async fn get_plasmix_metabolism_report(
    metabolism_id: Path<i32>,
    pool: Data<Pool>,
    session: Session,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let metabolism_id = metabolism_id.into_inner();
    let item = PlasmixMetabolism::find_by_id(metabolism_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let user_id = get_user_id(&session);
    if item.user_id != user_id && !User::is_active_admin(user_id, &mut conn).await {
        return Err(ServiceError::Unauthorized {
            error_message: "insufficient permission".to_string(),
        }
        .into());
    }
    let oss_client = OssClient::new();
    let download_file = if let Some(ref report_file) = item.report_file
        && !report_file.is_empty()
    {
        report_file.clone()
    } else {
        let report_file = run_plasmix_metabolism_qc(&item, &oss_client)
            .await
            .map_err(|e| ServiceError::InternalServerError {
                error_message: e.to_string(),
            })?;
        item.update_report(&report_file, &mut conn)
            .await
            .map_err(|e| ServiceError::InternalServerError {
                error_message: e.to_string(),
            })?;
        report_file
    };
    let download_url = oss_client.signature_download_file(download_file);
    Ok(HttpResponse::Ok().json(download_url))
}

pub async fn run_plasmix_metabolism_qc(
    _item: &PlasmixMetabolism,
    _client: &OssClient,
) -> io::Result<String> {
    Ok("".to_string())
}

pub async fn run_plasmix_metabolism_qc_score(
    _item: &PlasmixMetabolism,
    _client: &OssClient,
) -> io::Result<(f32, f32)> {
    Ok((0.0, 0.0))
}

pub async fn get_plasmix_metabolism_qc_samples(
    item: &PlasmixMetabolism,
    client: &OssClient,
) -> io::Result<(i32, i32, i32, i32, i32)> {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io::Error::other("epoch error"))?;

    let timestamp_nanos = duration_since_epoch.as_nanos();
    let dir_path = format!("/tmp/omis-portal-plasmix-metabolism-{}", timestamp_nanos);
    let meta_file = format!("{}/meta_file.csv", dir_path);
    fs::create_dir_all(&dir_path).await?;
    client.download_file(&item.meta_file, &meta_file).await?;
    let mut rdr = csv::Reader::from_path(&meta_file)?;
    let mut samples = Vec::<String>::new();
    for result in rdr.deserialize() {
        let record: Result<HashMap<String, String>, csv::Error> = result;
        match record {
            Ok(record) => {
                record.iter().for_each(|(k, v)| {
                    if k.to_lowercase() == "sample" {
                        samples.push(v.trim().to_lowercase());
                    }
                });
            }
            Err(e) => println!("Failed to parse metadata record, error: {}", e),
        }
    }
    fs::remove_dir_all(&dir_path).await?;
    Ok((
        samples.iter().filter(|s| *s == "f").count() as i32,
        samples.iter().filter(|s| *s == "m").count() as i32,
        samples.iter().filter(|s| *s == "p").count() as i32,
        samples.iter().filter(|s| *s == "x").count() as i32,
        samples.iter().filter(|s| *s == "y").count() as i32,
    ))
}

// admin dna
pub async fn list_dna_with_username(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = DNA::get_all_with_username(&mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    let items = items
        .into_iter()
        .map(|i| {
            let mut values = serde_json::to_value(i.0).unwrap();
            let map = values.as_object_mut().unwrap();
            map.insert("username".to_string(), serde_json::json!(i.1));
            values
        })
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(items))
}

pub async fn signature_dna_vcf(dna_id: Path<i32>, pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let dna_id = dna_id.into_inner();
    let item = DNA::find_by_id(dna_id, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    let oss_client = OssClient::new();
    let download_url = oss_client.signature_download_raw_file(item.vcf_file);
    Ok(HttpResponse::Ok().json(download_url))
}

pub async fn update_dna_score(
    dna_id: Path<i32>,
    Json(score): Json<DNAScore>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let dna_id = dna_id.into_inner();
    DNA::update_score(dna_id, score, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("dna score updated"))
}

// admin rna
pub async fn list_rna_with_username(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = RNA::get_all_with_username(&mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    let items = items
        .into_iter()
        .map(|i| {
            let mut values = serde_json::to_value(i.0).unwrap();
            let map = values.as_object_mut().unwrap();
            map.insert("username".to_string(), serde_json::json!(i.1));
            values
        })
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(items))
}

pub async fn get_rna_data_signature(rna_id: Path<i32>, pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let rna_id = rna_id.into_inner();
    let item = RNA::find_by_id(rna_id, &mut conn).await.map_err(|e| {
        ServiceError::InternalServerError {
            error_message: e.to_string(),
        }
    })?;
    let oss_client = OssClient::new();
    let meta_file_url = oss_client.signature_download_raw_file(item.meta_file);
    let exp_file_url = oss_client.signature_download_raw_file(item.exp_file);
    let count_file_url = oss_client.signature_download_raw_file(item.count_file);
    let payload = serde_json::json!({
        "meta_file": meta_file_url,
        "exp_file": exp_file_url,
        "count_file": count_file_url
    });
    Ok(HttpResponse::Ok().json(payload))
}

pub async fn update_rna_notes(
    rna_id: Path<i32>,
    Json(notes): Json<Option<String>>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let rna_id = rna_id.into_inner();
    RNA::update_notes(rna_id, notes, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("rna notes updated"))
}

// admin protein
pub async fn list_protein_with_username(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = Protein::get_all_with_username(&mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = items
        .into_iter()
        .map(|i| {
            let mut values = serde_json::to_value(i.0).unwrap();
            let map = values.as_object_mut().unwrap();
            map.insert("username".to_string(), serde_json::json!(i.1));
            values
        })
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(items))
}

pub async fn get_protein_data_signature(
    protein_id: Path<i32>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let protein_id = protein_id.into_inner();
    let item = Protein::find_by_id(protein_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let oss_client = OssClient::new();
    let meta_file_url = oss_client.signature_download_raw_file(item.meta_file);
    let exp_file_url = oss_client.signature_download_raw_file(item.exp_file);
    let payload = serde_json::json!({
        "meta_file": meta_file_url,
        "exp_file": exp_file_url,
    });
    Ok(HttpResponse::Ok().json(payload))
}

pub async fn update_protein_notes(
    protein_id: Path<i32>,
    Json(notes): Json<Option<String>>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let protein_id = protein_id.into_inner();
    Protein::update_notes(protein_id, notes, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("protein notes updated"))
}

// admin metabolism
pub async fn list_metabolism_with_username(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = Metabolism::get_all_with_username(&mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = items
        .into_iter()
        .map(|i| {
            let mut values = serde_json::to_value(i.0).unwrap();
            let map = values.as_object_mut().unwrap();
            map.insert("username".to_string(), serde_json::json!(i.1));
            values
        })
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(items))
}

pub async fn get_metabolism_data_signature(
    metabolism_id: Path<i32>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let metabolism_id = metabolism_id.into_inner();
    let item = Metabolism::find_by_id(metabolism_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let oss_client = OssClient::new();
    let meta_file_url = oss_client.signature_download_raw_file(item.meta_file);
    let exp_file_url = oss_client.signature_download_raw_file(item.exp_file);
    let payload = serde_json::json!({
        "meta_file": meta_file_url,
        "exp_file": exp_file_url,
    });
    Ok(HttpResponse::Ok().json(payload))
}

pub async fn update_metabolism_notes(
    metabolism_id: Path<i32>,
    Json(notes): Json<Option<String>>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let metabolism_id = metabolism_id.into_inner();
    Metabolism::update_notes(metabolism_id, notes, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("metabolism notes updated"))
}

// admin plasmix protein
pub async fn list_plasmix_protein_with_username(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = PlasmixProtein::get_all_with_username(&mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = items
        .into_iter()
        .map(|i| {
            let mut values = serde_json::to_value(i.0).unwrap();
            let map = values.as_object_mut().unwrap();
            map.insert("username".to_string(), serde_json::json!(i.1));
            values
        })
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(items))
}

pub async fn get_plasmix_protein_data_signature(
    protein_id: Path<i32>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let protein_id = protein_id.into_inner();
    let item = PlasmixProtein::find_by_id(protein_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let oss_client = OssClient::new();
    let meta_file_url = oss_client.signature_download_raw_file(item.meta_file);
    let exp_file_url = oss_client.signature_download_raw_file(item.exp_file);
    let payload = serde_json::json!({
        "meta_file": meta_file_url,
        "exp_file": exp_file_url,
    });
    Ok(HttpResponse::Ok().json(payload))
}

pub async fn update_plasmix_protein_notes(
    protein_id: Path<i32>,
    Json(notes): Json<Option<String>>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let protein_id = protein_id.into_inner();
    PlasmixProtein::update_notes(protein_id, notes, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("plasmix protein notes updated"))
}

// admin plasmix metabolism
pub async fn list_plasmix_metabolism_with_username(pool: Data<Pool>) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = PlasmixMetabolism::get_all_with_username(&mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let items = items
        .into_iter()
        .map(|i| {
            let mut values = serde_json::to_value(i.0).unwrap();
            let map = values.as_object_mut().unwrap();
            map.insert("username".to_string(), serde_json::json!(i.1));
            values
        })
        .collect::<Vec<_>>();
    Ok(HttpResponse::Ok().json(items))
}

pub async fn get_plasmix_metabolism_data_signature(
    metabolism_id: Path<i32>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let metabolism_id = metabolism_id.into_inner();
    let item = PlasmixMetabolism::find_by_id(metabolism_id, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let oss_client = OssClient::new();
    let meta_file_url = oss_client.signature_download_raw_file(item.meta_file);
    let exp_file_url = oss_client.signature_download_raw_file(item.exp_file);
    let payload = serde_json::json!({
        "meta_file": meta_file_url,
        "exp_file": exp_file_url,
    });
    Ok(HttpResponse::Ok().json(payload))
}

pub async fn update_plasmix_metabolism_notes(
    metabolism_id: Path<i32>,
    Json(notes): Json<Option<String>>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    let mut conn = pool
        .get()
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    let metabolism_id = metabolism_id.into_inner();
    PlasmixMetabolism::update_notes(metabolism_id, notes, &mut conn)
        .await
        .map_err(|e| ServiceError::InternalServerError {
            error_message: e.to_string(),
        })?;
    Ok(HttpResponse::Ok().json("plasmix metabolism notes updated"))
}
