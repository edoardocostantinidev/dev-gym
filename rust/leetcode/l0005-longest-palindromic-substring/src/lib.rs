pub fn naive(s: String) -> String {
    let mut longest = String::new();
    if s.len() == 1 {
        return s;
    } else if s.len() == 2 {
        if s.chars().nth(0).unwrap() == s.chars().nth(1).unwrap() {
            return s;
        }
    }
    for (i, _) in s.chars().enumerate() {
        for (j, _) in s.chars().enumerate() {
            if i >= j {
                continue;
            }
            let r = &s[i..j];
            if is_palindrome(r) && r.len() > longest.len() {
                longest = r.to_owned();
            }
        }
    }
    longest
}

pub fn is_palindrome(s: &str) -> bool {
    let len = s.len();
    match len {
        0 | 1 => true,
        2 => s.chars().nth(0).unwrap() == s.chars().nth(len - 1).unwrap(),
        _ => {
            s.chars().nth(0).unwrap() == s.chars().nth(len - 1).unwrap()
                && is_palindrome(&s[1..len - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(naive(String::from("babad")), String::from("bab"));
        assert_eq!(naive(String::from("cbbd")), String::from("bb"));
    }
}
