use log::info;
use chrono::Local;
use actix_web::HttpServer;
use actix_web::App;
use fern::Dispatch;
use rust_init_demo::api;
use rust_init_demo::db;
use rust_init_demo::setting;
use rust_init_demo::error::Result;


fn log_init() {
    std::env::set_var("RUST_LOG", "sqlx::query=error");

    let level = match setting::get_log_level().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        _ => log::LevelFilter::Error,
    };
    let log_path = setting::get_log_path();
    // Builder::new()
    //     .parse_env("RUST_LOG")
    //     .format(|buf, record| {
    //         let level = { buf.default_styled_level(record.level()) };
    //         writeln!(
    //             buf,
    //             "{} {} [{}:{}] {}",
    //             Local::now().format("%Y-%m-%d %H:%M:%S"),
    //             level,
    //             record.module_path().unwrap_or("<unnamed>"),
    //             record.line().unwrap_or(0),
    //             &record.args()
    //         )
    //     })
    //     .filter(None, level).init();

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} [{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path).unwrap())
        .level(level)
        .level_for("sqlx::query", log::LevelFilter::Error)
        .apply().expect("log init error");
}

#[actix_web::main]
async fn main() -> Result<()> {
    log_init();
    let conn_string = setting::get_conn_string();
    info!("conn_string:{}",conn_string);
    // db::init_connections(conn_string.as_str()).await?;
    let config = &*setting::SETTING;
    let app = &config.app;
    info!("server listening at http://{}:{}", app.host, app.port);

    HttpServer::new(move|| {
        App::new()
        .service(api::test_api::routes())
    })
    .bind((app.host.as_str(), app.port))?
    .run()
    .await?;

    Ok(())
}