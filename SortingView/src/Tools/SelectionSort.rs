use crate::Tools::Op::interface::{SortType, print_Matriz, Sorting};
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
            if j % this_delay == 0 {
                print_Matriz(sorting, SortType::RangeUnique(j));
            }
        }
        sorting.array.swap(i, min);
        sorting.operations.movs += 3;
        sorting.operations.time = start.elapsed().as_millis();
    }
    
    // Finalizar com o array ordenado
    for i in 0..sorting.array.len() {
        print_Matriz(sorting, SortType::RangeUnique(i + 1));
    }
}
