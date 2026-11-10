use std::env;
use std::fs;
struct Config {
    query: String,
    file_path: String,
    is_sensitive: bool,
}

fn search(query: &str, contents: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.to_string());
        } else {
            continue;
        }
    }
    return result;
}

fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let lower_query: String = query.to_lowercase();

    for line in contents.lines() {
        let lower_line: String = line.to_lowercase();
        if lower_line.contains(lower_query.as_str()) {
            result.push(line.to_string());
        }
    }
    return result;
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    if args.len() < 3 || args.len() > 4 {
        return Err("input paramter err".to_string());
    }
    let mut minigrep_paramter: Config = Config {
        query: args[1].clone(),
        file_path: args[2].clone(),
        is_sensitive: false,
    };
    if let Some(option) = args.get(3) {
        if option == "--case-sensitive" {
            minigrep_paramter.is_sensitive = true;
        } else {
            return Err("input arg index 3 err".to_string());
        }
    }
    return Ok(minigrep_paramter);
}

pub fn minigrep_run() {
    let args: Vec<String> = env::args().collect();
    let minigrep_paramter: Config = match parse_args(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };
    let result: Result<String, std::io::Error> = fs::read_to_string(&minigrep_paramter.file_path);
    match result {
        Ok(contents) => {
            // read info of keyword and printf
            let ret: Vec<String>;
            if false == minigrep_paramter.is_sensitive {
                ret = search_case_insensitive(&minigrep_paramter.query, &contents);
            } else {
                ret = search(&minigrep_paramter.query, &contents);
            }
            for line in &ret {
                println!("{}", line);
            }
        }
        Err(error) => {
            eprintln!("读取文件失败：{}", error);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_args;
    use super::search;
    use super::search_case_insensitive;
    #[test]
    fn test_search() {
        let expected: Vec<String> = vec![
            "AT32 is running.".to_string(),
            "AT32 sends data.".to_string(),
        ];
        let contents: &str = "AT32 is running.\nUART is ready.\nAT32 sends data.";
        let query: &str = "AT32";
        let ret: Vec<String> = search(query, contents);
        assert_eq!(ret, expected);
    }

    #[test]
    fn test_search_no_match() {
        let contents: &str = "TTFE is running.\nUART is ready.\nAT32 sends data.";
        let query: &str = "AABC";
        let ret: Vec<String> = search(query, contents);
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
        let ret: Vec<String> = search_case_insensitive(query, contents);
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
        assert!(!minigrep_paramter.is_sensitive);
    }
    #[test]
    fn test_parse_args_illger() {
        use super::Config;
        let args: Vec<String> = vec![
            "serial_rust.exe".to_string(),
            "AT32".to_string(),
            "sample.txt".to_string(),
            "--abc.txt".to_string(),
        ];
        let result: Result<Config, String> = parse_args(&args);
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_args_too_few() {
        use super::Config;
        let args: Vec<String> = vec!["serial_rust.exe".to_string(), "AT32".to_string()];
        let result: Result<Config, String> = parse_args(&args);
        assert!(result.is_err());
    }
    #[test]
    fn test_parse_args_is_ok() {
        use super::Config;
        let args: Vec<String> = vec![
            "serial_rust.exe".to_string(),
            "AT32".to_string(),
            "sample.txt".to_string(),
            "--case-sensitive".to_string(),
        ];
        let minigrep_paramter: Config = match parse_args(&args) {
            Ok(config) => config,
            Err(error) => {
                panic!("合法参数解析失败：{}", error);
            }
        };
        assert!(minigrep_paramter.is_sensitive);
    }
}
