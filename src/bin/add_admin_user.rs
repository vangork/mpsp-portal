use diesel_async::{AsyncConnection, AsyncMysqlConnection};
use dotenv::dotenv;
use mpsp_portal::models::user::{NewUser, User};
use std::env;
use std::io::stdin;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = AsyncMysqlConnection::establish(&database_url)
        .await
        .expect("database connection");

    let mut email = String::new();
    let mut username = String::new();
    let mut password = String::new();

    println!("What would you like your email to be?");
    stdin().read_line(&mut email).unwrap();
    let email = email.trim().to_lowercase();

    println!("What would you like your username to be?");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("What would you like your password to be?");
    stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let admin = NewUser::admin(username, &email, password);
    User::add_new_user(admin, &mut conn).await.unwrap();
    println!("\nSaved admin user {} with email {}", username, email);
}
