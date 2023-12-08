#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::books::FileType;

    // Parsing of files
    #[actix_rt::test]
    async fn parse_epub() {
        const PATH: &str = "./books/The_Artists_Way_Julia_Cameron.epub";
        let path = Path::new(PATH);
        let result = FileType::parse_epub(path).unwrap();
        let author = "Julia Cameron".to_string();
        let title = "The Artist's Way".to_string();
        let genres = vec!["Self Help".to_string(), "Creativity".to_string(), "Nonfiction".to_string()];

        assert_eq!(result.author, author);
        assert_eq!(result.title, title);
        assert_eq!(result.genres, genres);
    }
}
