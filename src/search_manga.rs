use reqwest::Client;
use reqwest::header::USER_AGENT;


pub struct SearchManga{
    base_url : String,
    title : String
}

impl SearchManga{

    pub fn new(title: String) -> Self{
        SearchManga{
            base_url : "https://api.mangadex.org".to_string(),
            title
        }
        
    }

    //search for a manga

    pub async fn search(&self) -> Result<String , reqwest::Error>{
        let url = format!("{}/manga", self.base_url);

        let client = Client::new();
            // here we search for the manga based on the title and the url, and we need a header to identify ourselfves 
        let response = client.get(url)
            .query(&[("title", &self.title)])
            .header(USER_AGENT, "KindMango")
            .send().await?
            .text().await?;

        Ok(response)
    }


}