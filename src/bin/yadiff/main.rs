use config::Config;
use reqwest::Error;
use yadiff::diff;
use yadiff::filescanner;
use yadiff::yadisk::get_all_files;

mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::new();
    let resources = get_all_files(&config.token, &config.remote, config.window).await?;

    let ya_files = diff::from_yadisk(resources, &config.remote);

    println!("\nremote files:");
    for file in ya_files.iter() {
        println!("{:?} {:?}", file.path, file.sha256);
    }

    let localpaths = filescanner::get_all_files(&config.local);
    let localfiles = diff::from_local(localpaths, &config.local);

    println!("\nlocal files:");
    for file in localfiles.iter() {
        println!("{:?} {:?}", file.path, file.sha256);
    }

    println!("\n find diff:");
    diff::find_diff(&ya_files, &localfiles);

    Ok(())
}
