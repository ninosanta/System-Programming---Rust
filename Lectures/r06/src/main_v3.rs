use std::io::{stdin, stdout, Write};

pub(crate) fn main_v3() {
    let mut s = String::new();

    /* Scriviamo una struttura di tipo REPL i.e., Read Evaluate Process Loop */
    loop {
        print!("Type something: ");
        stdout().flush().unwrap();
        s.clear();  // spiegato giù

        /* leggiamo una riga da standard input e la depositiamo dentro s */
        stdin().read_line(&mut s).unwrap();

        /* vogliamo spezzare la riga in più pezzi e lo mettiamo in un Vec di stringhe finale.
         * Lo spazio identifica un pezzo nuovo */
        let v : Vec<&str> = s.trim()
            .split(" ")
            .filter(| s | { *s != "" })  /* l'equivalente del Javascript
                                                 * .filter(s => s != "") */
            .collect();
        println!("v: {:?}", v);  // metodo di dbg, non è printable altrimenti

        /* per comodità usiamo uno slice di v coincidente con v */
        let s : &[&str] = &v[..];  /* s sarà un Fat Pointer contenente il puntatore al buffer di v e
                                    * il numero di elementi */
        match s {  // match sa anche confrontare strutture
            /* casi valutati in ordine */
            [] => println!("empty line"),
            ["!", ..] => println!("comment"),  // tutte le frasi che iniziano con ! sono commenti
            ["inc", x] if x.parse::<i32>()
                                 .is_ok() => println!("increment"), // se x è convertibile a numero
            [x] => println!("single element: {}", x),
            [x, y] => println!("two elements: {}, {}", x, y),
            err => println!("unknown command: '{:?}'", err),
        }

        /* esempio di assegnazione con match */
        let item = 69;
        let i = match item {
            0 => { "zero" },
            10 ..= 69 => { "sixtynine" },
            40 | 80 => { "fourty or eighty" },
            _ => { "JFC" },
        };  // assegnazione, ";" necessario
        println!("item: {}", i);

    }
}
