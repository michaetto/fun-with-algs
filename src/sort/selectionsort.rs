use crate::sort::Sorter;
use std::fmt::Debug;

struct SelectionSort;

impl Sorter for SelectionSort {
    // [sorted | unstorted]
    // pick min from unsorted and put it at the end of sorted
    fn sort<T: Ord + Debug>(slice: &mut [T]) {
        if let 0 | 1 = slice.len() {
            return;
        }
        for unsorted in 0..slice.len() {
            let mut min = unsorted;
            for i in (unsorted + 1)..slice.len() {
                if slice[min] > slice[i] {
                    min = i
                }
            }
            slice.swap(unsorted, min)
        }
    }
}

#[test]
fn test_selection_sort() {
    use crate::sort::tests::test_sorting;
    test_sorting::<SelectionSort>();
}
