use actix_web::web::Data;
use structopt::StructOpt;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};

mod forwarder;

mod errors;
use errors::LogLevel;

use self::forwarder::ForwardingTable;

#[derive(Debug, StructOpt)]
#[structopt(name = "webserver")]
/// Personal page backend, shows either a dashboard if logged in or
/// a static site generated using Hugo
struct Opt {
    /// port on which this server will listen
    #[structopt(long)]
    port: u16,
    /// log level least, one of: Critical, Info
    #[structopt(long, default_value = "Critical")]
    verbosity: LogLevel,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    color_eyre::install().unwrap();

    let opt = Opt::from_args();
    errors::setup_logging(&opt.verbosity).unwrap();

    let forwarder = ForwardingTable::init();
    let data = Data::new(forwarder);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            // Enable the logger.
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/{a}")
                    // .name("url shortner")
                    // .guard(guard::Get())
                    .to(forwarder::route),
            )
            .service(Files::new("/", "./files/public/").index_file("index.html"))
    })
    .bind(format!("127.0.0.1:{}", opt.port))?
    .run()
    .await
}
