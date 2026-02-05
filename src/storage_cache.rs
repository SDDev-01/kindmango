use std::{fs, path::PathBuf};

use dirs;

pub struct StorageCache{
    title_id : String,
    chapter_id : String,
    cbz : Vec<u8>
}
impl StorageCache{
    pub fn new(title_id:String, chapter_id:String, cbz:Vec<u8>)->Self{
        StorageCache{
            title_id,
            chapter_id,
            cbz
        }
    }

    pub fn storage(&self)->PathBuf{
        let cache = dirs::cache_dir().expect(r"something went wrong ( – ⌓ – )");
        let kindmango = cache.join("kindmango").join(&self.title_id).join(&self.chapter_id);
        fs::create_dir_all(&kindmango).unwrap();

        let name = format!("{}.cbz",self.chapter_id);
        let name_path = kindmango.join(name);
        if !name_path.exists(){
            fs::write(&name_path, &self.cbz).unwrap();
        };
        name_path
    }


}