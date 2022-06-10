/** The objective of this exercise is to built a Packaged Task.
 * A Packaged Task is an abstraction that exists in C++ but that does not exist in Rust. It is an
 * object that encapsulates a Lambda i.e., it offers a method to throw a Lambda and returns the
 * return value of the lambda to another Object called Future that is thought to read the result
 * of the Task i.e., they are separated, through a get() that is a blocking method or a try_get().
 *
 * A Packaged Task is useful whenever we want to create a thread pool.
 **/

mod pt {
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use std::panic::UnwindSafe;
    use std::sync::mpsc::{channel, Receiver, TryRecvError};
    use crate::pt::AsyncResult::{Ongoing};

    pub enum AsyncResult<A, B> {
        Ongoing(A),
        Done(B),
    }

    #[derive(Debug)]
    pub struct PTError {
        message: String,
    }

    impl PTError {
        fn new(msg: &str) -> Self {
            PTError{
                message: msg.to_string(),
            }
        }
    }

    impl Display for PTError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "PTError: {}", self.message)
        }
    }

    impl Error for PTError {

    }

    type Result<T> = std::result::Result<T, PTError>;  // to avoid to make functions return the type of the Error

    /** Future **/
    pub struct Future<T: Send> {
        rx: Receiver<Result<T>>,
    }

    impl<T: Send> Future<T> {
        fn new(rx: Receiver<Result<T>>) -> Self {  // not public!
            Future { rx }
        }

        pub fn get(self) -> Result<T> {  // self and not &self! because we want to consume it!
            let r = self.rx.recv();
            if r.is_err() {
                Err(PTError::new("PackagedTask has been dropped"))
            } else {
                r.unwrap()
            }
        }

        pub fn try_get(self) -> AsyncResult<Self, Result<T>> {
            let r = self.rx.try_recv();

            if r.is_err() {
                match r.err().unwrap() {
                    TryRecvError::Empty => return Ongoing(self),
                    TryRecvError::Disconnected => return AsyncResult::Done(
                        Err(PTError::new("PackagedTask has been dropped"))),
                }
            } else {  // r is ok
                return AsyncResult::Done(r.unwrap());
            }
        }
    }

    /** PackagedTask **/
    pub struct PackagedTask<Args> {
        f: Box<dyn FnOnce(Args)>  // Box of the most general function, of dynamic dimension, called just once
    }

    unsafe impl<Args> Send for PackagedTask<Args>  {}

    impl<Args> PackagedTask<Args> {
        fn new(f: Box<dyn FnOnce(Args)>) -> Self {
            PackagedTask{ f }
        }

        pub fn execute(self, a: Args) {
            (self.f)(a);  // take the function self.f and pass 'a' to it
        }
    }

    pub fn packaged_task<F, Args, T: 'static + Send>(f: F) -> (PackagedTask<Args>, Future<T>)
        where
            F: FnOnce(Args) -> T + UnwindSafe + 'static,  // UnwindSafe to avoid that a panic kill everything
            Args: UnwindSafe,
    {
        /* channel to make the lambda communicate with the rest */
        let (tx, rx) = channel::<Result<T>>();

        /* lambda */
        let g = move| a: Args |  {
            /* f can break itself -> we call it through a wrapper called catch_unwind */
            let res = std::panic::catch_unwind(|| f(a))  // || because catch_unwind does not want functions with args
                .map_err(|_| PTError::new("Lambda panicked"));
            tx.send(res).unwrap_or(());  // send fails -> return void
        };

        let pt = PackagedTask::new(Box::new(g));  // what we send
        let f = Future::new(rx);

        (pt, f)
    }
}

mod test {
    use std::thread::sleep;
    use std::time::Duration;
    use crate::pt::{AsyncResult, packaged_task};

    #[test]
    fn basic_behavior() {
        /* PackagedTask that encapsulates a lambda that takes an i32 and increments it */
        let (pt, f) = packaged_task(|a: i32| a + 1);
        pt.execute(5);

        assert_eq!(f.get().unwrap(), 6);
    }

    #[test]
    fn pt_execution_in_a_remote_thread() {
        let (pt, f) = packaged_task(|a: i32| a + 1);
        /* remote thread */
        let h = std::thread::spawn(move || {
            sleep(Duration::from_millis(50));
            pt.execute(5);
        });
        /* main thread: get called immediately */
        assert_eq!(f.get().unwrap(), 6);
        h.join().unwrap();
    }

    /* vice versa */
    #[test]
    fn f_collection_in_remote_thread() {
        let (pt, f) = packaged_task(|a: i32| a + 1);

        let h = std::thread::spawn(move || {
            assert_eq!(f.get().unwrap(), 6);
        });

        pt.execute(5);
        h.join().unwrap();
    }

    #[test]
    fn panicking_function_returns_an_error() {
        let (pt, f) = packaged_task(|_| {
            /* callback that panics after slept */
            sleep(Duration::from_millis(50));
            panic!("Panic");
        });
        pt.execute(());  // try to execute the callback
        assert!(f.get().is_err());
    }

    #[test]
    fn dropped_pt_does_not_cause_a_panic() {
        let (pt, f) = packaged_task(|a: i32| a + 1);
        drop(pt);  // pt is dropped
        assert!(f.get().is_err());
    }

    #[test]
    fn dropped_future_does_not_cause_a_panic() {
        let (pt, f) = packaged_task(|a: i32| a + 1);
        drop(f);
        pt.execute(5);
        assert!(true);  // this point of the program has been reached
    }

    #[test]
    fn try_getting() {
        let (pt, mut f) = packaged_task(|a: i32| a + 1);
        match f.try_get() {
            AsyncResult::Ongoing(f1) => { f = f1; }  // the ongoing contains the future
            _ => { unreachable!() }
        }
        pt.execute(5);
        match f.try_get() {
            AsyncResult::Done(Ok(x)) => { assert_eq!(x, 6); }
            _ => { assert!(false); }
        }
    }

    /* cycling try_get */
    #[test]
    fn try_async_get() {
        let (pt, f) = packaged_task(|a: i32| a + 1);
        let h = std::thread::spawn(move || {
            let mut f1 = f;  // copy the future returned by the PT
            loop {
                match f1.try_get() {
                    AsyncResult::Ongoing(f2) => f1 = f2,  // future update
                    AsyncResult::Done(Ok(v)) => return v,
                    _ => unreachable!(),  // error
                }
                sleep(Duration::from_millis(10));
            }
        });
        sleep(Duration::from_millis(50));
        pt.execute(5);
        assert_eq!(h.join().unwrap(),6);
    }

    #[test]
    fn dropping_pt_try_async_get() {
        let (pt, f) = packaged_task(|a: i32| a + 1);
        drop(pt);
        match f.try_get() {
            AsyncResult::Done(r) => assert!(r.is_err()),
            _ => unreachable!()
        };
    }

}

fn main() {
    println!("Hello, world!");
}
