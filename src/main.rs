use structopt::StructOpt;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};

mod errors;
use errors::LogLevel;

#[derive(Debug, StructOpt)]
#[structopt(name="webserver")]
/// Personal page backend, shows either a dashboard if logged in or 
/// a static site generated using Hugo
struct Opt {
    /// port on which this server will listen
    #[structopt(long, default_value = "Critical")]
    port: u16,
    /// log level least, one of: Critical,
    #[structopt(long, default_value = "Critical")]
    verbosity: LogLevel,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    errors::setup_logging(&opt.verbosity).unwrap();

    HttpServer::new(|| {
        App::new()
            // Enable the logger.
            .wrap(middleware::Logger::default())
            .service(Files::new("/", "./files/public/").index_file("index.html"))
    })
    .bind(format!("127.0.0.1:{}",opt.port))?
    .run()
    .await
}
