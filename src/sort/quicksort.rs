use crate::sort::Sorter;

pub struct QuickSort;
impl Sorter for QuickSort {
    fn sort<T: Ord + Clone>(slice: &mut [T]) {
        quicksort(slice)
    }
}

// Make the choice of pivot more balanced.
fn put_pivot_at_first<T: Ord + Clone>(slice: &mut [T]) {
    // take median of [first, mid, last]
    // and put it as first in order to make it a pivot
    let len = slice.len();
    let first = 0;
    let last = len - 1;
    let mid = len / 2;

    let index = if slice[first] <= slice[mid] {
        // first <= mid
        if slice[mid] <= slice[last] {
            // first <= mid <= last
            mid
        } else if slice[last] <= slice[first] {
            // last <= first <= mid
            first
        } else {
            last
        }
    } else {
        // mid < first
        if slice[first] <= slice[last] {
            // mid < first <= last
            first
        } else if slice[last] <= slice[mid] {
            // last <= mid < first
            mid
        } else {
            last
        }
    };

    slice.swap(0, index);
}

fn quicksort<T: Ord + Clone>(slice: &mut [T]) {
    // recursively order by: [unsorted | pivot | unsorted ]

    if slice.is_empty() {
        return;
    }

    put_pivot_at_first(slice);

    // everything on left side of slice[left] (exluding it) is less or equal to the pivot
    // everything on right side of slice[right] (exluding it) is greater then pivot

    let mut left = 1;
    let mut right = slice.len() - 1;

    // left: 1 ..= slice.len()
    // right: 0 ..= (slice.len()-1)
    while left <= right {
        if slice[left] <= slice[0] {
            // already on the correct side
            left += 1; // number of elements on left (correct) side increases
        } else if slice[0] < slice[right] {
            // already on correct side
            right -= 1; // number of elements on right (correct) side increases
                        // this subtraction will not overflow usize, because:
                        // - pivot is on 0 index
                        // - and pivot is NOT '<' then pivot itself
        } else {
            // here: slice[right] <= pivot < slice[left]

            // move elements to the correct side:
            slice.swap(left, right);
            // after swap: slice[left] <= pivot < slice[right]

            left += 1; // number of elements on left (correct) side increases
            right -= 1; // number of elements on right (correct) side increases
                        // this subtraction will not overflow usize, the same reason as above
        }
    }
    // after the loop indexes 1..left (i.e. 1..=(left-1)) has something <= pivot and pivot is slice[0]
    // put pivot at the end of left side:
    slice.swap(0, left - 1); // now pivot is already on right/sorted place as well

    let (left_side, right_side) = slice.split_at_mut(left - 1); // after this right_side has pivot as first element
    assert!(left_side.last() <= right_side.first());
    // recursive over left side
    quicksort(left_side);
    // .. and right side, but without pivot
    quicksort(&mut right_side[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::tests::works_for_sorter;

    #[test]
    fn quicksort_works() {
        works_for_sorter::<QuickSort>();
    }

    #[test]
    fn put_pivot_works() {
        let mut things = [1, 2, 3];
        put_pivot_at_first(&mut things);
        assert_eq!(things[0], 2);

        let mut things = [3, 1, 2];
        put_pivot_at_first(&mut things);
        assert_eq!(things[0], 2);

        let mut things = [2, 3, 1];
        put_pivot_at_first(&mut things);
        assert_eq!(things[0], 2);

        let mut things = [3, 2, 1];
        put_pivot_at_first(&mut things);
        assert_eq!(things[0], 2);

        let mut things = [1, 3, 2];
        put_pivot_at_first(&mut things);
        assert_eq!(things[0], 2);

        let mut things = [2, 1, 3];
        put_pivot_at_first(&mut things);
        assert_eq!(things[0], 2);
    }
}
