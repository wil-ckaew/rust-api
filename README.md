Sistem em Rust backend api documents
//comando arquivo para executar
cargo new rust-api
cd rust-api
touch src/services.rs
curl --request GET \ --url http://localhost:8080/api/healthchecker \ --reader 'Content-type: application/json'
touch .env docker-compose.yml src/model.rs src/schema.rs
cargo install sqlx-cli
sqlx migrate add -r init
sqlx migrate run
sqlx migrate revert


cargo watch -q -c -w src/ -x run
//executar docke
docker-compose up -d

sudo systemctl restart docker
sudo systemctl daemon-reload
 docker-compose --version
 sudo curl -L "https://github.com/docker/compose/releases/download/{latest_version}/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
 sudo systemctl restart docker
 docker-compose up -d

 cargo watch -q -c -w src/ -x run
