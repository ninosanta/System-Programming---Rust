use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]  // in some simple cases PartialEq can be derivate too
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        let p = Point{x, y};
        // println!("Creating Point at address {:p}", &p);
        p  // return value
    }

    fn swap(self) -> Self {
        Point { x: self.y, y: self.x }
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping point at address {:p}", self);
    }
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {  // p == 3 ?
        self.x == other.x && self.y == other.y
        // this won't work with 3 == p
    }
}

impl PartialEq<Point> for i32 {
    fn eq(&self, other: &Point) -> bool {
        // self == other  NOOOOOOOOOOOOOO
        other == self
    }
}

impl PartialEq<i32> for Point {
    fn eq(&self, &other: &i32) -> bool {
        self.x + self.y == other
    }
}

/* thought as a stringify for programmers
 * implemented above through the macro #[derive()] */
// impl Debug for Point {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Point")
//             .field("x", &self.x)
//             .field("y", &self.y)
//             .finish()
//     }
// }

/* thought as a stringify for users */
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p1: Point = Point::new(1, 2);
    let p2: Point = Point::new(2, 1);

    println!("p1: {}, p2: {}", p1, p2);

    if p1 == p2 {  // to do this check the trait 'PartialEq' must be implemented
        println!("p1: {:?}, is equal p2: {:?}!", p1, p2);
    } else {
        println!("p1: {:?}, is not equal p2: {:?}!", p1, p2);
    }

    let mut p3 = Point::new(2, 1);
    p3 = p3.swap();
    if p1 == p3 {
        println!("p1: {:?}, is equal p3: {:?}!", p1, p3);
    } else {
        println!("p1: {:?}, is not equal p3: {:?}!", p1, p3);
    }

    drop(p3);  // we can drop resources before the end of their scope

    if p1 == 3 {
        println!("The sum of the fields of p1: {:?}, is equal to 3", p1);
    } else {
        println!("The sum of the fields of p1: {:?}, is not equal to 3", p1);
    }

    if 3 == p1 {
        println!("The sum of the fields of p1: {:?}, is equal to 3", p1);
    } else {
        println!("The sum of the fields of p1: {:?}, is not equal to 3", p1);
    }

}
