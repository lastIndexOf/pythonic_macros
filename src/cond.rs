pub macro p_cond {
    () => {},
    ($true_value:tt if $cond:tt else $false_value:expr) => {
        {
            if $cond {
                $true_value
            } else {
                $false_value
            }
        }
    },
    ($true_value:tt if $cond_left:tt == $cond_right:tt else $false_value:expr) => {
        {
            if $cond_left == $cond_right {
                $true_value
            } else {
                $false_value
            }
        }
    },
    ($true_value:tt if $cond_left:tt > $cond_right:tt else $false_value:expr) => {
        {
            if $cond_left > $cond_right {
                $true_value
            } else {
                $false_value
            }
        }
    },
    ($true_value:tt if $cond_left:tt >= $cond_right:tt else $false_value:expr) => {
        {
            if $cond_left >= $cond_right {
                $true_value
            } else {
                $false_value
            }
        }
    },
    ($true_value:tt if $cond_left:tt < $cond_right:tt else $false_value:expr) => {
        {
            if $cond_left < $cond_right {
                $true_value
            } else {
                $false_value
            }
        }
    },
    ($true_value:tt if $cond_left:tt <= $cond_right:tt else $false_value:expr) => {
        {
            if $cond_left <= $cond_right {
                $true_value
            } else {
                $false_value
            }
        }
    },
}

#[cfg(test)]
mod test_p_cond {
    use super::*;

    #[test]
    fn test_p_cond() {
        assert_eq!(10, p_cond!(10 if 10 == 10 else 20));
        assert_eq!(20, p_cond!(10 if 10 == 20 else 20));
        assert_eq!(10, p_cond!(10 if 10 >= 10 else 20));
        assert_eq!(10, p_cond!(10 if 10 > 0 else 20));
        assert_eq!(10, p_cond!((2 * 5) if 10 > 0 else 20));
    }
}
