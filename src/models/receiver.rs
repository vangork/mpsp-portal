use crate::db::Connection;
use crate::schema::receivers;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, prelude::*};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = receivers)]
pub struct Receiver {
    pub id: i32,
    pub name: String,
    #[serde(rename = "institution")]
    pub address: String,
    pub phone: String,
    pub email: String,
    pub default: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = receivers)]
pub struct NewReceiver {
    pub name: String,
    #[serde(rename = "institution")]
    pub address: String,
    pub phone: String,
    pub email: String,
    pub default: bool,
}

impl NewReceiver {
    pub fn new(name: &str, address: &str, phone: &str, email: &str, default: bool) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
            phone: phone.to_string(),
            email: email.to_ascii_lowercase(),
            default: default,
        }
    }
}

impl Receiver {
    pub async fn add_new_receiver(
        receiver: NewReceiver,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        diesel::insert_into(receivers::table)
            .values(receiver)
            .execute(conn)
            .await
    }

    pub async fn find_receiver_by_id(receiver_id: i32, conn: &mut Connection) -> QueryResult<Receiver> {
        receivers::table.find(receiver_id).get_result::<Receiver>(conn).await
    }

    pub async fn update_receiver(
        receiver_id: i32,
        receiver: NewReceiver,
        conn: &mut Connection,
    ) -> Result<(), diesel::result::Error> {
        if receiver.default {
            // If the updated receiver is set to default, unset the default flag for all other receivers
            diesel::update(receivers::table.filter(receivers::dsl::id.ne(receiver_id)))
                .set(receivers::dsl::default.eq(false))
                .execute(conn)
                .await?;
        }
        diesel::update(receivers::table.find(receiver_id))
            .set((
                receivers::dsl::name.eq(receiver.name),
                receivers::dsl::address.eq(receiver.address),
                receivers::dsl::phone.eq(receiver.phone),
                receivers::dsl::email.eq(receiver.email),
                receivers::dsl::default.eq(receiver.default),
            ))
            .execute(conn)
            .await?;
        Ok(())
    }

    pub async fn get_all(conn: &mut Connection) -> QueryResult<Vec<Receiver>> {
        receivers::table.load::<Receiver>(conn).await
    }

    pub async fn get_default_receiver(conn: &mut Connection) -> QueryResult<Option<Receiver>> {
        let receiver = receivers::table
            .filter(receivers::dsl::default.eq(true))
            .get_result::<Receiver>(conn)
            .await
            .optional()?;
        if let Some(r) = receiver {
            return Ok(Some(r));
        }
        // If no default receiver is set, return the first one
        let receiver = receivers::table.first::<Receiver>(conn).await.optional()?;
        Ok(receiver)
    }

    pub async fn delete_receiver(receiver_id: i32, conn: &mut Connection) -> Result<(), diesel::result::Error> {
        diesel::delete(receivers::table.find(receiver_id)).execute(conn).await?;
        Ok(())
    }
}
