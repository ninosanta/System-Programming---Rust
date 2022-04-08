/* creiamo un sotto-modulo del progetto:
 * le cose che scriviamo dentro il modulo, se non le rendiamo public non potrà vederle nessuno */
use std::thread::sleep;
use std::time::Duration;
use crate::timer::Timer;

mod timer {
    use::std::time::Instant;

    /* struttura public, ma vogliamo che i suoi campi siano privati */
    pub struct Timer {
        label: String,  // per capire a cosa fa riferimento un timer
        start: Instant,
    }

    /* occorrono dei metodi per interagire dall'esterno con i campi della struct */
    impl Timer {
        /* funzione pub che crea un'istanza della struct */
        pub fn new(label: String) -> Self {
            Timer {
                label,  // variabile chiamata come il campo -> non occorre fare label: label
                start: Instant::now(),
            }
        }
    }

    /* di base, di costruire un distruttore non ci importa molto perché la cosa importante è
     * liberare la memoria e questo rust lo fa già di suo.
     * Però noi vogliamo anche vedere, alla distruzione, che ore sono e stampare una stringa
     * -> implementiamo il _Tratto_ drop */
    impl Drop for Timer {
        fn drop(&mut self) {
            println!("Elapsed time for {}: {:?}", self.label, Instant::now() - self.start);
        }
    }
}

fn main() {
    let _t: Timer = Timer::new("l1".to_string());
    for i in 1..=5 {
        let _t2: Timer = Timer::new(format!("{}{}", "l", i));
        sleep(Duration::new(1, 0));
        println!("Ciclo #{}", i);
        if i == 3 {
            panic!("Forced crash");
            /* vedremo che _t1 e _t2 verranno comunque distrutti :) */
        }
    }
}
