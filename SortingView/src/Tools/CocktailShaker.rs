use crate::Tools::Op::interface::{SortType, renderInterface, Sorting};
use crate::Tools::Op::Constantes::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn sort(sorting: &mut Sorting) {
    let this_delay = sorting.get_delay() * 2;
    let start_time = Instant::now();
    let mut swapped = true;
    let mut start = 0;
    let mut end = sorting.array.len();

    while swapped {
        swapped = false;

        // Fase ascendente
        for i in start..end - 1 {
            if sorting.array[i] > sorting.array[i + 1] {
                sorting.array.swap(i, i + 1);
                swapped = true;
                sorting.operations.movs += 3;
            }
            sorting.operations.comp += 1;
           
            renderInterface(sorting, SortType::TwoRange(i, i + 1));
            sleep(Duration::from_millis(sorting.get_delay()));
            
        }
  
        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        // Fase descendente
        for i in (start..end - 1).rev() {
            if sorting.array[i] > sorting.array[i + 1] {
                sorting.array.swap(i, i + 1);
                swapped = true;
                sorting.operations.movs += 3;
            }
            sorting.operations.comp += 1;
           
                renderInterface(sorting, SortType::TwoRange(i, i + 1));
                sleep(Duration::from_millis(sorting.get_delay()));
            
        }
        start += 1;
    }

    sorting.operations.time = start_time.elapsed().as_millis();
    
    // Finalizar com o array ordenado
    for i in 0..sorting.array.len() {
        renderInterface(sorting, SortType::RangeUnique(i + 1));
    }
}
