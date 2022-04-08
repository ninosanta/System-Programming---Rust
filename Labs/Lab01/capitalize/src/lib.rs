pub fn capitalize(s: &str) -> String {
    let mut s1 = String::new();
    let mut first: bool = true;

    for ch in s.chars() {
        match ch {
            c if c.is_alphabetic() => {  // alphabetic incliude anche gli alfabeti stranieri
                if first == true {
                    s1 += c.to_uppercase()
                        .to_string()  /* perché to_uppercase può ritornare più caratteri e.g.,
                                             * il to_uppercase di ß è SS */
                        .as_str();  // l'operatore + mi sa che vuole degli slice
                    first = false;
                } else {
                    s1.push(c);
                }
            },
            ' ' => {
                if first == false {
                    first = true;
                }
                s1.push(ch);
            },
            // None => String::new(),
            _ => s1.push(ch),
        }
    }
    return s1;
}

/* https://stackoverflow.com/a/38406885 */
/*
fn capitalize_word(w: &str) -> String {

    let mut chars = w.chars();

    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

    for str in s.split(" ") {
        match str {
            " " => s1.push(' '),
            _ => s1 += capitalize_word(&str).as_str(),
        }
    }

    non funzionava perché si mangiava gli spazi
*/