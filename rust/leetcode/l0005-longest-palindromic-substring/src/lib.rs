// not mine, didn't know about windows() function
pub fn longest_palindrome(s: String) -> String {
    fn is_palidrome(s: &[u8]) -> bool {
        s.iter().zip(s.iter().rev()).all(|(l, r)| l == r)
    }

    for size in dbg!(1..=s.len()).rev() {
        match s
            .as_bytes()
            .windows(size)
            .find(|substr| is_palidrome(substr))
        {
            Some(pal) => return String::from_utf8(pal.to_vec()).unwrap(),
            None => continue,
        }
    }
    String::from("")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            longest_palindrome(String::from("babad")),
            String::from("bab")
        );
        assert_eq!(longest_palindrome(String::from("cbbd")), String::from("bb"));
    }
}
