use std::io::{stdin, stdout, Write};

/**
 * Instructions:
 * The ISBN-10 verification process is used to validate book identification numbers. These normally
 * contain dashes and look like: 3-598-21508-8
 *
 * ISBN:
 * The ISBN-10 format is 9 digits (0 to 9) plus one check character (either a digit or an X only).
 * In the case the check character is an X, this represents the value '10'. These may be
 * communicated with or without hyphens, and can be checked for their validity by the following
 * formula:
 * (x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1)
 * mod 11 == 0
 * If the result is 0, then it is a valid ISBN-10, otherwise it is invalid.
 *
 * Task:
 * Given a string the program should check if the provided string is a valid ISBN-10. Putting this
 * into place requires some thinking about preprocessing/parsing of the string prior to calculating
 * the check digit for the ISBN.
 * The program should be able to verify ISBN-10 both with and without separating dashes.
 *
 * Caveats:
 * Converting from strings to numbers can be tricky in certain languages. Now, it's even trickier
 * since the check digit of an ISBN-10 may be 'X' (representing '10'). For instance 3-598-21507-X is
 * a valid ISBN-10.
 *
 * Bonus tasks:
 * Generate a valid ISBN-13 from the input ISBN-10 (and maybe verify it again with a derived
 * verifier).
 * Generate valid ISBN, maybe even from a given starting ISBN.
 */

fn verify_isbn10(isbn10: String) -> bool {
    isbn10
        .chars()
        /* fold() takes two arguments: an initial value, and a closure with two arguments:
         *      - an ‘accumulator’ -> in this case the touple (count,sum,valid)
         *      - an element -> in this case a char c
         * The closure returns the value that the accumulator should have for the next iteration.
         * The initial value is the value the accumulator will have on the first call.
         * After applying this closure to every element of the iterator, fold() returns the
         * accumulator.
         */
        .fold((0, 0, true), |(count, sum, valid), c| match c {
            '0'..='9' => (
                count + 1,
                (sum + (10 - count) * c.to_digit(10).unwrap()) % 11,
                valid && (count < 10),
            ),
            'X' => (count + 1, (sum + 10) % 11, valid && (count == 9)),
            '-' => (count, sum, valid),
            _ => (0, 0, false),
        }) == (10, 0, true)
}

fn main() {
    /* prendendo l'ISBN da stdin non vaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa!
     * suppongo prenda anche il '\n' quando premo invio */
    // let mut s = String::new();
    //
    // print!("Type an ISBN10: ");
    // stdout().flush().unwrap();
    //
    // stdin().read_line(&mut s).unwrap();

    let s = "3-598-21508-8".to_string();

    if verify_isbn10(s) {
        println!("The ISBN10 inserted is a valid one");
    } else {
        println!("The ISBN10 inserted is NOT a valid one");
    }
}
