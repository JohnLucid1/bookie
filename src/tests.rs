#[cfg(test)]
mod tests {
    const PATH: &str = "The_Artists_Way_Julia_Cameron.epub";
    use epub::doc::*;
    use std::{
        fmt::Write,
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn read_book_name() {
        let doc = EpubDoc::new(PATH).unwrap();
        let doc_subjects = doc.metadata.get("subject").unwrap(); // TODO: if more than one tag use metaadta.get instead of mdata
        println!("{:#?}", doc_subjects)
    }

    fn parse_tags(book: &EpubDoc<BufReader<File>>, tag: &str) -> Option<Vec<String>> {
        book.metadata.get(tag).cloned()
    }

    #[test]
    fn test_it_all() {
        let book = EpubDoc::new(PATH).unwrap();
        let thingy = parse_tags(&book, "subject");
        println!("{:#?}", thingy);
    }

    #[actix_rt::test]
    async fn db_book() {
        // NOTE: This is a test of db (not saving)
        let db_url = std::env::var("DB_URL").expect("Coudln't get url from .env file");
        let pool = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Couldn't connect to db");
    }

    // fn get_xml_attr(xml: &str, tag: &[u8]) -> Option<Vec<String>> {
    //     let mut reader = Reader::from_str(xml);
    //     reader.trim_text(true);

    //     let mut buf = Vec::new();
    //     let mut tags_values: Vec<String> = Vec::new();
    //     let mut inside_tag = false;
    //     loop {
    //         match reader.read_event_into(&mut buf) {
    //             Ok(Event::Start(e)) => {
    //                 if e.as_ref() == tag {
    //                     inside_tag = true;
    //                 }
    //             }
    //             Ok(Event::Text(e)) => {
    //                 if inside_tag {
    //                     tags_values.push(e.unescape().unwrap().to_owned().to_string());
    //                     inside_tag = false;
    //                 }
    //             }
    //             Ok(Event::Eof) => break,
    //             Err(e) => eprintln!("ERROR: {:?}", e),
    //             _ => (),
    //         }
    //         buf.clear();
    //     }
    //     return Some(tags_values);
    // }

    // fn search_file_in_epub(file_path: &str, target_extension: &str) -> Option<String> { // TODO: only use this bullsheet on tags aka subject
    //     let file = File::open(file_path).expect("Failed to open .epub file");
    //     let mut archive = ZipArchive::new(file).expect("Failed to open .epub zip");

    //     for i in 0..archive.len() {
    //         let mut file = archive.by_index(i).expect("Failed to get file by id");
    //         let file_name = file.name().to_string();

    //         if file_name.ends_with(target_extension) {
    //             let mut buffer = String::new();
    //             file.read_to_string(&mut buffer)
    //                 .expect("Failed to read file content");
    //             return Some(buffer);
    //         }
    //     }
    //     return None;
    // }
}

/*
    langauge
    publisher
    title
    description
    subject
*/
