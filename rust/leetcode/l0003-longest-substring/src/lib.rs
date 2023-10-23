use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut map: HashSet<&u8> = Default::default();
    let mut max = 0;
    let mut current = 0;
    s.as_bytes().iter().for_each(|b| {
        println!("Before {current}");
        let was_not_already_present = map.insert(b);
        if !was_not_already_present {
            if max < current {
                max = current;
            }
            current = 1;
        } else {
            current += 1;
        }
        println!("After {current}");
    });
    if max < current {
        current
    } else {
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
        //assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
        //assert_eq!(length_of_longest_substring(String::from(" ")), 1);
        assert_eq!(length_of_longest_substring(String::from("dvdf")), 3);
    }
}
