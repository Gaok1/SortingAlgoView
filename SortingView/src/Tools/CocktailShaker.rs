use crate::Tools::Op::Matriz::{SortType, print_Matriz, Operations};
use crate::Tools::Op::Constantes::*;

use std::thread;
use std::time::Duration;

/** Cocktail Shaker Sort
 * @param array: &mut [usize;width] - Array to be sorted
 * @param matriz: &mut [[&str;width];height] - Matrix to be printed
 * @return void
 */
pub fn sort(array: &mut [usize;width], matriz: &mut [[&str;width];height]) -> () {
    let this_delay = delay * 2;
    let mut op = Operations { time: 0, movs: 0, comp: 0 };
    let start_time = std::time::Instant::now();
    let mut swapped = true;
    let mut start = 0;
    let mut end = array.len();

    while swapped {
        swapped = false;

        // Fase ascendente
        for i in start..end - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
                op.movs += 3;
            }
            op.comp += 1;
            if i % this_delay == 0 {
                print_Matriz(matriz, array,  SortType::TwoRange(i,i+1), &op);
                
            }
        }

        // Se não houve troca, saia do loop
        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        // Fase descendente
        for i in (start..end - 1).rev() {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
                op.movs += 3;
            }
            op.comp += 1;
            if i % this_delay == 0 {
                print_Matriz(matriz, array,  SortType::TwoRange(i,i+1), &op);
                
            }
        }
        start += 1;
    }

    op.time = start_time.elapsed().as_millis();
    for i in 0..array.len() {
        print_Matriz(matriz, array,  SortType::RangeUnique(i + 1), &op);
        
    }
}