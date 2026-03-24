use std::net::{Ipv4Addr, SocketAddr};
use tlog::{env, jwt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port: u16 = env::read_num("PORT", 3000);
    let jwt_secret_key: String = env::read_string("JWT_SECRET_KEY", "secret##tlog##secret");
    // let db_passphr: &str = &env::read_str("DB_PASSPHR", "12345678");

    tracing_subscriber::fmt::try_init()?;
    jwt::init(jwt_secret_key);
    // db::init(db_passphr);

    let app = tlog::routes::router();
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Server listening on port {}", port);
    axum::serve(listener, app).await?;

    Ok(())
}
