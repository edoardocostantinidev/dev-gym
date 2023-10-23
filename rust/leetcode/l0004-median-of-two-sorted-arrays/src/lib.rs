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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(naive(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(naive(vec![1, 2], vec![3]), 2.0);
    }
}
