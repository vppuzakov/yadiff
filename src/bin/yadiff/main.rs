use config::Config;
use reqwest::Error;
use yadiff::yadisk::get_resource;

mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new();
    let resource = get_resource(config.token, config.remote).await?;

    println!("{:?}", resource);
    Ok(())
}
