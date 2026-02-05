use std::collections::HashMap;

use serde::{Deserialize};

#[derive(Deserialize)]
pub struct SearchMangaResponse{
    data : Vec<Manga>
} 

#[derive(Deserialize)]
pub struct Manga{
    id : String,
    attributes : Attribute
}

#[derive(Deserialize)]
pub struct Attribute{
    title : HashMap<String,String>,
    #[serde(rename = "altTitles")]
    alt_titles : Vec<HashMap<String,String>>,
    #[serde(rename = "availableTranslatedLanguages")]
    available_translated_languages : Vec<String>

}

impl SearchMangaResponse {

    pub fn organize_data(json : String) -> Vec<Manga>{
    let parsed : SearchMangaResponse = match serde_json::from_str(&json){
        Ok(search) => search,
        Err(e) => {
            println!(r"{}, something happened ଽ ૮( ⁰▱๋⁰ )ა", e);
            return Default::default();
        }
    }; 
    parsed.data
    }
    
}

impl Manga {
    pub fn get_id(&self)->&str{
        &self.id
    }

    pub fn get_attributes(&self)->&Attribute{
        &self.attributes
    }
}

impl Attribute {
    pub fn get_title(&self)->&HashMap<String,String>{
        &self.title
    }
    pub fn get_alt_title(&self)->&Vec<HashMap<String,String>>{
        &self.alt_titles
    }
    pub fn get_languages(&self)->&Vec<String>{
        &self.available_translated_languages
    }
}