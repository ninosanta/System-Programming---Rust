use std::io::{stdin, stdout, Write};

pub(crate) fn main_v1() {
    let mut s = String::new();

    /* Scriviamo una struttura di tipo REPL i.e., Read Evaluate Process Loop */
    loop {
        print!("Type something: ");
        stdout().flush().unwrap();  // se non flusho il S.O. mostra l'output solo dopo l'a-capo
        /* leggiamo una riga da standard input e la depositiamo dentro s */
        stdin().read_line(&mut s).unwrap();  /* .unwrap() significa "verifica sia andato tutto
                                                  * bene, altrimenti lancia un panic.
                                                  * Inoltre, read_line legge aggiungendo in fondo
                                                  * al buffer. s andrà pulita a fine ciclo. */
        println!(">{}<", s.trim());  /* tra virgolette per vedere dove inizia e finisce la stringa
                                      * con il trim() togliamo gli spazi iniziali e finali */
        s.clear();  // spiegato su
    }
}
