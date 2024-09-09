use crate::Tools::Op::*;
use crate::Tools::Op::interface::{SortType, print_Matriz, Sorting};

// StalinSort
pub fn sort(sorting: &mut Sorting) {
    let mut newarray = copy_array(&sorting.array);

    let start = std::time::Instant::now();
    let mut i = 0;
    let mut size = newarray.len();
    
    while i < size {
        if i > 0 && newarray[i] < newarray[i - 1] {
            remove(&mut newarray, i);
            size -= 1;
            sorting.operations.movs += 1;
            sorting.operations.comp += 1;
            sorting.operations.time = start.elapsed().as_millis();
            print_Matriz(sorting, SortType::RangeUnique(i));
        } else {
            i += 1;
        }
    }

    for i in 0..newarray.len() {
        print_Matriz(sorting, SortType::RangeUnique(i + 1));
    }

    sorting.array = newarray; // Atualiza o array na estrutura com o array modificado
}

// Função que faz a cópia do array
fn copy_array(array: &Vec<usize>) -> Vec<usize> {
    array.clone() // Clona o array em vez de copiar manualmente
}

// Função que remove um elemento do array
fn remove(array: &mut Vec<usize>, index: usize) {
    array.remove(index); // Remove o elemento na posição especificada
}
