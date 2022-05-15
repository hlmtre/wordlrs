use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 3)
        || (args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()))
    {
        wordlrs::help();
        std::process::exit(1);
    }
    // letters known in the word first (yellow)
    let good_letters: Vec<char> = args[1].chars().collect();
    // letters known NOT in the word (grey)
    let bad_letters: Vec<char> = args[2].chars().collect();
    // letters in the word, AND in the right place
    // unknown positions denoted with underscores (_)
    let mut five_letters = "_____".to_string();
    if args.len() == 4 {
        // they can otherwise be unspecified
        five_letters = args[3].to_owned();
    }
    let perfect_letters = wordlrs::get_letter_placements(five_letters);
    // go get the words
    let url = "https://raw.githubusercontent.com/tabatkins/wordle-list/main/words";
    let resp = reqwest::blocking::get(url)?;
    let status = resp.status();
    if status != reqwest::StatusCode::OK {
        println!("HTTP error: {:#?}", status);
        std::process::exit(2);
    }
    let body = resp.text();
    let v = match body {
        Ok(b) => b.split_whitespace().map(|s| s.to_string()).collect(),
        Err(_) => Vec::<String>::new(),
    };
    let matching_words = wordlrs::match_letters(perfect_letters, good_letters, bad_letters, &v);
    println!("{}", matching_words.join(", "));
    Ok(())
}
