use crate::sort::Sorter;

pub struct BubbleSort;
impl Sorter for BubbleSort {
    fn sort<T: Ord + Clone>(slice: &mut [T]) {
        if let 0 | 1 = slice.len() {
            return;
        }
        let mut swapped = true;
        let mut sink = slice.len() - 1;
        while swapped && sink > 0 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::tests::works_for_sorter;

    #[test]
    fn bubblesort_works() {
        works_for_sorter::<BubbleSort>();
    }
}
