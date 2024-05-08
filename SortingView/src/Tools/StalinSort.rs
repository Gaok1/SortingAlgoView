use crate::Tools::Op::Matriz::{SortType, print_Matriz, Operations};
use crate::Tools::Op::Constantes::*;


//stalinSort
pub fn sort(array: &mut [usize;width], matriz: &mut [[&str;width];height]){
    let mut newarray = copyArray(array);

    let mut op = Operations{time:0, movs:0, comp:0};
    let start = std::time::Instant::now();
    let mut i = 0;
    let mut size = array.len();
    while i < size{
        if i > 0 && newarray[i] < newarray[i-1]{
            remove(&mut newarray,i);
            size-=1;
            op.movs += 1;
            op.comp += 1;
            op.time = start.elapsed().as_millis();
            print_Matriz(matriz, &newarray,  SortType::RangeUnique(i), &op);
        }else{
            i += 1;
        }
    }
    for i in 0..newarray.len(){
        print_Matriz(matriz, &newarray,  SortType::RangeUnique(i+1), &op);
    }
}

fn copyArray(array: &[usize;width]) -> [usize;width]{
    let mut copy = [0;width];
    for i in 0..array.len(){
        copy[i] = array[i];
    }
    copy
}

fn remove(array: &mut [usize;width], index: usize){
    array[index] = 0;
    for i in index..array.len()-1{
        array.swap(i,i+1);
    }
}
