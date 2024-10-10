use crate::sort::Sorter;

pub struct SelectionSort;
impl Sorter for SelectionSort {
    fn sort<T: Ord + Clone>(slice: &mut [T]) {
        // [sorted | unsorted]
        // find smallest in unsorted and move it to the end of sorted
        for unsorted in 0..slice.len() {
            let mut smallest_in_rest = unsorted;
            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallest_in_rest] {
                    smallest_in_rest = i;
                }
            }
            slice.swap(unsorted, smallest_in_rest);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::tests::works_for_sorter;

    #[test]
    fn selectionsort_works() {
        works_for_sorter::<SelectionSort>();
    }
}
