static OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    let rows = minefield.len() as i32;
    let columns = minefield[0].chars().count() as i32;

    /* array creation: for each row */
    (0..rows).map(|x|
        {
            let row = minefield[x as usize].as_bytes();
            /* array creation: for each column */
            (0..columns).map(|y|
                {
                    /* for each entry having ' ' we count how many '*' are next to it */
                    match row[y as usize]
                    {
                        b' ' =>
                            {
                                match OFFSETS
                                    .iter()
                                    .map(|&(dx, dy)| (x + dx, y + dy))
                                    .filter(|&(x, y)|
                                        {
                                            (x >= 0 && x < rows) && (y >= 0 && y < columns)
                                        })
                                    .filter(|&(x, y)|
                                        {
                                            minefield[x as usize].as_bytes()[y as usize] == b'*'
                                        })
                                    .count()
                                {
                                    0 => ' ',
                                    n => (n as u8 + '0' as u8) as char,
                                }
                            },
                        b'*' => '*',
                        _ => ' ',  // make it work even though a string contains '.' instead of ' '
                    }
                }).collect()
        }).collect()
}
