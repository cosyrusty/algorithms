// [3, 2, 4, 1]
pub fn selection_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    if arr.is_empty() || arr.len() == 1 {
        return;
    }

    // find min element index
    let mn = arr
        .iter()
        .enumerate()
        .min_by_key(|&(_index, value)| value)
        .unwrap()
        .0;

    // [3, 2, 4, 1] = [1, 2, 4, 3]
    arr.swap(0, mn);

    // call again for [2, 4, 3]
    selection_sort(&mut arr[1..])
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::selection_sort;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        selection_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        selection_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        selection_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        selection_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        selection_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        selection_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
