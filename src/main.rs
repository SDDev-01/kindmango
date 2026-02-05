mod search_manga;
mod search_manga_response;
mod search_chapter;
mod search_chapter_response;
mod search_scanlator;
mod search_scanlator_response;
mod server_chapter;
mod server_chapter_response;
mod models;
mod download_chapter;
mod download_chapter_response;
mod storage_cache;
use futures::stream::{self};
use models::ServerInfo;
use core::f32;
use std::{path::PathBuf, process::Command};
use server_chapter::ServerChapter;
use search_scanlator::SearchScanlator;
use search_chapter::SearchChapter;
use search_manga::SearchManga;
use std::io::{self, Write};
use tokio::{self};
use futures::StreamExt;
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

use crate::{download_chapter::DownloadChapter,search_chapter_response::Chapter, search_manga_response::Manga, storage_cache::StorageCache};

pub struct MangaInfo {
    pub id: String,
    pub title: String,
    pub available_translated_languages: Vec<String>,
}

pub struct ChapterInfo {
    pub id: String,
    pub index : usize,
    pub number: String,
    pub title: String,
    pub scanlator_name: String,
}

pub enum Menu {
    ReadNow,
    Next,
    Previous,
    Back,
    DoNothing
}

#[tokio::main]
async fn main() {
    println!("Hello, and thank you for using this program, all credits to mangadex!");
        //search mangas
    let title = ask_title();
    let search_manga = SearchManga::new(title);
    let manga_result = search_manga.search();
    let id_list : Vec<Manga> = match manga_result.await {
        Ok(json) => {
            search_manga_response::SearchMangaResponse::organize_data(json)
        }
        Err(e) => {
            println!("ERROR!!!(ó﹏ò｡): {}", e);
            return;
        }
    };
        //search language and manga title
    let manga_info = select_manga(id_list);
    let id = &manga_info.id; //of the title
    let language_list = &manga_info.available_translated_languages;
    let language = ask_language(language_list.to_vec());

        //search chapters
    let search_chapter = SearchChapter::new(id.to_string(), language);
            //vector of chapters
    let chapters:Vec<Chapter> = match search_chapter.search_chapter().await { 
        Ok(json) => {
            search_chapter_response::SearchChapterResponse::chapter_data(json)
        }
        Err(e) => {
            println!(r"{}, something happened (ᵕ—ᴗ—)", e);
            Vec::default()
        }       
    };
        //chapter menu 
    let index = select_chapter(&chapters);
    let chapter_info = get_chapter_info(&chapters, index).await;
    let mut desicion: Menu;
    let mut other_chapter: Option<ChapterInfo> = None;
    loop {
        if let Some(other_chapter) = &other_chapter{
            desicion = handle_menu_choice(&other_chapter);
        }else {
            desicion = handle_menu_choice(&chapter_info);
        }
        match desicion {
            Menu::ReadNow => {
                let (tx, rx) = oneshot::channel();
                tokio::spawn(loading(rx));

                let server_chapter;

                if let Some(chapter)=&other_chapter{
                    server_chapter = download_manga(&chapter).await;
                }else{
                    server_chapter = download_manga(&chapter_info).await;
                }

                let manga_cbz = manga_cbz(&server_chapter,tx).await;
                let title_id = manga_info.id.clone();
                let chapter_id:String;

                if let Some(chapter)=&other_chapter{
                    chapter_id = chapter.id.clone();
                }else{
                    chapter_id = chapter_info.id.clone();
                }

                let save = save_cache(title_id, chapter_id, manga_cbz);
                open_yacreader(save);
                continue;
            }
            Menu::Next => {
                if let Some(chapter)=&other_chapter{
                    other_chapter = Some(next_chapter(&chapter, &chapters).await);  
                }else{
                    other_chapter = Some(next_chapter(&chapter_info, &chapters).await);
                }
                continue;
            }
            Menu::Previous => {
                if let Some(chapter)=&other_chapter{
                    other_chapter = Some(previous_chapter(&chapter, &chapters).await);  
                }else{
                    other_chapter = Some(previous_chapter(&chapter_info, &chapters).await);
                }
                continue;
            }
            Menu::Back => {
                break;
            }
            _ => {
                panic!("aaaaaAAAAAAAAA (ᗒᯅᗕ;)՞°")
            }
        }
    }

}

//function that will ask for the title

fn ask_title()-> String{
    println!("please enter the name of the manga youre searching for: ");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("you didnt enter a correct string");
    let title = title.trim().to_string();
    title
}

fn show_manga(parsed:&Vec<search_manga_response::Manga>){
    
    for (i, manga) in  parsed.iter().enumerate(){
        let name = manga.get_attributes().get_title().values().next().unwrap();
            // this one search goes to altTitles and iter it while searching for a Key that has the variable en, then it tooks the value of that key
        let alt_en = manga.get_attributes().get_alt_title()
                                    .iter()
                                    .find_map(|map| map.get("en")); 
            
        if let Some(alt_name) = alt_en {
            println!("{}) {} =/a.k.a./= {}", i+1, name, alt_name);
        }else{
            println!("{}) {}", i+1, name);
        }
    }
}

