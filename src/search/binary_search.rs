/// Binary search for element in slice.
/// If not found returns Err(insertion_index), where 'insertion_index' is the index where element can be inserted so that collection is still sorted.
pub fn binary_search<T: Ord>(sorted: &[T], el: &T) -> Result<usize, usize> {
    // left, mid, right
    let mut left = 0;
    let mut size = sorted.len();
    let mut right = size;

    while left < right {
        let mid = left + size / 2; // it will not overflow usize as well as it is in bound, cause size/2 < size and left + size <= len(), so left + size/2 < len()
                                   // note: size/2 < size, because left < right
        let m = &sorted[mid];
        // el is on left of m
        if el < m {
            right = mid
        }
        // el is on right of m
        if m < el {
            left = mid + 1
        }
        if m == el {
            return Ok(mid);
        }
        size = right - left
    }
    Err(left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_works() {
        let a = [1, 2, 40, 61, 121, 333, 335, 380];
        assert_eq!(Err(2), binary_search(&a, &4));
        assert_eq!(Ok(1), binary_search(&a, &2));
        assert_eq!(Ok(4), binary_search(&a, &121));
        assert_eq!(Ok(0), binary_search(&a, &1));
        assert_eq!(Ok(7), binary_search(&a, &380));
        assert_eq!(Ok(6), binary_search(&a, &335));
        assert_eq!(Err(8), binary_search(&a, &400));
    }
}
