fn main() {
    let sentence = "Hello my dear adorable Anna";
    let sentence = pig_latin_sentence(sentence);
    println!("{sentence}");
}


fn pig_latin_sentence(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(pig_latin)
        .collect::<Vec<_>>()
        .join(" ")
}


fn pig_latin(word: &str) -> String {
    let lowercase = word.to_lowercase();
    let mut chars = lowercase.chars();
    let first = chars.next().unwrap();
    let rest: String = chars.collect();
    match first {
        'a' | 'e'| 'i' | 'o' | 'u' => format!("{word}-hay"),
        _ => format!("{rest}-{first}ay"),
    }
}