fn select_manga(parsed:Vec<search_manga_response::Manga>) -> MangaInfo {
    show_manga(&parsed);
    let selection:usize = ask_manga();

    let selected = &parsed[selection];
    let id = selected.get_id().to_string();
    let title = selected.get_attributes().get_title().values().next().cloned().unwrap_or_else(|| "Untitled".to_string());
    let available_translated_languages = selected.get_attributes().get_languages().clone();

    MangaInfo { id ,title, available_translated_languages}

}

// function that will ask for a number from the list to know which one the user will watch
// TODO: i need to add a function that will show another page because theres a lot of results

fn ask_manga()-> usize{    
    println!("please enter the number of the manga you want to read: ");
    let number = ask_number();

    if number >10{
        println!(r"(っ˶°ㅁ°ς) !! Sorry please enter a number on the list!");
        ask_number()
    }else {
        number
    }

    
}

fn ask_number() -> usize{
    loop {
        let mut input= String::new();

        if io::stdin().read_line(&mut input).is_err(){
            println!(r"\(˚☐˚”)/ ERROR!!! \(˚☐˚”)/ ");
            continue;
        }
            //it will convert number from a string to i32, if it cant, then itll show an error

        let number:usize = match input.trim().parse(){
            Ok(n) => {
                n
            },
            Err(_) => {
                println!(r"(óロò ᵕ ) Please enter a positive number!!!");
                continue;
            }
        };
        return number-1;
       
    }
}

fn make_menu(list : &Vec<String>){
    for (i, name) in list.iter().enumerate(){
        println!("{}) {}",i+1 , name)
    }
}

fn ask_language(chapter_list:Vec<String>)->String{
    println!("Please choose a language!");
    make_menu(&chapter_list);
    let selection = ask_number();
    let language = match chapter_list.get(selection) {
        Some(lang) => lang.clone(),
        None => {
            println!(r"sorry that wasnt an option! (꒪⌓꒪)");
            String::new()
        }
    };
    language
}

fn show_chapter(chapter_list:&Vec<search_chapter_response::Chapter>){
    println!("Select your chapter by the index!");
    for (i,chapter) in chapter_list.iter().enumerate(){
        let number = chapter.attributes.get_chapter_number().unwrap_or("---");
        let title = chapter.attributes.get_title().unwrap_or("untitled (ーー;)");
        println!("{}) chapter {} {}",i+1 ,number ,title );
    }
}

fn select_chapter(chapter_list:&Vec<Chapter>) -> usize{
    show_chapter(chapter_list);
    let selection = ask_number();
    selection
}

async fn get_chapter_info(chapters:&Vec<search_chapter_response::Chapter>, index : usize) -> ChapterInfo{
    
    let data = chapters.get(index).unwrap();
        //information of the chapter
    let id = data.get_chapter_id().to_string();
    let number = data.get_attributes().get_chapter_number().unwrap_or("---").to_string();
    let title = data.get_attributes().get_title().unwrap_or("untitled (ーー;)").to_string();
    let scanlator_id = data.get_relationship().iter()
    .find(|rel| rel.get_relationship_type() == "scanlation_group")
    .map(|rel| rel.get_relationship_id())
    .expect("scanlator should exist (ᵕ,•ᴗ•)");
    let scanlator_name = get_scanlator_name(scanlator_id).await.to_string();
    
    ChapterInfo { id, index, number, title, scanlator_name}
}

async fn get_scanlator_name(id:&str) -> String{
    let search_scanlator = SearchScanlator::new(id.to_string());
    let scanlator_result = match search_scanlator.search_scanlator().await {
        Ok(json) => {
            search_scanlator_response::SearchScanlatorResponse::scanlator(json)
        }
        Err(e) => {
            println!(r"{}, something happened (°ー°〃)", e);
            return "scanlator should exist (ᵕ,•ᴗ•)".to_string();
        }
    };
    let scanlator_name = scanlator_result.get_attributes().get_scanlator().to_string();
    scanlator_name
}

fn show_menu(info:&ChapterInfo){
    println!(r"chapter {}) {}, scanlator: {}", info.number, info.title, info.scanlator_name);
    println!("---------------------------------------------------------------------------------------------------");
    println!("1) Read Now");
    println!("2) Next");
    println!("3) Previous");
    println!("4) Back")
}

fn handle_menu_choice(chapter_info:&ChapterInfo)->Menu{
    show_menu(&chapter_info);
    loop {
        println!("select your option!");
        let number = ask_number();
        match number {
            0=> {
                return Menu::ReadNow;
            },
            1=> {
                return Menu::Next;
            },
            2=> {
                return Menu::Previous;
            },
            3=> {
                return Menu::Back;
            },
            _=> println!(r"please enter a number on the list (^^ゞ")
        }
    }

}

