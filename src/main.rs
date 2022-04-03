use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use reqwest::Client;
mod testfuture;

pub const URL_1: &str = "https://www.kirima.xyz/post/sha256";
pub const URL_2: &str = "https://www.kirima.xyz/post/gentleintrosolana";

#[tokio::main]
 async fn main() -> Result<(), Report> {
    setup()?;

    info!("Hello, world!");
    info!("Building a testfuture");

    let fut = testfuture::TestFuture {};
    info!("Awaiting the future");
    fut.await;
    info!("Done awaiting that dumb future");


    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

 //async fn fetch_url(client: &Client, url: &str) -> Result<(), Report> {
 //   let res = client.get(url).send().await?.error_for_status()?;
   // info!(url, content_type = ?res.headers().get("content-type"));
  //  Ok(())
 //}