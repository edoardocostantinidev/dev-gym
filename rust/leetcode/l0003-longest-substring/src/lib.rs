use std::collections::HashSet;

pub fn naive(s: String) -> i32 {
    let mut max = 0;
    let mut already_present: HashSet<char> = HashSet::new();
    for (i, c1) in s.chars().enumerate() {
        already_present.clear();
        already_present.insert(c1);
        for (_, c2) in s[i + 1..].chars().enumerate() {
            if already_present.contains(&c2) {
                break;
            } else {
                already_present.insert(c2);
            }
        }
        let len: i32 = already_present.len() as i32;
        if max < len {
            max = len;
        }
    }

    max
}

pub fn faster(s: String) -> i32 {
    let mut max_len: usize = 0;

    // [1] longest substring is the one with the largest
    //    difference between positions of repeated characters;
    //    thus, we should create a storage for such positions
    let mut pos: [usize; 128] = [0; 128];

    // [2] while iterating through the string (i.e., moving
    //    the end of the sliding window), we should also
    //    update the start of the window
    let mut start: usize = 0;

    for (end, ch) in s.chars().enumerate() {
        dbg!((max_len, pos, start, end, ch));
        // [3] get the position for the start of sliding window
        //    with no other occurences of 'ch' in it
        start = start.max(pos[ch as usize]);

        // [4] update maximum length
        max_len = max_len.max(end - start + 1);

        // [5] set the position to be used in [3] on next iterations
        pos[ch as usize] = end + 1;
    }

    return max_len as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(faster(String::from("abcabcbb")), 3);
        assert_eq!(naive(String::from("pwwkew")), 3);
        assert_eq!(naive(String::from(" ")), 1);
        assert_eq!(naive(String::from("dvdf")), 3);
    }
}
