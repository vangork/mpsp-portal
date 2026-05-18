use aliyun_oss_rust_sdk::oss::OSS;
use aliyun_oss_rust_sdk::request::RequestBuilder;
use aliyun_oss_rust_sdk::url::UrlApi;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::{fs, io};

#[derive(Deserialize, Serialize)]
pub struct OssClient {
    pub region: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    pub bucket: String,
}

impl OssClient {
    pub fn new() -> Self {
        let access_key_id =
            env::var("ALIYUN_ACCESS_KEY_ID").expect("ALIYUN_ACCESS_KEY_ID is not set in .env file");
        let access_key_secret = env::var("ALIYUN_ACCESS_KEY_SECRET")
            .expect("ALIYUN_ACCESS_KEY_SECRET is not set in .env file");
        let bucket =
            env::var("ALIYUN_OSS_BUCKET").expect("ALIYUN_OSS_BUCKET is not set in .env file");
        let region =
            env::var("ALIYUN_OSS_REGION").expect("ALIYUN_OSS_REGION is not set in .env file");

        Self {
            region,
            access_key_id,
            access_key_secret,
            bucket,
        }
    }

    pub fn signature_download_raw_file(&self, file: String) -> String {
        let client = OSS::new(
            &self.access_key_id,
            &self.access_key_secret,
            &format!("{}.aliyuncs.com", &self.region),
            &self.bucket,
        );
        let builder = RequestBuilder::new().with_expire(3600 * 24);
        client.sign_download_url(file, &builder)
    }

    pub fn signature_download_file(&self, file: String) -> String {
        let client = OSS::new(
            &self.access_key_id,
            &self.access_key_secret,
            &format!("{}.aliyuncs.com", &self.region),
            &self.bucket,
        );
        let builder = RequestBuilder::new()
            .with_expire(60)
            .oss_download_speed_limit(100);
        client.sign_download_url(file, &builder)
    }

    pub async fn download_file(&self, file: &str, local_path: &str) -> io::Result<()> {
        let client = OSS::new(
            &self.access_key_id,
            &self.access_key_secret,
            &format!("{}.aliyuncs.com", &self.region),
            &self.bucket,
        );
        let builder = RequestBuilder::new();
        let bytes = client
            .get_object(file, builder)
            .await
            .map_err(|e| io::Error::other(format!("unable to download {}: {}", file, e)))?;
        fs::write(local_path, &bytes).await
    }

    pub async fn upload_file(&self, file: &str, local_path: &str) -> io::Result<()> {
        let client = OSS::new(
            &self.access_key_id,
            &self.access_key_secret,
            &format!("{}.aliyuncs.com", &self.region),
            &self.bucket,
        );
        let builder = RequestBuilder::new();
        client
            .put_object_from_file(file, local_path, builder)
            .await
            .map_err(|e| io::Error::other(format!("unable to upload {}: {}", file, e)))
    }
}

impl Default for OssClient {
    fn default() -> Self {
        Self::new()
    }
}
