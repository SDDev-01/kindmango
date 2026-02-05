use reqwest::Client;
use reqwest::header::USER_AGENT;
use crate::models::ServerInfo;

#[derive(Clone)]
pub struct DownloadChapter{
    info : ServerInfo,
    client: Client
}

impl DownloadChapter{
    pub fn new(info : &ServerInfo)->Self{
        DownloadChapter{
            info: info.clone(),
            client: Client::new(),
        }
    }

    pub async fn download_chapter(&self ,page:&str)-> Result<Vec<u8>, reqwest::Error>{
    //    
        let url = format!("{}/data/{}/{}", self.info.base_url, self.info.hash, page);
        let response = self.client.get(url)
            .header(USER_AGENT, "KindMango")
            .send().await.unwrap()
            .bytes().await.unwrap();
        let data:Vec<u8>=response.to_vec();

    Ok(data)
    }
}

