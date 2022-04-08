use capitalize::capitalize;

#[test]
fn more_than_a_word() {
    let string = capitalize("questa e una frase ß");
    assert_eq!(string, "Questa E Una Frase SS");
}

#[test]
fn just_a_word() {
    let string = capitalize("questaEunaFraseß");
    assert_eq!(string, "QuestaEunaFraseß");
}

#[test]
fn more_than_an_emphasized_word() {
    let string = capitalize("àuesta è ùna ìrase ùß");
    assert_eq!(string, "Àuesta È Ùna Ìrase Ùß");
}

#[test]
fn empty_string() {
    let string = capitalize("");
    assert!(string.is_empty());
}

#[test]
fn more_than_a_space_among_words() {
    let string = capitalize("  questa  è  una   frase ß  ");
    assert_eq!(string, "  Questa  È  Una   Frase SS  ")
}