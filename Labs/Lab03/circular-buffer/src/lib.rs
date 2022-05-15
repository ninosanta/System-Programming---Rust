pub struct CircularBuffer<T> {
    head: usize,
    tail: usize,
    items: Vec<Option<T>>
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            head: 0,
            tail: 0,
            items: (0..capacity).map(|_x| None).collect(),
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        /* write the element in the tail
         * if in the tail there is still something, then buffer is full! */
        if !self.is_full() {
            self.overwrite(_element);
            Ok(())
        } else {
            Err(Error::FullBuffer)
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        self.items[self.head]
            .take()  // takes the value out of the option, leaving a None in its place
            .ok_or(Error::EmptyBuffer)  // transforms the Option<T> into a [Result<T, E>],
                                            // mapping Some(v) to Ok(v) and None to Err(err).
            .map(|x| {
                self.increase_head();
                x
            })
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.items = (0..self.items.capacity()).map(|_x| None).collect();
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.is_full() {
            /* tail overwriting: but tail when it is full is equal to the head */
            self.items[self.head] = Some(_element);
            /* head update */
            self.increase_head();
        } else {
            self.items[self.tail] = Some(_element);
            self.increase_tail();
        }
    }

    fn is_full(&self) -> bool {
        self.items[self.tail].is_some()
    }

    fn increase_head(&mut self) {
        self.head = (self.head + 1) % self.items.capacity();
    }

    fn increase_tail(&mut self) {
        self.tail = (self.tail + 1) % self.items.capacity();
    }
}
