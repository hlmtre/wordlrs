use std::collections::HashMap;
pub fn match_letters(
    perfect_letters: HashMap<u8, char>,
    good_letters: Vec<char>,
    bad_letters: Vec<char>,
    #[allow(clippy::ptr_arg)] target_strings: &Vec<String>,
) -> Vec<String> {
    let mut our_target_words = target_strings.clone();
    // TODO i guess this is all really ineffecient
    for (k, v) in &perfect_letters {
        our_target_words.retain(|i| i.chars().nth(usize::from(*k)).unwrap() == *v);
    }

    for l in good_letters {
        // retain is pretty fast
        // it uses unsafe under the hood and shifts
        // array elements to fill spaces left after removing not-matchers
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