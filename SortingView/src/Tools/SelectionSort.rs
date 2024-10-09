use std::thread::sleep;
use std::time::Duration;

use crate::Tools::Op::interface::{renderInterface, SortType, Sorting};
use crate::Tools::Op::Constantes::*;

pub fn sort(sorting: &mut Sorting) {
    let this_delay = sorting.get_delay() * 2;
    let start = std::time::Instant::now();
    for i in 0..sorting.array.len() {
        let mut min = i;
        for j in i + 1..sorting.array.len() {
            if sorting.array[j] < sorting.array[min] {
                min = j;
            }
            sorting.operations.comp += 1;

            renderInterface(sorting, SortType::RangeUnique(j));
            sleep(Duration::from_millis(sorting.get_delay()));
        }
        sorting.array.swap(i, min);
        sorting.operations.movs += 3;
        sorting.operations.time = start.elapsed().as_millis();
    }

    // Finalizar com o array ordenado
    for i in 0..sorting.array.len() {
        renderInterface(sorting, SortType::RangeUnique(i + 1));
    }
}
