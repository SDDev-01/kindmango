use reqwest::Client;
use reqwest::header::USER_AGENT;

pub struct ServerChapter{
    base_url : String,
    id : String,
}

impl ServerChapter{
    pub fn new(id: String) -> Self{
        ServerChapter{
            base_url : "https://api.mangadex.org".to_string(),
            id
        }
    }

    pub async fn search(&self)->Result<String, reqwest::Error>{
        let url = format!(r"{}/at-home/server/{}", self.base_url, self.id);
        let client = Client::new();
        let response = client.get(url)
            .header(USER_AGENT, "KindMango")
            .send().await?
            .text().await?;
        Ok(response)
    }
}