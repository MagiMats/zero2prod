use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read config");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Could not find an open port");
    startup::run(listener, connection)?.await
}
