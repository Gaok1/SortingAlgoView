use crate::Tools::Op::Matriz::{SortType, print_Matriz, Operations};
use crate::Tools::Op::Constantes::*;

pub fn quickSort(array: &mut [usize;width], matriz: &mut [[&str;width];height]) {
    let mut op = Operations{time: 0, movs: 0, comp: 0};
    let start = std::time::Instant::now();
    quick_sort_helper(array, 0, array.len() - 1, matriz, &mut op, start);
    for i in 0..array.len(){
        print_Matriz(matriz, array, i+1, SortType::Range_Unique(i+1), &op);
    }
}

fn quick_sort_helper(array: &mut [usize;width], low: usize, high: usize, matriz: &mut [[&str;width];height], op: &mut Operations, start: std::time::Instant) {
    if low < high {
        let pivot_index = partition(array, low, high, op, start,matriz);
        if pivot_index > 0 {
            print_Matriz(matriz, array, pivot_index, SortType::Range_Unique(pivot_index), op);
        }
        quick_sort_helper(array, low, pivot_index.saturating_sub(1), matriz, op, start);
        quick_sort_helper(array, pivot_index + 1, high, matriz, op, start);
    }
}

fn partition(array: &mut [usize;width], low: usize, high: usize, op: &mut Operations, start: std::time::Instant, matriz: &mut [[&str;width];height]) -> usize {
    let pivot = array[high];
    let mut i = low;
    for j in low..high {
        if array[j] <= pivot {
            array.swap(i, j);
            i += 1;
            op.movs += 3;
        }
        op.comp += 1;
        op.time = start.elapsed().as_millis();
        if(j%6 == 0){
            print_Matriz(matriz, array, j, SortType::Range_Unique(j), op);
        }
    }
    array.swap(i, high);
    op.movs += 3;
    op.time = start.elapsed().as_millis();
    print_Matriz(matriz, array, high, SortType::Range_Unique(high), op);
    i
}
