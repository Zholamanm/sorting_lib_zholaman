pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod sorts {
    pub mod quick_sort;
    pub mod selection_sort;
    pub mod insertion_sort;
    pub mod merge_sort;
}

#[cfg(test)]
mod tests {
    use super::sorts::quick_sort::quick_sort;
    use super::sorts::selection_sort::selection_sort;
    use super::sorts::insertion_sort::insertion_sort;
    use super::sorts::merge_sort::merge_sort;

    #[test]
    fn test_quick_sort() {
        let mut vec = vec![4, 3, 2, 1];
        quick_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4]);
    }

    #[test]
    fn test_selection_sort() {
        let mut vec = vec![4, 3, 2, 1];
        selection_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut vec = vec![4, 3, 2, 1];
        insertion_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4]);
    }

    #[test]
    fn test_merge_sort() {
        let mut vec = vec![4, 3, 2, 1];
        merge_sort(&mut vec);
        assert_eq!(vec, [1, 2, 3, 4]);
    }
}

