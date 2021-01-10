use config::Config;
use reqwest::Error;
use yadiff::yadisk::get_all_files;

mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new();
    let resource = get_all_files(&config.token, &config.remote, config.window).await?;

    println!("{:?}", resource);
    Ok(())
}
