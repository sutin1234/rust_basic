use regex::Regex;

fn main() {
    let valid_date = "2023-02-18";
    let invalid_date = "23-02-18";
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("my date is matched: {}", re.is_match(&valid_date));
    println!("my date is matched: {}", re.is_match(&invalid_date));
}
