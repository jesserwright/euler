// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6 and 9. The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

// 1st attempt
fn compute(bound: u32) -> u32 {
    let mut acc = 0;
    for n in 1..bound {
        if n % 3 == 0 || n % 5 == 0 {
            acc += n;
        }
    }
    acc
}

// using iterators
fn compute_iter(bound: u32) -> u32 {
    (1..bound).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[test]
fn prob1() {
    assert_eq!(compute(10), 23);
    assert_eq!(compute(1000), 233_168);
    assert_eq!(compute_iter(10), 23);
    assert_eq!(compute_iter(1000), 233_168);
}
