use actix_web::{App, HttpServer};
use listenfd::ListenFd;
use tracing_actix_web::TracingLogger;

use meetup_facade::init::{app_config, telemetry_config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    telemetry_config::init("meetup_facade");

    let app_factory = || {
        App::new()
            .wrap(TracingLogger::default())
            .configure(app_config::config)
    };
    let server = if let Some(listener) = ListenFd::from_env().take_tcp_listener(0)? {
        HttpServer::new(app_factory).listen(listener)?
    } else {
        HttpServer::new(app_factory).bind(get_configured_address())?
    };

    tracing::info!("Starting app, listening on {:?}", server.addrs());
    server.run().await
}

///Google Cloud Run Port Injection
fn get_configured_address() -> String {
    match std::env::var("PORT") {
        Ok(port) => format!("0.0.0.0:{}", port),
        Err(_e) => "127.0.0.1:8000".into(),
    }
}
