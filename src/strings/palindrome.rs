pub fn is_palindrome(word: &str) -> bool {
    let chars: Vec<_> = word.chars().collect();
    do_check(&chars)
}

fn do_check(items: &[char]) -> bool {
    match items {
        [first, middle @ .., last] => first == last && do_check(middle),
        [] | [_] => true,
    }
}
