use crate::sort::Sorter;
use std::fmt::Debug;

struct BubbleSort;
impl Sorter for BubbleSort {
    // [unsorted | sorted]
    // sorted: sink+1..slice.len()
    // check pairs and if element is bigger then next ... then swap them
    fn sort<T: Ord + Debug>(slice: &mut [T]) {
        // let mut things = [4, 2, 5, 3, 1];
        if let 0 | 1 = slice.len() {
            return;
        }
        let mut swapped = true;
        let mut sink = slice.len() - 1;
        while sink > 0 && swapped {
            swapped = false;
            for i in 0..sink {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
            sink -= 1;
        }
    }
}

struct BubbleSortWithSinkAtTheBeginning;

impl Sorter for BubbleSortWithSinkAtTheBeginning {
    // [sorted | unsorted]
    // sorted: 0..sink
    fn sort<T: Ord + Debug>(slice: &mut [T]) {
        if let 0 | 1 = slice.len() {
            return;
        }
        let mut sink = 0;
        let mut swapped = true;
        while sink < slice.len() && swapped {
            swapped = false;
            for i in (sink..slice.len() - 1).rev() {
                // last ... 0
                if slice[i + 1] < slice[i] {
                    slice.swap(i + 1, i);
                    swapped = true;
                }
            }
            sink += 1;
        }
    }
}

struct BubbleSortDescending;
impl Sorter for BubbleSortDescending {
    // [unsorted | sorted ]
    // sorted: sink+1..len()-1
    fn sort<T: Ord + Debug>(slice: &mut [T]) {
        if let 0 | 1 = slice.len() {
            return;
        }
        let mut sink = slice.len() - 1;
        let mut swapped = true;
        while sink > 0 && swapped {
            swapped = false;
            for i in 0..sink {
                if slice[i] < slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
            sink -= 1;
        }

        slice.reverse();
    }
}

#[test]
fn test_bubble_sort() {
    use crate::sort::tests::test_sorting;
    test_sorting::<BubbleSort>();
}

#[test]
fn test_bubble_sort_with_sink_at_the_beginning() {
    use crate::sort::tests::test_sorting;
    test_sorting::<BubbleSortWithSinkAtTheBeginning>();
}

#[test]
fn test_bubble_sort_descending() {
    use crate::sort::tests::test_sorting;
    test_sorting::<BubbleSortDescending>();
}
