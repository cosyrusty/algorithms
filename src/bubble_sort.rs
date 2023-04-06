pub fn bubble_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut n = arr.len();

    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }

        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::bubble_sort;

    #[test]
    fn basic() {
        let mut nums = [1, 3, 2, 8, 4];
        bubble_sort(&mut nums);
        assert!(is_sorted(&nums));
    }

    #[test]
    fn empty() {
        let mut nums: [usize; 0] = [];
        bubble_sort(&mut nums);
        assert!(is_sorted(&nums));
    }

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut ve1);
        assert!(is_sorted(&ve1));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut ve2);
        assert!(is_sorted(&ve2));
    }

    #[test]
    fn basic_char() {
        let mut nums = ['b', 'd', 'c', 'z', 'a', 'b'];
        bubble_sort(&mut nums);
        assert!(is_sorted(&nums));
    }

    #[test]
    fn reverse() {
        let mut nums = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        bubble_sort(&mut nums);
        assert!(is_sorted(&nums));
    }
}
