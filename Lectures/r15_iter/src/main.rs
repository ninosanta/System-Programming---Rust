fn main() {
    let mut v: Vec<String> = vec!["alpha".to_string(), "beta".to_string(),
                                  "gamma".to_string(), "delta".to_string()];

    /* quando facciamo:
     * for s in &v {}
     * il compilatore lo trasforma in: */
    for s in v.iter() {  // facendo v.iter() si ottiene un reference al dato contenuto
        println!("{}", s);
    }

    /* quando facciamo:
     * for s in &mut v {}
     * il compilatore lo trasforma in: */
    for s in v.iter_mut() {
        s.push_str("-1");
    }
    println!("{:?}", v);

    /* quando facciamo:
     * for s in v {}
     * il compilatore lo trasforma in: */
    for s in v.into_iter() {
        println!("{}", s);  // into_iter() consuma gli elementi! -> s li prende, li usa quid e li butta
    }

    /* Il tratto Iterator possiede il metodo into_iter() implementato di default: */
    let mut vec = vec![String::from("a"), String::from("b"), String::from("c")];
    let it = vec.iter_mut();
    for s in it {  // quell'it è in realtà un it.into_iter()
        s.push_str("-1");
    }
    println!("{:?}", vec);
}
