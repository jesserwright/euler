// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn compute(mut n: i64) -> i64 {
    let mut largest = 1;
    while n > 1 {
        let mut curr = 2;
        while n % curr != 0 && curr < n {
            curr += 1;
        }
        if curr > largest {
            largest = curr;
        }
        n /= curr;
    }
    largest
}

#[test]
fn prob3() {
    assert_eq!(compute(600_851_475_143), 6857);
    assert_eq!(compute(13_195), 29);
}
