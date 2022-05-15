use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub fn match_letters(
    perfect_letters: HashMap<u8, char>,
    good_letters: Vec<char>,
    bad_letters: Vec<char>,
    #[allow(clippy::ptr_arg)] target_strings: &Vec<String>,
) -> Vec<String> {
    let mut our_target_words = target_strings.clone();
    // TODO i guess this is all really ineffecient
    for (k, v) in &perfect_letters {
        // this one's confusing. here you go:
        // get_letter_placements turns __i__ into a HashMap with contents
        // { 2: 'i' }. we check each entry in that map to the target (possible) words.
        // each word that DOES NOT have the known good letter in that position in its
        // 'array' of characters is removed from the list of possible words.
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

pub fn get_letter_placements(five: String) -> HashMap<u8, char> {
    let mut placed_letters = HashMap::<u8, char>::new();
    for (position, l) in five.chars().enumerate() {
        if l != '_' {
            placed_letters.insert(position.try_into().unwrap(), l);
        }
    }
    return placed_letters;
}

pub fn save_temp_file(content: String) -> Result<(), Box<dyn std::error::Error>> {
    let dir = env::temp_dir();
    // read, write, and create if not exists
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(dir.join("wordlrs.txt"))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn get_temp_contents() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open(env::temp_dir().join("wordlrs.txt"))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let word_vector = content.split_whitespace().map(|s| s.to_string()).collect();
    Ok(word_vector)
}

pub fn help() {
    let help_string = "
    [wordlrs] a wordle solver

    Usage: 
        wordlrs <good letters> <bad letters> <perfect letters>

        Good letters are letters that are in the word, but whose position is unknown.
        Bad letters are explicitly not in the word.
        Perfect letters are the correct letter, and their location is known.
        Use underscores to indicate an unknown letter at that position, like so:
            _a_n_ would match 'paint' or 'tawny'.


    Example:
        wordlrs drit slaenkcomfyu __i__
        ";
    println!("{}", help_string);
}
