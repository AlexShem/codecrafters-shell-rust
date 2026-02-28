/// Parse a command line into command name and arguments
pub fn parse_command_line(input: &str) -> anyhow::Result<Vec<String>> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '\'' => {
                // Consume the single quote
                chars.next();
                // Read until the closing quote
                while let Some(&inner_ch) = chars.peek() {
                    if inner_ch == '\'' {
                        chars.next();
                        break;
                    }
                    current_arg.push(inner_ch);
                    chars.next();
                }
            }
            '"' => {
                // Consume the opening double quote
                chars.next();
                while let Some(&inner_ch) = chars.peek() {
                    if inner_ch == '"' {
                        chars.next();
                        break;
                    }
                    current_arg.push(inner_ch);
                    chars.next();
                }
            }
            '\\' => {
                // Consume the backslash
                chars.next();
                // If there's a next character, add it literally
                if let Some(&next_ch) = chars.peek() {
                    current_arg.push(next_ch);
                    chars.next();
                }
            }
            ' ' | '\n' | '\t' | '\r' => {
                chars.next();
                // If current_arg is not empty, push it as a complete argument
                if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
            }
            _ => {
                current_arg.push(ch);
                chars.next();
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let res = parse_command_line("echo hello");
        assert!(res.is_ok());

        let args = res.unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "echo");
        assert_eq!(&args[1..], vec!["hello"])
    }

    #[test]
    fn test_parse_empty() {
        assert!(parse_command_line("").is_ok_and(|p| p.is_empty()));
        assert!(parse_command_line("   ").is_ok_and(|p| p.is_empty()));
    }

    #[test]
    fn test_single_quote_preserves_spaces() {
        let input = "echo 'hello world'";
        let res = parse_command_line(input);
        assert!(res.is_ok());

        let args = res.unwrap();
        assert_eq!(&args[0], "echo");
        assert_eq!(&args[1..], vec!["hello world"]);
    }

    #[test]
    fn test_consecutive_spaces() {
        let input = "echo hello    world";
        let res = parse_command_line(input);
        assert!(res.is_ok());

        let args = res.unwrap();
        assert_eq!(args, vec!["echo", "hello", "world"]);
    }

    #[test]
    fn test_spaces_preserved_within_quotes() {
        let input = "echo 'hello    world'";
        let res = parse_command_line(input);
        assert!(res.is_ok());

        let args = res.unwrap();
        assert_eq!(&args[0], "echo");
        assert_eq!(&args[1..], vec!["hello    world"]);
    }

    #[test]
    fn test_adjacent_quotes_are_concatenated() {
        let input = "echo 'hello''world'";
        let res = parse_command_line(input);
        assert!(res.is_ok());

        let args = res.unwrap();
        assert_eq!(args, vec!["echo", "helloworld"]);
    }

    #[test]
    fn test_empty_quotes_ignored() {
        let result = parse_command_line("echo hello''world");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "helloworld"]);
    }

    #[test]
    fn test_mixed_quoted_and_unquoted() {
        let result = parse_command_line("echo 'quoted arg' unquoted");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "quoted arg", "unquoted"]);
    }

    #[test]
    fn test_special_chars_in_quotes() {
        let result = parse_command_line("echo '$VAR' '*pattern'");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "$VAR", "*pattern"]);
    }

    #[test]
    fn test_empty_double_quotes_ignored() {
        let result = parse_command_line("echo hello\"\"world");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "helloworld"]);
    }

    #[test]
    fn test_special_chars_in_double_quotes() {
        let result = parse_command_line("echo \"$VAR\" \"*pattern\"");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "$VAR", "*pattern"]);
    }

    #[test]
    fn test_single_quotes_inside_double_quotes() {
        let result = parse_command_line("echo \"shell's test\"");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "shell's test"]);
    }

    #[test]
    fn test_separate_double_quoted_arguments() {
        let result = parse_command_line("echo \"hello\" \"world\"");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "hello", "world"]);
    }

    #[test]
    fn test_mixed_single_and_double_quotes() {
        let result = parse_command_line("echo 'single' \"double\" unquoted");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "single", "double", "unquoted"]);
    }

    #[test]
    fn test_concatenation_mixed_quotes() {
        let result = parse_command_line("echo 'hello'\"world\"'test'");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "helloworldtest"]);
    }

    #[test]
    fn test_escaped_spaces() {
        let result = parse_command_line("echo three\\ \\ \\ spaces");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "three   spaces"]);
    }

    #[test]
    fn test_escaped_space_before_unescaped_spaces() {
        let result = parse_command_line("echo before\\     after");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "before ", "after"]);
    }

    #[test]
    fn test_escaped_letter() {
        let result = parse_command_line("echo test\\nexample");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "testnexample"]);
    }

    #[test]
    fn test_escaped_backslash() {
        let result = parse_command_line("echo hello\\\\world");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "hello\\world"]);
    }

    #[test]
    fn test_escaped_single_quotes() {
        let result = parse_command_line("echo \\'hello\\'");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, vec!["echo", "'hello'"]);
    }
}
