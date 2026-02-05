use reqwest::Client;
use reqwest::header::USER_AGENT;

pub struct SearchChapter{
    base_url : String,
    id : String,
    language : String
}

impl SearchChapter{
    pub fn new(id:String, language:String)->Self{
        SearchChapter{
            base_url:"https://api.mangadex.org".to_string(),
            id,
            language,
        }
    }
    pub async fn search_chapter(&self)-> Result<String,reqwest::Error>{
        let url = format!("{}/manga/{}/feed", self.base_url, self.id);
        let client = Client::new();
        let response = client.get(url)
            .query(&[
                ("translatedLanguage[]", self.language.as_str()),
                ("order[chapter]", "asc"),
            ])
            .header(USER_AGENT, "KindMango")
            .send().await?
            .text().await?;
        Ok(response)
    }
}