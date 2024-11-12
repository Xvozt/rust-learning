// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!

fn main() {
    let input1 = ("hello world", "ello-hay orld-way");
    let input2 = ("apple orange", "apple-hay orange-hay");
    let input3 = ("a i o", "a-hay i-hay o-hay");
    let input4 = ("hello apple", "ello-hay apple-hay");
    let input5 = ("my gym", "y-may ym-gay");

    assert_eq!(input1.1, string_converter(input1.0));
    assert_eq!(input2.1, string_converter(input2.0));
    assert_eq!(input3.1, string_converter(input3.0));
    assert_eq!(input4.1, string_converter(input4.0));
    assert_eq!(input5.1, string_converter(input5.0));
}

fn string_converter(initial: &str) -> String {
    let mut converted = String::new();
    for word in initial.split_whitespace() {
        let pig_lain_word = match word.chars().next() {
            Some(first_char) if "aeiouAEIOU".contains(first_char) => {
                format!("{}-hay", word)
            }
            Some(first_char) => {
                let remaining = &word[first_char.len_utf8()..];
                format!("{}-{}ay", remaining, first_char)
            }
            None => continue,
        };

        if !converted.is_empty() {
            converted.push(' ');
        }
        converted.push_str(&pig_lain_word);
    }
    converted
}
