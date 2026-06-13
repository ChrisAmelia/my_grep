#[cfg(test)]
mod tests {
    use crate::{grep::{search_in_one_file, FileMatch}, Flag};

    const FILE_TEST: &str = "src/hello_world.txt";

    #[test]
    fn search_in_one_file_with_inexistant_file_should_error_not_found() {
        let result = search_in_one_file("not_found.txt", "impossible", &[]).map_err(|e| e.kind());
        assert_eq!(Err(std::io::ErrorKind::NotFound), result)
    }

    #[test]
    fn search_in_one_file_should_show_line_2() {
        let result: FileMatch = search_in_one_file(FILE_TEST, "dolor", &[]).unwrap();

        assert_eq!(1, result.lines[0].index);
        assert_eq!("dolor sit amet", result.lines[0].content);
    }

    #[test]
    fn search_in_one_file_with_inexistant_word_should_empty_result() {
        let result: FileMatch = search_in_one_file(FILE_TEST, "english", &[]).unwrap();

        assert!(result.lines.is_empty());
    }

    #[test]
    fn search_in_one_file_should_show_two_results() {
        let result: FileMatch = search_in_one_file(FILE_TEST, "ipsum", &[]).unwrap();

        assert_eq!(0, result.lines[0].index);
        assert_eq!("lorem ipsum", result.lines[0].content);

        assert_eq!(2, result.lines[1].index);
        assert_eq!("lorem ipsum dolor", result.lines[1].content);
    }

    #[test]
    fn search_in_one_file_should_return_with_empty() {
        let result: FileMatch = search_in_one_file(FILE_TEST, "hello world", &[]).unwrap();

        assert!(result.lines.is_empty());
    }

    #[test]
    fn search_in_one_file_insensitive_test() {
        // Looking for "DOLOR", note the uppercased word
        let result: FileMatch = search_in_one_file(FILE_TEST, "DOLOR", &[]).unwrap();
        assert!(result.lines.is_empty());

        // Looking for "DOLOR", with -i set
        let result: FileMatch = search_in_one_file(FILE_TEST, "DOLOR", &[Flag::Insensitive]).unwrap();
        assert_eq!(1, result.lines[0].index);
        assert_eq!("dolor sit amet", result.lines[0].content);
    }
}
