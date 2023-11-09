#[cfg(test)]
mod tests {
    // TODO: rewrite test so they will use eq!
    const PATH: &str = "./books/The_Artists_Way_Julia_Cameron.epub";
    use std::path::Path;

    use crate::books::FileType;

    #[actix_rt::test]
    async fn testing_parse() {
        let path = Path::new(PATH);
        let thingy = FileType::parse_epub(path).unwrap();
        println!("{:#?}", thingy);
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
