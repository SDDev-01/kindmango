use zip::write::SimpleFileOptions;
use std::io::{Cursor, Write};
pub struct DownloadChapterResponse{
    img: Vec< Vec<u8> >
}
// this one will download the chapter and make it an cbz to read it later
impl DownloadChapterResponse{
    pub fn new(img:Vec< Vec<u8> >)->Self{
        DownloadChapterResponse { img }
    }

    pub fn transform_cbz(&self)-> zip::result::ZipResult<Vec<u8>>{
        let buffer = Cursor::new(Vec::new());
        let mut zip = zip::ZipWriter::new(buffer);

        for (i, page) in self.img.iter().enumerate(){
            let name = format!("{:03}.png", i+1);
            zip.start_file(name, SimpleFileOptions::default())?;
            zip.write_all(page)?;
        }
        let buffer = zip.finish()?;
        Ok(buffer.into_inner())
    }
}