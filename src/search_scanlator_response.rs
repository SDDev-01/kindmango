use serde::{Deserialize};

#[derive(Deserialize)]
pub struct SearchScanlatorResponse{
    data : Data
}

#[derive(Deserialize)]
pub struct Data{
    attributes : Attribute
}

#[derive(Deserialize)]
pub struct Attribute{
    name : String
}

impl SearchScanlatorResponse{
    pub fn scanlator(json:String) -> Data{
        let response:SearchScanlatorResponse = serde_json::from_str(&json).expect("scanlator should exist (ᵕ,•ᴗ•)");
        response.data
    }
}

impl Data{
    pub fn get_attributes(&self)->&Attribute{
        &self.attributes
    }
}

impl Attribute{
    pub fn get_scanlator(&self)->&str{
        &self.name
    }
}