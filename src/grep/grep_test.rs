#[cfg(test)]
mod tests {
    use crate::{grep::{grep, Match}, Flag};

    const FILE_TEST: &str = "src/hello_world.txt";

    #[test]
    fn grep_with_inexistant_file_should_error_not_found() {
        let result = grep("not_found.txt", "impossible", &[]).map_err(|e| e.kind());
        assert_eq!(Err(std::io::ErrorKind::NotFound), result)
    }

    #[test]
    fn grep_should_show_line_2() {
        let result: Vec<Match> = grep(FILE_TEST, "dolor", &[]).unwrap();

        assert_eq!(1, result[0].line_number);
        assert_eq!("dolor sit amet", result[0].line);
    }

    #[test]
    fn grep_should_show_two_results() {
        let result: Vec<Match> = grep(FILE_TEST, "ipsum", &[]).unwrap();

        assert_eq!(0, result[0].line_number);
        assert_eq!("lorem ipsum", result[0].line);

        assert_eq!(2, result[1].line_number);
        assert_eq!("lorem ipsum dolor", result[1].line);
    }

    #[test]
    fn grep_should_return_with_empty() {
        let result: Vec<Match> = grep(FILE_TEST, "hello world", &[]).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn grep_insensitive_test() {
        // Looking for "DOLOR", note the uppercased word
        let result: Vec<Match> = grep(FILE_TEST, "DOLOR", &[]).unwrap();
        dbg!(&result);
        assert!(result.is_empty());

        // Looking for "DOLOR", with -i set
        let result: Vec<Match> = grep(FILE_TEST, "DOLOR", &[Flag::Insensitive]).unwrap();
        assert_eq!(1, result[0].line_number);
        assert_eq!("dolor sit amet", result[0].line);
    }
}
