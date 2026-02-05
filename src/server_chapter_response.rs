use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ServerChapterResponse{
    #[serde(rename = "baseUrl")]
    base_url : String,
    chapter : Chapter
}


#[derive(Deserialize)]
pub struct Chapter{
    hash : String,
    data : Vec<String> 
}
impl ServerChapterResponse{
    pub fn server_data(json:String) -> ServerChapterResponse{
        let response:ServerChapterResponse = serde_json::from_str(&json).expect(r"something went wrong ( ;• - •;)");
        response
    }
    pub fn get_base_url(&self) -> String{
        self.base_url.to_string()
    }
    pub fn get_chapter(&self) -> &Chapter{
        &self.chapter
    }
}

impl Chapter {
    pub fn get_hash(&self) -> String{
        self.hash.to_string()
    }
    pub fn get_data(&self) -> Vec<String>{
        self.data.clone()
    }    
}