use crate::Tools::Op::interface::{SortType, renderInterface, Sorting};
use crate::Tools::Op::Constantes::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn sort(sorting: &mut Sorting) {
    let start = Instant::now();
    
    for i in 0..sorting.array.len() {
        for j in 0..sorting.array.len() - 1 {
            if sorting.array[j] > sorting.array[j + 1] {
                sorting.array.swap(j, j + 1);
                sorting.operations.movs += 3;
                sorting.operations.comp += 1;
                sorting.operations.time = start.elapsed().as_millis();
                
               
                renderInterface(sorting, SortType::TwoRange(j, j + 1));
                sleep(Duration::from_millis(sorting.get_delay()));
                
            }
        }
    }
    
    // Finalizar com o array ordenado
    for i in 0..sorting.array.len() {
        renderInterface(sorting, SortType::RangeUnique(i + 1));
    }
}
