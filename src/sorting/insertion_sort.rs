pub fn insertion_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    // 3 2 1
    for i in 0..arr.len() {
        for j in (0..i).rev() {
            if arr[j + 1] < arr[j] {
                arr.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::insertion_sort;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
