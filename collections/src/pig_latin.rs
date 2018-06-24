pub fn pig_latin(string: &str) -> String {
    let lower = string.to_lowercase();
    let split = lower.split_whitespace();
    let mut result = String::new();

    for word in split {
        result.push_str(&match word.chars().next().unwrap() {
            'a' | 'e' | 'i' | 'o' | 'u' => format!(" {}-hay", word),
            _ => format!(" {}-{}ay", str_car_cdr(word).1, str_car_cdr(word).0),
        })
    }
    result.trim().to_string()
}

fn str_car_cdr(s: &str) -> (&str, &str) {
    for i in 1..5 {
        let r = s.get(0..i);
        match r {
            Some(x) => return (x, &s[i..]),
            None => (),
        }
    }

    (&s[0..0], s)
}

#[cfg(test)]
mod tests {
    use pig_latin::pig_latin;

    #[test]
    fn it_moves_first_consonants_to_end() {
        assert_eq!("ello-hay orld-way", pig_latin("hello world"));
    }

    #[test]
    fn it_adds_hay_to_end_of_words_with_vowel() {
        assert_eq!(
            "y-may ame-nay is-hay apples-hay and-hay oranges-hay",
            pig_latin("my name is apples and oranges")
        );
    }
}
