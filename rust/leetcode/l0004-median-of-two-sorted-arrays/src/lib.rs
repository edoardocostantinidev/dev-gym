use std::{iter::Peekable, ops::Div, slice::Iter};

//run 55% mem 43%
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

//run 100% mem 99.88%
pub fn fast(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let combined_length = nums1.len() + nums2.len();
    let mut count = 0;
    let middle = if combined_length % 2 == 0 {
        (Some((combined_length / 2, combined_length / 2 - 1)), None)
    } else if combined_length > 2 {
        (None, Some(combined_length / 2))
    } else {
        (None, Some(1))
    };
    let mut p1 = nums1.iter().peekable();
    let mut p2 = nums2.iter().peekable();
    loop {
        let x = get_next(&mut p1, &mut p2);
        if let Some(num) = x {
            match middle {
                (None, Some(mid)) if count == mid - 1 => {
                    return get_next(&mut p1, &mut p2).unwrap_or(num) as f64
                }
                (Some((mid, _)), None) if count == mid - 1 => {
                    return (num + get_next(&mut p1, &mut p2).unwrap()) as f64 / 2.0
                }
                _ => {
                    count += 1;
                }
            }
        } else {
            return 0.0;
        }
    }
}

fn get_next(p1: &mut Peekable<Iter<'_, i32>>, p2: &mut Peekable<Iter<'_, i32>>) -> Option<i32> {
    let n1: Option<i32> = p1.peek().map(|x| **x);
    let n2: Option<i32> = p2.peek().map(|x| **x);

    match (n1, n2) {
        (Some(n1), Some(n2)) => {
            let result = if n1 < n2 {
                p1.next();
                n1
            } else {
                p2.next();
                n2
            };
            Some(result)
        }
        (Some(n), _) => {
            p1.next();
            Some(n)
        }
        (_, Some(n)) => {
            p2.next();
            Some(n)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(naive(vec![1, 2], vec![3, 4]), 2.5);
        // assert_eq!(naive(vec![1, 2], vec![3]), 2.0);

        assert_eq!(fast(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(fast(vec![1, 2], vec![3]), 2.0);
        assert_eq!(fast(vec![], vec![1]), 1.0);
    }
}
