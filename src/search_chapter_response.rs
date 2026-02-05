use serde::{Deserialize};

//this module will get the chapter list
//DATA OF THE CHAPTER

#[derive(Deserialize)]
pub struct SearchChapterResponse{
    data: Vec<Chapter>
}

#[derive(Deserialize)]
pub struct Chapter{
    id : String,
    pub(crate) attributes : Attribute,
    relationships : Vec<Relationship>
}

#[derive(Deserialize)]
pub struct Attribute{	
    chapter : Option<String>,
    title : Option<String>
}


#[derive(Deserialize)]
pub struct Relationship{
    id : String,
    r#type: String,
}	

impl SearchChapterResponse{
    pub fn chapter_data(json : String) -> Vec<Chapter>{
        let response:SearchChapterResponse  = match serde_json::from_str(&json) {
            Ok(search) => search,
            Err(e) => {
                println!(r"{}, something happened (・へ・)", e);
                return Default::default();
            }
        }; 
        response.data
    }
}

impl Chapter {
    pub fn get_chapter_id(&self)->&str{
        &self.id
    }
    pub fn get_attributes(&self)->&Attribute{
        &self.attributes
    }
    pub fn get_relationship(&self)->&Vec<Relationship>{
        &self.relationships
    }
}

impl Attribute {
    pub fn get_chapter_number(&self)->Option<&str>{
        self.chapter.as_deref()
    }

    pub fn get_title(&self)->Option<&str>{
        self.title.as_deref()
    }
}

impl Relationship {
    pub fn get_relationship_id(&self)->&str{
        &self.id
    }
    pub fn get_relationship_type(&self)->&str{
        &self.r#type
    }
}