async fn get_json_chapter(info:&ChapterInfo)->String{
    let server_chapter = ServerChapter::new(info.id.to_string());
        let search_response = match server_chapter.search().await {
            Ok(json) => {
                json
            }
            Err(e)=>{
                println!(r"{}, something happened °՞(ᗒᗣᗕ)՞°", e);
                return "something happened °՞(ᗒᗣᗕ)՞°".to_string();
            }
        };
        search_response
}

fn get_server_data(json:String)->ServerInfo{
    let server_chapter = server_chapter_response::ServerChapterResponse::server_data(json);
        //data of search_chapter_response
    let base_url = server_chapter.get_base_url();
    let hash = server_chapter.get_chapter().get_hash();
    let data = server_chapter.get_chapter().get_data();

    ServerInfo { base_url, hash, data}
}

async fn download_data(info : &ServerInfo)->Vec<Vec<u8>>{
    let download = DownloadChapter::new(info);
    let mut pages = Vec::new();
    for single_page in &info.data{
        let data = download.download_chapter(single_page);
        pages.push(data);
    };

    let manga = stream::iter(pages).buffered(7).collect::<Vec<_>>().await;
    
    //let manga = join_all(pages).await;

    let mut correct_manga = Vec::new();
    for correct_pages in manga{
        match correct_pages {
            Ok(correct) => {
                correct_manga.push(correct);
            }
            Err(e)=>{
                println!(r"{}, there were some errors while downloading the pages (っ- ‸ - ς)", e);
            }
        }
    };
    correct_manga
}

async fn manga_cbz(info : &ServerInfo, stop:oneshot::Sender<()>)->Vec<u8>{
//    let start = Instant::now();

    let manga_images = download_data(info).await;
    let download_response = download_chapter_response::DownloadChapterResponse::new(manga_images);
    let cbz = download_response.transform_cbz().expect(r"tranform to cbz went wrong ( • ᴖ • ｡)");
    let _ = stop.send(());
    
//    time("manga_cbz", start);
    cbz
}

fn save_cache(title_id:String, chapter_id:String, cbz:Vec<u8>)->PathBuf{
    let storage = StorageCache::new(title_id, chapter_id, cbz);
    let path = storage.storage();
    path
}

fn open_yacreader(manga_path : PathBuf){
    Command::new("YACReader").arg(manga_path).spawn().expect("something went wrong with YACReader");
}

async fn download_manga(chapter_info:&ChapterInfo)->ServerInfo{
    let json = get_json_chapter(&chapter_info).await;
    let server_chapter = get_server_data(json);
    server_chapter
}

async fn next_chapter(chapter_info:&ChapterInfo, chapter_list:&Vec<Chapter>)->ChapterInfo{
    let actual_chapter:Option<f32> = chapter_info.number.parse().ok();
    let mut index = chapter_info.index;
    if let Some(actual_chapter) = actual_chapter {
        for (i,chapter) in chapter_list[index..].iter().enumerate(){
            let number = chapter.get_attributes().get_chapter_number().and_then(|s| s.parse::<f32>().ok());
            if let Some(number) = number {
                if actual_chapter<number {
                    return get_chapter_info(chapter_list, index+i).await;
                }
            }else {
                return get_chapter_info(chapter_list, index+i).await;
            }
        }
    } else {
        index+= 1;
        return get_chapter_info(chapter_list, index).await;
    }
    return get_chapter_info(chapter_list, index+1).await;
}

async fn previous_chapter(chapter_info:&ChapterInfo, chapter_list:&Vec<Chapter>)->ChapterInfo{
    let actual_chapter:Option<f32> = chapter_info.number.parse().ok();
    let index = chapter_info.index;
    if let Some(actual_chapter) = actual_chapter {
        for (i,chapter) in chapter_list[..index].iter().enumerate().rev(){
            let number = chapter.get_attributes().get_chapter_number().and_then(|s| s.parse::<f32>().ok());
            if let Some(number) = number {
                if actual_chapter>number {
                    return get_chapter_info(chapter_list, i).await;
                }
            }else {
                return get_chapter_info(chapter_list, i).await;
            }
        }
    } else if index>0{
        return get_chapter_info(chapter_list, index-1).await;
    }
    return get_chapter_info(chapter_list, 0).await;
}


async fn loading(mut stop:oneshot::Receiver<()>){
    let frames = ["/","—","\\","|"];
    let mut i = 0;
    loop{
        if stop.try_recv().is_ok(){
            break;
        }

        print!("\rLoading ... {}", frames[i % frames.len()]);
        io::stdout().flush().unwrap();
        i += 1;

        sleep(Duration::from_millis(120)).await;
    }
    println!("\r\rReady o(≧∇≦o)");
}

/// to take the time
/// so youll start it with
/// let start = Instant::now();
/// and to end it and show the time youll put 
/// time("function_name", start);
use std::time::Instant;

pub fn time(name: &str, start: Instant) {
    let span = start.elapsed();
    println!(" {} took {:?}", name, span);
}