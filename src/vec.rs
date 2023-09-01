pub macro p_vec {
    ($ee:expr; for $ele:ident in $pp:expr) => {
        {
            let mut arr = vec![];

            for $ele in $pp {
                arr.push($ee);
            }

            arr
        }
    },
    () => {}
}

#[cfg(test)]
mod test_p_vec {
    use super::*;

    #[test]
    fn test_p_vec_with_for_loop() {
        assert_eq!(p_vec![i; for i in 0..5], [0, 1, 2, 3, 4]);
        assert_eq!(p_vec![i * 2; for i in 0..5], [0, 2, 4, 6, 8]);
        assert_eq!(p_vec![i * i; for i in 0..5], [0, 1, 4, 9, 16]);
    }
}
