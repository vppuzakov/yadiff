use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Error;

use super::Categories;
use super::Resource;

pub async fn get_resource(token: &String, path: &String, limit: u32, offset: u32) -> Result<Resource, Error> {
    let client = reqwest::Client::new();

    let request_url = format!(
        "https://cloud-api.yandex.net/v1/disk/resources?path={path}&limit={limit}&offset={offset}",
        path = path,
        limit = limit,
        offset = offset,
    );

    let response = client
        .get(&request_url)
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, format!("OAuth {token}", token = token))
        .header(USER_AGENT, "yadiff")
        .send()
        .await?;

    let resource: Resource = response.json().await?;
    Ok(resource)
}

pub async fn get_all_files(token: &String, path: &String, limit: u32) -> Result<Vec<Resource>, Error> {
    let mut files: Vec<Resource> = Vec::new();
    let mut offset = 0;

    let mut folder = get_resource(&token, &path, 1, 0).await?;
    let mut embedded = folder.embedded.unwrap();
    let total = embedded.total;

    while offset < total {
        folder = get_resource(&token, &path, limit, offset).await?;
        embedded = folder.embedded.unwrap();
        for item in embedded.items {
            if let Categories::FILE = item.category {
                files.push(item);
            }
        }
        offset += limit;
    }

    Ok(files)
}
