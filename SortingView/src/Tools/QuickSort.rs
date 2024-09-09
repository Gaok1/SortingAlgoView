use crate::Tools::Op::interface::{SortType, print_Matriz, Sorting};
use crate::Tools::Op::Constantes::*;
use std::time::Instant;

pub fn sort(sorting: &mut Sorting) {
    let start = Instant::now();
    quick_sort_helper(sorting, 0, sorting.array.len() - 1, start);

    for i in 0..sorting.array.len() {
        print_Matriz(sorting, SortType::RangeUnique(i + 1));
    }
}

fn quick_sort_helper(sorting: &mut Sorting, low: usize, high: usize, start: Instant) {
    if low < high {
        let pivot_index = partition(sorting, low, high, start);
        
        // Recursivamente chamar o quick_sort_helper para a esquerda e direita do pivô
        quick_sort_helper(sorting, low, pivot_index.saturating_sub(1), start);
        quick_sort_helper(sorting, pivot_index + 1, high, start);
    }
}

fn partition(sorting: &mut Sorting, low: usize, high: usize, start: Instant) -> usize {
    let pivot = sorting.array[high];
    let mut i = low;

    for j in low..high {
        if sorting.array[j] <= pivot {
            sorting.array.swap(i, j);
            i += 1;
            sorting.operations.movs += 3;
        }
        sorting.operations.comp += 1;
        sorting.operations.time = start.elapsed().as_millis();
        print_Matriz(sorting, SortType::TwoRange(j, pivot));
    }
    
    sorting.array.swap(i, high);
    sorting.operations.movs += 3;
    sorting.operations.time = start.elapsed().as_millis();
    print_Matriz(sorting, SortType::TwoRange(i, high));

    i
}
