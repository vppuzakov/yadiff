use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Error;

use crate::yadisk::models::Resource;

pub async fn get_resource(token: String, path: String) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let request_url = format!(
        "https://cloud-api.yandex.net/v1/disk/resources?path={path}",
        path = path,
    );

    let response = client
        .get(&request_url)
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, format!("OAuth {token}", token = token))
        .header(USER_AGENT, "yadiff")
        .send()
        .await?;

    let resource: Resource = response.json().await?;
    println!("{:?}", resource);
    Ok(())
}
