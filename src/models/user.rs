use crate::db::Connection;
use crate::handlers::auth::Credential;
use crate::schema::users;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, prelude::*};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: i32,
    pub active: bool,
    pub notes: String,
    pub last_seen: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: i32,
    pub active: bool,
    pub notes: String,
}

impl NewUser {
    pub fn admin(username: &str, email: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_ascii_lowercase(),
            password: password.to_string(),
            role: 2,
            active: true,
            notes: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewPassword {
    pub current_password: String,
    pub new_password: String,
}

fn encrypt_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

impl User {
    pub async fn login(
        cred: Credential,
        conn: &mut Connection,
    ) -> Result<Option<User>, diesel::result::Error> {
        let user = users::table
            .filter(users::dsl::email.eq(cred.email.to_ascii_lowercase()))
            .filter(users::dsl::active.eq(true))
            .get_result::<User>(conn)
            .await
            .optional()?;
        if let Some(user) = user
            && let Ok(val) = verify(&cred.password, &user.password)
            && val
        {
            diesel::update(users::table.find(user.id))
                .set(users::dsl::last_seen.eq(diesel::dsl::now))
                .execute(conn)
                .await?;
            return Ok(Some(user));
        }
        Ok(None)
    }

    pub async fn is_active(user_id: i32, conn: &mut Connection) -> bool {
        users::table
            .find(user_id)
            .get_result::<User>(conn)
            .await
            .map(|user| user.active)
            .unwrap_or(false)
    }

    pub async fn is_active_admin(user_id: i32, conn: &mut Connection) -> bool {
        users::table
            .find(user_id)
            .get_result::<User>(conn)
            .await
            .map(|user| user.role == 2 && user.active)
            .unwrap_or(false)
    }

    pub async fn find_user_by_id(user_id: i32, conn: &mut Connection) -> QueryResult<User> {
        users::table.find(user_id).get_result::<User>(conn).await
    }

    pub async fn add_new_user(
        user: NewUser,
        conn: &mut Connection,
    ) -> Result<usize, diesel::result::Error> {
        let new_user = NewUser {
            email: user.email.to_ascii_lowercase(),
            username: user.username,
            password: encrypt_password(&user.password),
            role: user.role,
            active: user.active,
            notes: user.notes,
        };
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(conn)
            .await
    }

    pub async fn update_password(
        user_id: i32,
        password: NewPassword,
        conn: &mut Connection,
    ) -> Result<bool, diesel::result::Error> {
        let user = users::table.find(user_id).get_result::<User>(conn).await?;
        if let Ok(val) = verify(&password.current_password, &user.password)
            && val
        {
            diesel::update(users::table.find(user.id))
                .set(users::dsl::password.eq(encrypt_password(&password.new_password)))
                .execute(conn)
                .await?;
            return Ok(true);
        }
        Ok(false)
    }

    pub async fn update_user(
        user_id: i32,
        user: NewUser,
        conn: &mut Connection,
    ) -> Result<(), diesel::result::Error> {
        if user.password.is_empty() {
            diesel::update(users::table.find(user_id))
                .set((
                    users::dsl::username.eq(user.username),
                    users::dsl::email.eq(user.email.to_ascii_lowercase()),
                    users::dsl::role.eq(user.role),
                    users::dsl::active.eq(user.active),
                    users::dsl::notes.eq(user.notes),
                ))
                .execute(conn)
                .await?;
        } else {
            diesel::update(users::table.find(user_id))
                .set((
                    users::dsl::username.eq(user.username),
                    users::dsl::email.eq(user.email.to_ascii_lowercase()),
                    users::dsl::password.eq(encrypt_password(&user.password)),
                    users::dsl::role.eq(user.role),
                    users::dsl::active.eq(user.active),
                    users::dsl::notes.eq(user.notes),
                ))
                .execute(conn)
                .await?;
        }
        Ok(())
    }

    pub async fn get_all(conn: &mut Connection) -> QueryResult<Vec<User>> {
        users::table.load::<User>(conn).await
    }
}
