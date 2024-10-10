pub mod bubblesort;
pub mod heapsort;
pub mod insertionsort;
pub mod mergesort;
pub mod quicksort;
pub mod selectionsort;

pub trait Sorter {
    fn sort<T: Ord + Clone>(slice: &mut [T]);
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn works_for_sorter<S>()
    where
        S: Sorter,
    {
        let mut things = [4, 2, 5, 3, 1];
        S::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 4, 5]);

        let mut things = [4, 2, 3, 5, 3, 1];
        S::sort(&mut things);
        assert_eq!(things, [1, 2, 3, 3, 4, 5]);

        let mut things = [1, 2, 3];
        S::sort(&mut things);
        assert_eq!(things, [1, 2, 3]);

        let mut things = [3, 2, 1];
        S::sort(&mut things);
        assert_eq!(things, [1, 2, 3]);

        let mut things = [2, 1];
        S::sort(&mut things);
        assert_eq!(things, [1, 2]);

        let mut things = [1, 2];
        S::sort(&mut things);
        assert_eq!(things, [1, 2]);

        let mut things = [1];
        S::sort(&mut things);
        assert_eq!(things, [1]);

        let mut things = Vec::<u32>::new();
        S::sort(&mut things);
        assert_eq!(things, []);
    }

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T: Ord>(slice: &mut [T]) {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        works_for_sorter::<StdSorter>();
    }
}
