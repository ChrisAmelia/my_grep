#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use crate::cli::{get_filename, get_pattern, parse_flags, Flag};

    #[test]
    fn test_parse_flags() {
        // No flags
        let args = vec!["grep", "hello.txt", "lorem"];
        assert!(parse_flags(&args).is_empty());

        // -i, -n
        let args = vec!["grep", "hello.txt", "lorem", "-i", "-n"];
        assert_eq!(vec![Flag::Insensitive, Flag::ShowLineNumber], parse_flags(&args));
    }

    #[test]
    fn test_get_filename() {
        // No filename
        let args = vec!["grep", "-i", "-n"];
        assert_eq!(Err(ErrorKind::NotFound), get_filename(&args).map_err(|e| e.kind()));

        // Filename given
        let args = vec!["grep", "-i", "-n", "hello_world.txt", "lorem"];
        assert_eq!("hello_world.txt", get_filename(&args).unwrap());
    }

    #[test]
    fn test_get_pattern() {
        // No pattern
        let args = vec!["grep", "-i", "-n", "hello_world.txt"];
        assert_eq!(Err(ErrorKind::NotFound), get_pattern(&args).map_err(|e| e.kind()));

        // Pattern given
        let args = vec!["grep", "-i", "-n", "hello_world.txt", "lorem"];
        assert_eq!("lorem", get_pattern(&args).unwrap());
    }
}
