// Convert strings to pig latin. 
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
// Keep in mind the details about UTF-8 encoding!

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn pig_latin() {
    let phrase = String::from("hello I am feeling very good today");

    let mut words_vector = split_words_lowercase(&phrase);

    for word in &mut words_vector {
    *word = convert_to_pig_latin(word);
    }

    let new_phrase = words_vector.join(" ");

    println!("{}", new_phrase);
}

fn split_words_lowercase(phrase: &str) -> Vec<String> {
    phrase
    .to_lowercase()
    .split_whitespace()
    .map(String::from)
    .collect()
}

fn convert_to_pig_latin(word: &str) -> String {
    let first_character: char = word.chars().next().unwrap();
    let word_no_first_character: String = word.chars().skip(1).collect();

    if VOWELS.contains(&first_character)  {
        format!("{word}-hay")
    } else {
        format!("{word_no_first_character}-{first_character}ay")
    }
}
