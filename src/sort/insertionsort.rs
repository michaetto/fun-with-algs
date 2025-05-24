use crate::sort::Sorter;

pub struct InsertionSortWithIndividualSwaps;
impl Sorter for InsertionSortWithIndividualSwaps {
    fn sort<T: Ord>(slice: &mut [T]) {
        for unsorted in 1..slice.len() {
            // [sorted | unsorted]
            // take first element from unsorted and insert it in right place in sorted
            // find right place by linear compare and bubble swap
            let mut i = unsorted;
            while i > 0 && slice[i] < slice[i - 1] {
                slice.swap(i, i - 1);
                i -= 1;
            }
        }
    }
}

pub struct InsertionSortWithStdPartitionPointAndRotation;
impl Sorter for InsertionSortWithStdPartitionPointAndRotation {
    fn sort<T: Ord + Clone>(slice: &mut [T]) {
        // [sorted | unsorted]
        // take first element from unsorted and insert it in right place in sorted
        for unsorted in 1..slice.len() {
            let unsorted_element = &slice[unsorted];
            // find right place by std implementation of binary search
            let insertion_index = slice[..unsorted].partition_point(|e| e <= unsorted_element);
            // swap elements by std rotating
            slice[insertion_index..=unsorted].rotate_right(1);
        }
    }
}

use crate::search::binary_search::binary_search;
pub struct InsertionSortWithCustomBinarySearch;
impl Sorter for InsertionSortWithCustomBinarySearch {
    fn sort<T: Ord + Clone>(slice: &mut [T]) {
        // [sorted | unsorted]
        // take first element from unsorted and insert it in right place in sorted
        for unsorted in 1..slice.len() {
            let unsorted_element = &slice[unsorted];
            // find right place by custom implementation of binary search
            let insertion_index = match binary_search(&slice[..unsorted], unsorted_element) {
                Ok(index) => index,
                Err(index) => index,
            };
            // swap elements by std rotating
            slice[insertion_index..=unsorted].rotate_right(1);
        }
    }
}

#[test]
fn insertion_with_individual_swaps_works() {
    use crate::sort::tests::test_sorting;
    test_sorting::<InsertionSortWithIndividualSwaps>();
}

#[test]
fn insertion_with_std_partition_point_and_rotation_works() {
    use crate::sort::tests::test_sorting;
    test_sorting::<InsertionSortWithStdPartitionPointAndRotation>();
}

#[test]
fn insertion_with_custom_binary_search_works() {
    use crate::sort::tests::test_sorting;
    test_sorting::<InsertionSortWithCustomBinarySearch>();
}
