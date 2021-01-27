// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

fn prob4(max: i32) -> i32 {
    let mut curr_max = 0;
    for x in 1..=max {
        for y in 1..=x {
            let maybe_palindrome = x * y;
            if is_palindrome(maybe_palindrome) && maybe_palindrome > curr_max {
                curr_max = maybe_palindrome;
            }
        }
    }
    curr_max
}

#[test]
fn prob4_test() {
    let x = prob4(99);
    assert_eq!(x, 9009);
    let y = prob4(999);
    println!("answer: {}", y);
}

fn is_palindrome(num: i32) -> bool {
    let mut x: Vec<char> = num.to_string().chars().collect();
    loop {
        let len = x.len();
        if len > 1 {
            let a = x.remove(0);
            let b = x.remove(len - 2);
            if a != b {
                return false;
            }
        } else {
            return true;
        }
    }
}

#[test]
fn check_if_palindrome_test() {
    assert_eq!(is_palindrome(1001), true);
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(10000), false);
    assert_eq!(is_palindrome(1_234_321), true);
}
