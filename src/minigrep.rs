use std::env;
use std::fs;
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
pub fn minigrep_run() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("err log");
        println!("use way:cargo run keyword filepath");
        return;
    }
    let query: &String = &args[1];
    let file_name: &String = &args[2];
    println!("keyword:{}", query);
    println!("file_name:{}", file_name);
    let result: Result<String, std::io::Error> = fs::read_to_string(file_name);

    match result {
        Ok(contents) => {
            // read info of keyword and printf
            let ret: Vec<String> = search(&query, &contents);
            for line in &ret {
                println!("{}", line);
            }
        }
        Err(error) => {
            println!("read file is err ：{}", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::search;

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
        let query: &str = "TTFE";
        let ret: Vec<String> = search(query, contents);
        assert!(ret.is_empty());
    }
}
