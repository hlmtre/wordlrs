use std::{collections::HashMap, env};

fn match_letters(
    perfect_letters: HashMap<u8, char>,
    good_letters: Vec<char>,
    bad_letters: Vec<char>,
    #[allow(clippy::ptr_arg)] target_strings: &Vec<String>,
) -> Vec<String> {
    let mut our_target_words = target_strings.clone();
    for (k, v) in &perfect_letters {
        our_target_words.retain(|i| i.chars().nth(usize::from(*k)).unwrap() == *v);
    }

    for l in good_letters {
        our_target_words.retain(|i| i.contains(l));
    }
    for l in bad_letters {
        our_target_words.retain(|i| !i.contains(l));
    }
    return our_target_words;
}

fn get_letter_placements(five: String) -> HashMap<u8, char> {
    let mut position = 0;
    let mut placed_letters = HashMap::<u8, char>::new();
    for l in five.chars() {
        if l == '_' {
            position += 1;
            continue;
        } else {
            placed_letters.insert(position, l);
        }
    }
    return placed_letters;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let good_letters: Vec<char> = args[1].chars().collect();
    let bad_letters: Vec<char> = args[2].chars().collect();
    let perfect_letters = get_letter_placements(args[3].to_owned());
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
    let matching_words = match_letters(perfect_letters, good_letters, bad_letters, &v);
    println!("{:#?}", matching_words);
    Ok(())
}
