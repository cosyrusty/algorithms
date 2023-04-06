pub fn bubble_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 1..arr.len() {
            if arr[i] < arr[i - 1] {
                arr.swap(i, i - 1);
                sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn basic() {
        let mut nums = [1, 3, 2, 8, 4];
        bubble_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 8]);
    }

    #[test]
    fn basic_char() {
        let mut nums = ['b', 'd', 'c', 'z', 'a', 'b'];
        bubble_sort(&mut nums);
        assert_eq!(nums, ['a', 'b', 'b', 'c', 'd', 'z']);
    }

    #[test]
    fn reverse() {
        let mut nums = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        bubble_sort(&mut nums);
        assert_eq!(nums, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
