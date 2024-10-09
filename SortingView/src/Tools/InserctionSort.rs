use crate::Tools::Op::interface::{renderInterface, SortType, Sorting};
use crate::Tools::Op::Constantes::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn sort(sorting: &mut Sorting) {
    let start = Instant::now();

    for i in 1..sorting.array.len() {
        let mut j = i;
        while j > 0 && sorting.array[j] < sorting.array[j - 1] {
            sorting.array.swap(j, j - 1);
            j -= 1;
            sorting.operations.movs += 3;
            sorting.operations.comp += 2;
            sorting.operations.time = start.elapsed().as_millis();

            renderInterface(sorting, SortType::RangeUnique(j));
            sleep(Duration::from_millis(sorting.get_delay()));
        }
    }
    for i in 0..sorting.array.len() {
        renderInterface(sorting, SortType::RangeUnique(i + 1));
    }
}
