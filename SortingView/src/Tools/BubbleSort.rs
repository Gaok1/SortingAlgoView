use crate::Tools::Op::interface::{SortType, print_Matriz, Sorting};
use crate::Tools::Op::Constantes::*;
use std::time::Instant;

pub fn sort(sorting: &mut Sorting) {
    let start = Instant::now();
    
    for i in 0..sorting.array.len() {
        for j in 0..sorting.array.len() - 1 {
            if sorting.array[j] > sorting.array[j + 1] {
                sorting.array.swap(j, j + 1);
                sorting.operations.movs += 3;
                sorting.operations.comp += 1;
                sorting.operations.time = start.elapsed().as_millis();
                
                if j % sorting.get_delay() == 0 {
                    print_Matriz(sorting, SortType::TwoRange(j, j + 1));
                }
            }
        }
    }
    
    // Finalizar com o array ordenado
    for i in 0..sorting.array.len() {
        print_Matriz(sorting, SortType::RangeUnique(i + 1));
    }
}
