use my_cycle::{MyCycle};

#[test]
fn it_works() {
    let v = vec![1, 2, 3];
    let c1 =  MyCycle::new(v.iter(), 3);
    assert_eq!(c1.eq(&[1, 2, 3, 1, 2, 3, 1, 2, 3]), true)
}

#[test]
fn empty_iterator_as_input() {
    let v: Vec<u8> = Vec::new();
    let c1 = MyCycle::new(v.iter(), 0);
    assert_eq!(c1.count(), 0)
}

#[test]
fn cycle_from_a_cycle_product_of_elements_number() {
    let v = vec![1, 2, 3];
    let c1 = MyCycle::new(v.iter(), 3);
    let c2 = MyCycle::new(c1, 2);
    assert_eq!(c2.count(), 18);  // n1 * n2 * v.len() = 3 * 2 * 3
}

#[test]
fn chained_cycles() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2];
    let c1 = MyCycle::new(v1.iter().clone(), 3);
    let c2 = MyCycle::new(v2.iter().clone(), 2);
    assert_eq!(c1.chain(c2).count(), 13); // 3*3 + 2*2
}

#[test]
pub fn zipped_cycles_are_ordered_pairs() {
    let v = vec![1, 2, 3];
    let c1 = MyCycle::new(v.iter().clone(), 4);
    let c2 = MyCycle::new(v.iter().clone(), 4);
    c1.zip(c2).for_each( |(x, y)| assert_eq!(x, y));
}