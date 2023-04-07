pub fn merge_sort<T>(arr: &mut [T])
where
    T: Ord + Copy,
{
    if arr.is_empty() || arr.len() == 1 {
        return;
    }

    // 2 4   1 3
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let (start, mid, end) = (0, arr.len() / 2, arr.len() - 1);
    merge(arr, start, mid, end);
}

pub fn merge<T>(arr: &mut [T], start: usize, mid: usize, end: usize)
where
    T: Ord + Copy,
{
    let mut merged: Vec<T> = vec![];

    let mut i = start;
    let mut j = mid;

    // 1 4   2 3
    while i < mid && j <= end {
        if arr[i] < arr[j] {
            merged.push(arr[i]);
            i += 1;
        } else {
            merged.push(arr[j]);
            j += 1;
        }
    }

    while i < mid {
        merged.push(arr[i]);
        i += 1;
    }

    while j <= end {
        merged.push(arr[j]);
        j += 1;
    }

    for i in 0..merged.len() {
        arr[start + i] = merged[i];
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::merge;

    #[test]
    fn is_merged() {
        let mut nums = [2, 4, 1, 3];
        let (start, mid, end) = (0, nums.len() / 2, nums.len() - 1);
        merge(&mut nums, start, mid, end);
        assert!(is_sorted(&nums));
    }

    #[test]
    fn is_merged_2() {
        let mut nums = [2, 4, 1, 3, 5];
        let (start, mid, end) = (0, nums.len() / 2, nums.len() - 1);
        merge(&mut nums, start, mid, end);
        assert!(is_sorted(&nums));
    }

    #[test]
    fn is_merged_3() {
        let mut nums = [2, 4];
        let (start, mid, end) = (0, nums.len() / 2, nums.len() - 1);
        merge(&mut nums, start, mid, end);
        assert!(is_sorted(&nums));
    }

    // merge_sort
    use super::merge_sort;

    #[test]
    fn empty() {
        let mut arr: [u8; 0] = [];
        merge_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr: [char; 1] = ['a'];
        merge_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn already_sorted() {
        let mut arr: [&str; 3] = ["a", "b", "c"];
        merge_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn basic() {
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        merge_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        merge_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn repeated_elements() {
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        merge_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
