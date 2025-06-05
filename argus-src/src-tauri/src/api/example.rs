use anyhow::Result;
use crate::net_connection::http_client::get_base_url;

pub async fn get_example() -> Result<String> {
    let base_url = get_base_url();

    let client = reqwest::Client::new();
    // let client = ClientBuilder::new().timeout(Duration::from_secs(10)).build()?;

    // let res = client.get(url).send().await?;

    let url = format!("{}", base_url);
    println!("{}", url);
    let response = client.get(url).send().await?;
    let string = response.text().await?;
    // let post = response.json::<String>().await?;
    log::info!("请求返回:{}", string);
    Ok(string)
}

// pub async fn create_post(client: &HttpClient, new_post: &NewPost) -> Result<Post, ApiError> {
//     let url = "https://jsonplaceholder.typicode.com/posts";
//     let response = client.post(&url, new_post).await?;
//     let post = response.json::<Post>().await?;
//     Ok(post)
// }
