use crate::Tools::Op::Matriz::{SortType, print_Matriz, Operations};
use crate::Tools::Op::Constantes::*;




pub fn sort(array: &mut [usize;width], matriz: &mut [[&str;width];height]){
    let mut op = Operations{time:0, movs:0, comp:0};
    let start = std::time::Instant::now();
    for i in 1..array.len(){
        let mut j = i;
        while j > 0 && array[j] < array[j-1]{
            array.swap(j,j-1);
            j -= 1;
            op.movs += 3;
            op.comp += 2;
            op.time = start.elapsed().as_millis();
            if j%delay == 0{
                print_Matriz(matriz, array,  SortType::RangeUnique(j),&op);
            }
            
        }
    }
    for i in 0..array.len(){
        print_Matriz(matriz, array,  SortType::RangeUnique(i+1), &op);
    }
}