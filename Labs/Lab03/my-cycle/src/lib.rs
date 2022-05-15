use std::fmt::Debug;
#[derive(Debug, Clone)]
pub struct MyCycle<I: Clone + Iterator + Debug> {
    src: I,  // source iterator, it will be used to start over the iterations
    iter: I,
    repeat: usize,
    inf: bool,
}

impl<I: Clone+Iterator+Debug> MyCycle<I> {
    pub fn new(iter: I, repeat: usize) -> MyCycle<I> {
        MyCycle {
            src: iter.clone(),
            iter,
            repeat,
            inf: match repeat {
                0 => true,
                _ => false,
            }
        }
    }
}

impl<I> Iterator for MyCycle<I> where I: Clone + Iterator + Debug, {
    type Item = <I as Iterator>::Item;  // I::Item would have been good as well

    fn next(&mut self) -> Option<Self::Item> {
        /* our next is iter.next() */
        match self.iter.next() {
            None if self.repeat == 1 /* 1 because we are when the cycle has already finished
                                      * with 0 we'll cycle one more time! */
                && self.inf == false => None,
            None => {
                /* time to restart */
                self.iter = self.src.clone();  // restoring the iterator...
                if self.inf == false { self.repeat -= 1; }
                self.iter.next()  // return value
            },
            Some(i) => Some(i),
        }
    }
}
