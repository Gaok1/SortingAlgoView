use crate::Tools::Op::Matriz::{SortType, print_Matriz, Operations};
use crate::Tools::Op::Constantes::*;

pub fn sort(array: &mut [usize;width], matriz: &mut [[&str;width];height]){
    let thisDelay = delay *2;
    let mut op = Operations{time:0, movs:0, comp:0};
    let start = std::time::Instant::now();
    for i in 0..array.len(){
        let mut min = i;
        for j in i+1..array.len(){
            if array[j] < array[min]{
                min = j;
            }
            op.comp += 1;
            if(j % thisDelay == 0){
                print_Matriz(matriz, array,  SortType::RangeUnique(j),&op);
            }
        }
        array.swap(i,min);
        op.movs += 3;
        op.time = start.elapsed().as_millis();
    }//sorted
    for i in 0..array.len(){
        print_Matriz(matriz, array,  SortType::RangeUnique(i+1), &op);
    }
}