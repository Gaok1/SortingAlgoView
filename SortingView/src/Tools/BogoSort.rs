use crate::Tools::Op::interface::{SortType, renderInterface, shuffle, Sorting};
use crate::Tools::Op::Constantes::*;
use std::time::Instant;

pub fn sort(sorting: &mut Sorting) {
    let start = Instant::now();
    
    while !is_sorted(&sorting.array) {
        shuffle(sorting);
        sorting.operations.time = start.elapsed().as_millis();
        sorting.operations.movs += 1;
        sorting.operations.comp += 1;
        renderInterface(sorting, SortType::Void);
    }

    // Finalizar com o array ordenado
    for i in 0..sorting.array.len() {
        renderInterface(sorting, SortType::RangeUnique(i + 1));
    }
}

fn is_sorted(array: &[usize]) -> bool {
    for i in 1..array.len() {
        if array[i] < array[i - 1] {
            return false;
        }
    }
    true
}
