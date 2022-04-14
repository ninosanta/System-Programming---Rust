pub fn find_recur<T: AsRef<[K]>, K: Ord>(array: T, key: K) -> Option<usize> {
    let array = array.as_ref();
    let mid: usize = array.len() / 2;

    /* termination condition */
    if  array.is_empty() || (mid == 0 && array[mid] != key) {
        return None;
    }
    if key == array[mid] {
        return Some(mid)
    }

    /* recursion */
    let (left, right) = array.split_at(mid);
    if key < array[mid] {
        /* [0..mid) */
        find_recur(left, key)
    } else {
        /* [mid..array.len()) */
        match find_recur(right, key) {
            Some(i) => Some(i + mid),
            None => None,
        }
        //find(right, key).map(|i| { i + mid });  // more elegant
    }
}

pub fn find<T: AsRef<[K]>, K: Ord>(array: T, key: K) -> Option<usize> {
    let array = array.as_ref();
    let mut a_start = 0;
    let mut a_end = array.len();

    while a_start < a_end {
        let mid = (a_start + a_end) / 2;

        if key < array[mid] {
            a_end = mid;
        } else if key > array[mid] {
            a_start = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}