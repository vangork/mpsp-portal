use crate::db::Connection;
use crate::schema::config;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, prelude::*};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = config)]
pub struct Config {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = config)]
pub struct NewConfig {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebConfig {
    pub sample_types: String,
    pub test_items: String,
}

impl Config {
    pub async fn get_web_config(conn: &mut Connection) -> QueryResult<WebConfig> {
        let sample_types = config::table
            .filter(config::dsl::key.eq("sample_types"))
            .get_result::<Config>(conn)
            .await
            .optional()?;

        let test_items = config::table
            .filter(config::dsl::key.eq("test_items"))
            .get_result::<Config>(conn)
            .await
            .optional()?;

        Ok(WebConfig {
            sample_types: sample_types.map(|c| c.value).unwrap_or_else(|| "".into()),
            test_items: test_items.map(|c| c.value).unwrap_or_else(|| "".into()),
        })
    }

    pub async fn update_web_config(
        web_config: WebConfig,
        conn: &mut Connection,
    ) -> Result<(), diesel::result::Error> {
        let sample_types_config = NewConfig {
            key: "sample_types".into(),
            value: web_config.sample_types,
        };

        let test_items_config = NewConfig {
            key: "test_items".into(),
            value: web_config.test_items,
        };

        diesel::replace_into(config::table)
            .values(&sample_types_config)
            .execute(conn)
            .await?;

        diesel::replace_into(config::table)
            .values(&test_items_config)
            .execute(conn)
            .await?;
        Ok(())
    }
}
