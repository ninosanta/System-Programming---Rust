use std::io::{stdin, stdout, Write};

pub(crate) fn main_v2() {
    let mut s = String::new();
    let mut state = false;

    /* Scriviamo una struttura di tipo REPL i.e., Read Evaluate Process Loop */
    loop {
        println!("Current state is: {}", state);
        print!("Type something: ");
        stdout().flush().unwrap();  // se non flusho il S.O. mostra l'output solo dopo l'a-capo
        /* leggiamo una riga da standard input e la depositiamo dentro s */
        stdin().read_line(&mut s).unwrap();  /* .unwrap() significa "verifica sia andato tutto
                                                  * bene, altrimenti lancia un panic.
                                                  * Inoltre, read_line legge aggiungendo in fondo
                                                  * al buffer. s andrà pulita a fine ciclo. */
        match s.trim().to_ascii_lowercase().as_str() {  // tipo uno switch ma più potente
            /* match pretende che, quando cerchiamo delle espressioni, esprimiamo tutte le possibili
             * situazioni.
             * I vari casi vengono valutati in ordine */
            "on" => state = true,  // equivale a: s == "on" ? true
            "off" => state = false,
            "toggle" => state = !state,
            //_ => (),  // _ è tipo il default dello switch, () significa fai nulla
            x => (println!("unknown command '{}'", x)),  /* lego temporaneamente alla variabile
                                                               * x la stringa non matchata */
        }
        s.clear();  // spiegato su
    }
}
