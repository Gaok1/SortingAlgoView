use crate::Tools::Op::Matriz::{SortType, print_Matriz, Operations};
use crate::Tools::Op::Constantes::*;

pub fn selectionSort(array: &mut [usize;width], matriz: &mut [[&str;width];height]){
    let mut op = Operations{time:0, movs:0, comp:0};
    let start = std::time::Instant::now();
    for i in 0..array.len(){
        let mut min = i;
        for j in i+1..array.len(){
            if array[j] < array[min]{
                min = j;
                op.comp += 1;
            }
            if(j % delay*4 == 0){
                print_Matriz(matriz, array, i, SortType::Range_Unique(j),&op);
            }
            
        }
        array.swap(i,min);
        op.movs += 3;
        op.time = start.elapsed().as_millis();
    }
    for i in 0..array.len(){
        print_Matriz(matriz, array, i+1, SortType::Range_Unique(i+1), &op);
    }
}