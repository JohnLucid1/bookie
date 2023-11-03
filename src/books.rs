use std::{path::Path, io::BufReader, fs::File};

use epub::doc::{EpubDoc, DocError};

pub struct Book {
    title: String,
    author: String,
    book_path: String,
    description: String,
    download_count: u32,
    file_size: u32,
    language: String,
    genres: Vec<String>,
    format: String,
}

/*
    NOTE: all the metadata of books is stored in .opf file

    NOTE: How does the new function should work
        _it should receive a valid path:
        _it should get the filetype and\n
        depending on the filetype do all the parsing


    workflow
    1. Admin sends a book:
        book is saved in temp folder
        parse all the data from the book
        if no error than save and send success message
        else send error
*/

impl Book {
    fn new(temp_filepath: &Path) {
        if let Some(ext) = FileType::get_extension(temp_filepath) {
            match ext {
                FileType::EPUB => FileType::parse_epub(temp_filepath),
                FileType::FB2 => FileType::parse_fb2(temp_filepath),
            }
        }
    }
}

pub enum FileType {
    EPUB,
    FB2,
}

impl FileType {
    fn get_extension(path: &Path) -> Option<FileType> {
        if let Some(os_exstension) = path.extension() {
            let extension = os_exstension.to_str().unwrap_or("");
            match extension {
                "epub" => Some(FileType::EPUB),
                "fb2" => Some(FileType::FB2),
                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_tag(book: &EpubDoc<BufReader<File>>, tag:&str) -> Option<String> {
        book.mdata(tag)
    }
    
    fn parse_tags(book: &EpubDoc<BufReader<File>>, tag:&str) -> Option<Vec<String>> { 
        book.metadata.get(tag).cloned()
    }

    // NOTE: only done on adding a book to db
    // NOTE: this function should return a book and another function in db module should save it
    // TODO: get epub 
    pub fn save_epub(temp_path: &Path) -> Result<Book, DocError > {
        let doc = EpubDoc::new(temp_path).expect("Couldn't get temporary book");
        let title = FileType::parse_tag(&doc, "title");
        let author = FileType::parse_tag(&doc, "creator").unwrap_or("".into());
        let description = FileType::parse_tag(&doc, "description").unwrap_or("".into());
        let filesize = File::open(&temp_path).unwrap().metadata().unwrap().len(); // TODO: either this or get from tg message data
        let language = FileType::parse_tag(&doc, "language").unwrap_or("".into());
        let gunres = FileType::parse_tags(&doc, "subject").unwrap_or(Vec::new());

        Ok(Book {
            title, 
            author,
            book_path: 
              
        })
    }

    
    
    
    


    fn parse_fb2(path: &Path) {
        todo!();
    }
}

// creator, author,
// impl Parse for FileType {
//     fn parse(doc_path: String) -> Option<Book> {
//         // TODO: from path get extension and then parse accrodingly
//         // let path = Path::new(doc_path).extension();
//         if let Some(extension) = Parse::get_filetype(&doc_path) {
//             todo!()
//         }

//     }
// }

// pub trait Parse {
//     fn parse(path: String) -> Option<Book>;
// }
