/// Soluzione non ottima proposta dal professore seguendo un approccio imperativo di programmazione
/// poiché più simile a quanto siamo abituati finora.

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    //unimplemented!("Is {:?} a valid ISBN number?", isbn);
    let mut t = 0;  // totale
    let mut w = 10;  // peso

    /* passiamo dall'UTF-8 della stringa, al singolo char, scandendola char by char */
    for ch in isbn.chars() {
        match ch {
            '-' => continue,
            'x' | 'X' if w == 1 => t += 10 * 1,
            '0'..='9' => t += ch.to_digit(10).unwrap() * w,  /* unwrap "sbusta" il valore
                                                                 * ritornato da to_digit() perché
                                                                 * esso ritorna un oggetto che deve
                                                                 * essere testato (perché to_digit
                                                                 * potrebbe fallire) e poi sbustato.
                                                                 * In questo caso noi però sappiamo
                                                                 * di avergli passato sicuramente un
                                                                 * numero tra 0 e 9 e quindi
                                                                 * possiamo sbustare direttamente */
            _ => return false,
        }
        if w == 0 {  // se la stringa fosse troppo lunga si finirebbe sotto 0 con il peso!
            return false;
        }
        w -= 1;
    }
    t % 11 == 0 && w == 0
}
