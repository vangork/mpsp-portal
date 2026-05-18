# MSSP Portal

Molecular Phenotyping Sequencing Platform portal built on Vue3.js + Actix Web

## Development

#### Prerequisite

- Node.js
- Rust
- Docker

#### Steps
1. Start mysql and redis service in docker
```docker
docker-compose up -d
```

2. Install the MySQL lib dependency

- Ubuntu
```
sudo apt install libmysqlclient-dev
```
- Windows
Download MySQL Connector/C and set `MYSQLCLIENT_VERSION` and `MYSQLCLIENT_LIB_DIR` environment variables

3. Try to generate a session key
```
cargo run --bin generate_session_key
```

4. Copy .env.example to .env and set all the variables

5. Install diesel cli, download [watchexec](https://github.com/watchexec/watchexec) and init the database
```
cargo install diesel_cli --no-default-features --features "mysql"
diesel setup
```

6. Start the front and backend server in seperated console, navigate to http://localhost:5173/
```
watchexec -e rs -r cargo run
cd web && yarn dev
``` 


## For production

Some tips:
1. Secure MySQL and redis server
```
# generate redis secret key >64
openssl rand -base64 64
```
redis password needs to be url encoded when set in the .env file
https://www.urlencoder.org/

2. To build the source file
```
cargo build --release
cd web && yarn build
```
