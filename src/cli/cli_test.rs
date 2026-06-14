#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use crate::cli::{get_filenames, get_pattern, parse_flags, Flag};

    #[test]
    fn test_parse_flags() {
        // No flags
        let args = vec!["grep", "lorem", "hello.txt"];
        assert!(parse_flags(&args).is_empty());

        // -i
        let args = vec!["grep", "lorem", "hello.txt", "-i"];
        assert_eq!(vec![Flag::Insensitive], parse_flags(&args));
    }

    #[test]
    fn test_get_filenames() {
        // No filename
        let args = vec!["grep", "-i"];
        assert!(get_filenames(&args).is_none());

        // Filename given
        let args = vec!["grep", "lorem", "hello_world.txt"];
        assert_eq!("hello_world.txt", get_filenames(&args).unwrap()[0]);

        // Multiple files
        let args = vec!["grep", "lorem", "file1.txt", "file2.txt", "file3.txt"];
        let filenames = get_filenames(&args).unwrap();

        assert_eq!("file1.txt", filenames[0]);
        assert_eq!("file2.txt", filenames[1]);
        assert_eq!("file3.txt", filenames[2]);
    }

    #[test]
    fn test_get_pattern() {
        // No pattern
        let args = vec!["grep", "-i", "-n"];
        assert_eq!(Err(ErrorKind::NotFound), get_pattern(&args).map_err(|e| e.kind()));

        // Pattern given
        let args = vec!["grep", "-i", "-n", "lorem", "hello_world.txt"];
        assert_eq!("lorem", get_pattern(&args).unwrap());
    }
}
