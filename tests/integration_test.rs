mod tests {
    #[test]
    fn test_sort() {
        assert_eq!(handler::merge_sort(&[3, 2, 1].to_vec()), [1, 2, 3].to_vec());
        assert_eq!(handler::merge_sort(&[-11, 233, 44, 581].to_vec()), [-11, 44, 233, 581].to_vec());
        assert_eq!(handler::merge_sort(&[-12653, -111, -323, -3223].to_vec()), [-12653, -3223, -323, -111].to_vec());
    }

    #[test]
    fn test_replace() {
        assert_eq!(handler::replace(&123), 321);
        assert_eq!(handler::replace(&1123), 1321);
        assert_eq!(handler::replace(&22167512332), 22167532132);
    }
}