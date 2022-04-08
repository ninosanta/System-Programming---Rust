fn main() {
    let mut i = 32;

    println!("i: {}", i);

    /* creiamo un riferimento non mutabile a i */
    let r = &i;  /* questa cossa in C suonerebbe come r puntatore ad i
                       * qui è praticamente vero, pero' r non ha tipo puntatore
                       * bensi' &i32 */
    //i = i + 1;  // non si puo' fare perché i e' stato preso in prestito da r che e' immutabile
    println!("*r: {}", *r);

    i = i + 1;  // qui pero' posso farlo perche' r non sta usando la zona di memoria puntata

    let pippo = &i;  // OSS: e' possibile fare quanti reference non mutabili si vuole
    println!("pippo: {}", pippo);


    /* creiamo un riferimento mutabile a i */
    let s = &mut i;
    //let s2 = &mut i;  // non si puo' fare perche' c'e' gia' un riferimento mutabile!
    //let s2 = &i;  // nemmeno questo si puo' fare per lo stesso motivo.
    *s += 1;
    println!("Indirizzo: {:p}, *s: {}", s, *s);

}
