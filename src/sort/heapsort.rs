use crate::sort::Sorter;

pub struct HeapSort;
impl Sorter for HeapSort {
    fn sort<T: Ord + Clone>(slice: &mut [T]) {
        heap_sort(slice);
    }
}

fn bubble_down<T: Ord>(slice: &mut [T], p: usize) {
    // take bigger of the child and if it is greater then parent: swap them
    let ch1 = 2 * p + 1;
    if ch1 >= slice.len() {
        return; // no children
    }
    let ch2 = ch1 + 1;
    let bigger = if ch2 >= slice.len() || slice[ch1] > slice[ch2] {
        ch1
    } else {
        ch2
    };
    if slice[bigger] > slice[p] {
        slice.swap(p, bigger);
        bubble_down(slice, bigger);
    }
}

fn build<T: Ord>(slice: &mut [T]) {
    let mid = slice.len() / 2;
    for i in (0..=mid).rev() {
        bubble_down(slice, i);
    }
}

fn process_sort<T: Ord>(slice: &mut [T]) {
    if slice.len() < 2 {
        return;
    }
    // swap last and first, in other words put max at the end
    slice.swap(0, slice.len() - 1);
    // extract all but last element, i.e. last is already on right place
    let [heap @ .., _] = slice else {
        panic!("length should be at least: 2")
    };
    // repair heap, after swap
    bubble_down(heap, 0);
    process_sort(heap);
}

fn heap_sort<T: Ord>(slice: &mut [T]) {
    // make heap
    build(slice);
    // sort recursively
    process_sort(slice);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::tests::works_for_sorter;

    #[test]
    fn heapsort_works() {
        works_for_sorter::<HeapSort>();
    }

    #[test]
    fn build_works() {
        let mut a = [1, 2, 4, 5, 8, 3, 9, 8, 10];
        build(&mut a);
        assert_eq!(a, [10, 8, 9, 8, 1, 3, 4, 2, 5]);
    }
}
