use reqwest::Client;
use reqwest::header::USER_AGENT;

pub struct SearchScanlator{
    base_url : String,
    id : String,
}

impl SearchScanlator{
    pub fn new(id:String)->Self{
        SearchScanlator{
            base_url:"https://api.mangadex.org".to_string(),
            id,
        }
    }
    pub async fn search_scanlator(&self)-> Result<String,reqwest::Error>{
        let url = format!("{}/group/{}", self.base_url, self.id);
        let client = Client::new();
        let response = client.get(url)
            .header(USER_AGENT, "KindMango")
            .send().await?
            .text().await?;
        Ok(response)
    }
}