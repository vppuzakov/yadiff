use config::Config;
use reqwest::Error;
use yadiff::filescanner;
use yadiff::yadisk::get_all_files;

mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new();
    let files = get_all_files(&config.token, &config.remote, config.window, &config.remote).await?;

    println!("\nremote files:");
    for file in files {
        println!("{:?}", file.path);
    }

    let localfiles = filescanner::get_all_files(&config.local);
    println!("\nlocal files:");
    for file in localfiles {
        println!("{:?}", file.path);
    }
    Ok(())
}
