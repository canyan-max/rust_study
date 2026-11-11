use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        } else {
            continue;
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    let lower_query: String = query.to_lowercase();

    for line in contents.lines() {
        let lower_line: String = line.to_lowercase();
        if lower_line.contains(lower_query.as_str()) {
            result.push(line);
        }
    }
    result
}

#[allow(dead_code)]
fn search_with_line_number<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut ret: Vec<(usize, &'a str)> = Vec::new();
    let lower_query: String = query.to_lowercase();
    for (number, line) in contents.lines().enumerate() {
        let lower_line: String = line.to_lowercase();
        if lower_line.contains(lower_query.as_str()) {
            ret.push((number + 1, line));
        }
    }
    ret
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    if args.len() != 3 {
        return Err("input paramter err".to_string());
    }
    let minigrep_paramter: Config = Config {
        query: args[1].clone(),
        file_path: args[2].clone(),
    };
    Ok(minigrep_paramter)
}

fn search_file(config: &Config) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(&config.file_path)?;
    let case_sensitive = env::var("CASE_SENSITIVE").is_ok();
    let ret = if case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in &ret {
        println!("{}", line);
    }

    Ok(())
}

pub fn minigrep_start() {
    let args: Vec<String> = env::args().collect();

    let minigrep_paramter: Config = match parse_args(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    if let Err(error) = search_file(&minigrep_paramter) {
        eprintln!("read file error：{}", error);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::parse_args;
    use super::search;
    use super::search_case_insensitive;
    use super::search_with_line_number;
    #[test]
    fn test_search() {
        let expected: Vec<String> = vec![
            "AT32 is running.".to_string(),
            "AT32 sends data.".to_string(),
        ];
        let contents: &str = "AT32 is running.\nUART is ready.\nAT32 sends data.";
        let query: &str = "AT32";
        let ret: Vec<&str> = search(query, contents);
        assert_eq!(ret, expected);
    }

    #[test]
    fn test_search_no_match() {
        let contents: &str = "TTFE is running.\nUART is ready.\nAT32 sends data.";
        let query: &str = "AABC";
        let ret: Vec<&str> = search(query, contents);
        assert!(ret.is_empty());
    }
    #[test]
    fn test_loworhigh() {
        let expected: Vec<String> = vec![
            "AT32 is running.".to_string(),
            "AT32 sends data.".to_string(),
        ];
        let contents: &str = "AT32 is running.\nUART is ready.\nAT32 sends data.";
        let query: &str = "aT32";
        let ret: Vec<&str> = search_case_insensitive(query, contents);
        assert_eq!(ret, expected);
    }
    #[test]
    fn test_parse_args_default() {
        use super::Config;
        let args: Vec<String> = vec![
            "serial_rust.exe".to_string(),
            "AT32".to_string(),
            "sample.txt".to_string(),
        ];
        let minigrep_paramter: Config = match parse_args(&args) {
            Ok(config) => config,
            Err(error) => {
                panic!("合法参数解析失败：{}", error);
            }
        };
        assert_eq!(minigrep_paramter.query, "AT32");
        assert_eq!(minigrep_paramter.file_path, "sample.txt");
    }
    #[test]
    fn test_parse_args_too_few() {
        use super::Config;
        let args: Vec<String> = vec!["serial_rust.exe".to_string(), "AT32".to_string()];
        let result: Result<Config, String> = parse_args(&args);
        assert!(result.is_err());
    }
    #[test]
    fn test_search_query_scope() {
        let contents: String = String::from("AT32 is running.\nUART is ready.");
        let ret: Vec<&str>;
        {
            let query: String = String::from("AT32");
            ret = search(&query, &contents);
        }
        assert_eq!(ret, vec!["AT32 is running."]);
    }
    #[test]
    fn test_search_with_line_number() {
        let contents: &str = "AT32 is running.\nUART is ready.\nat32 sends data.";
        let expected: Vec<(usize, &str)> = vec![(1, "AT32 is running."), (3, "at32 sends data.")];
        let ret: Vec<(usize, &str)> = search_with_line_number("aT32", contents);

        assert_eq!(ret, expected);
    }
}
