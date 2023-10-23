use std::ops::Div;

pub fn naive(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut z = vec![];
    z.extend(nums1.into_iter());
    z.extend(nums2.into_iter());
    z.sort();
    let len = z.len();
    if len % 2 == 0 {
        let l1 = len.div(2);
        (z[l1] + z[l1 - 1]) as f64 / 2.0
    } else {
        z[len.div(2)] as f64
    }
}

pub fn fast(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let combined_length = nums1.len() + nums2.len();

    let (longer, shorter) = if nums1.len() >= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    let mut longer_iter = longer.into_iter();
    let mut shorter_iter = shorter.into_iter().peekable();
    let mut count = 0;
    let mut last_value = i32::MIN;
    let middle = if combined_length % 2 == 0 {
        (Some((combined_length / 2, combined_length / 2 - 1)), None)
    } else {
        (None, Some(combined_length / 2))
    };
    dbg!(middle);
    while let Some(n) = longer_iter.next() {
        match middle {
            (Some((mid, _)), _) if mid == count => return (n + last_value) as f64 / 2.0,
            (None, Some(mid)) if mid == count => return n as f64,
            _ => (),
        }
        dbg!((n, count, last_value), "before");

        while let Some(m) = shorter_iter.peek() {
            if *m < n {
                count += 1;
                match middle {
                    (Some((mid, _)), _) if mid == count => return (m + last_value) as f64 / 2.0,
                    (None, Some(mid)) if mid == count => return *m as f64,
                    _ => (),
                }
                last_value = *m;
                shorter_iter.next();
            } else {
                count += 1;
                last_value = n;
                break;
            }
        }

        dbg!((n, count, last_value), "after");
    }

    panic!("should not happen")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(naive(vec![1, 2], vec![3, 4]), 2.5);
        // assert_eq!(naive(vec![1, 2], vec![3]), 2.0);

        assert_eq!(fast(vec![1, 2], vec![3, 4]), 2.5);
        //assert_eq!(fast(vec![1, 2], vec![3]), 2.0);
    }
}
