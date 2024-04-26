#[cfg(test)]
mod test {
    use the_primeagen_algorithms::{binary_search, bubble_sort, two_crystal_ball};

    #[test]
    fn crystal_ball_test() {
        let good = vec![false; 637];
        let bad = vec![true; 300];
        let breaks = vec![good, bad].concat();
        let broke_location = two_crystal_ball(breaks);
        assert_eq!(broke_location, 637);
    }

    #[test]
    fn bs_doesnt_exist() {
        let mut haystack = vec![3, 5, 3, 0];
        haystack.sort();
        let needle = binary_search(haystack, 4);
        assert_eq!(needle, -1);
    }

    #[test]
    fn bs_test_start() {
        let mut haystack = vec![3, 5, 3, 0, 1, 2, 20, 5, 6, 7, 8, 9];
        haystack.sort();
        let needle = binary_search(haystack, 0);
        assert_eq!(needle, 0);
    }

    #[test]
    fn bs_test_end() {
        let mut haystack = vec![0, 1, 6, 7, 30, 9, 2, 1, 4, 4];
        haystack.sort();
        let needle = binary_search(haystack, 30);
        assert_eq!(needle, 30);
    }

    #[test]
    fn bs_test_exact_mid() {
        let haystack = vec![0, 1, 2, 3, 4, 5, 6];
        let needle = binary_search(haystack, 3);
        assert_eq!(needle, 3);
    }

    #[test]
    fn bs_test_mid() {
        let haystack = vec![0, 1, 3, 4, 5, 6];
        let needle = binary_search(haystack, 3);
        assert_eq!(needle, 3);
    }

    #[test]
    fn bubble_test() {
        let haystack = vec![3, 2, 5, 1, -1, 2, 6, 2, 9, 10, 5, 3, 12, 4];
        let sorted = bubble_sort(haystack);
        assert_eq!(sorted, vec![-1, 1, 2, 2, 2, 3, 3, 4, 5, 5, 6, 9, 10, 12])
    }
}
