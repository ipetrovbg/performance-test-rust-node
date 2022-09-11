use lambda_runtime::{handler_fn, Context};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let handler_callback = handler_fn(rust_handler);
    lambda_runtime::run(handler_callback).await?;

    Ok(())
}

async fn rust_handler(_ev: i8, _context: Context) -> Result<(), Error> {
    info!("Initializing rust lambda!");

    Ok(())
}