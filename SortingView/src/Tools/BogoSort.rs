use crate::Tools::Op::Matriz::{SortType, print_Matriz, Operations, shuffle};
use crate::Tools::Op::Constantes::*;



pub fn sort(array: &mut [usize;width], matriz: &mut [[&str;width];height]){

    let mut op = Operations{time:0, movs:0, comp:0};
    let start = std::time::Instant::now();
    while !isSorted(array){
        shuffle(array, matriz);
        op.time = start.elapsed().as_millis();
        op.movs += 1;
        op.comp += 1;
        print_Matriz(matriz, array, SortType::Void, &op);
    }
    for i in 0..array.len(){
        print_Matriz(matriz, array, SortType::RangeUnique(i+1), &op);
    }
}

fn isSorted(array: &[usize;width]) -> bool{
    for i in 1..array.len(){
        if array[i] < array[i-1]{
            return false;
        }
    }
    true
}   