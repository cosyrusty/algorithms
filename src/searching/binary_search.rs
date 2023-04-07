// TODO: impl for descending order, is_ascending = arr.first() > arr.last()

pub fn binary_search<T>(value: &T, arr: &[T]) -> Option<usize>
where
    T: PartialEq + std::cmp::PartialOrd,
{
    if arr.is_empty() {
        return None;
    }

    recursive_search(value, arr, 0, arr.len() - 1)
}

fn recursive_search<T>(value: &T, arr: &[T], start: usize, end: usize) -> Option<usize>
where
    T: PartialEq + std::cmp::PartialOrd,
{
    if end == start {
        if arr[start] == *value {
            return Some(start);
        } else {
            return None;
        }
    }

    let mid = (end + start + 1) / 2;

    // 1 2 3 4
    if arr[mid] > *value {
        return recursive_search(value, arr, 0, mid - 1);
    } else {
        return recursive_search(value, arr, mid, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = binary_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings_asc() {
        let index = binary_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));

        let index = binary_search(&"google", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(4));
    }

    #[test]
    fn search_ints_asc() {
        let index = binary_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = binary_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = binary_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = binary_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);

        let index = binary_search(&5, &[4, 3, 2, 1]);
        assert_eq!(index, None);
    }
}
