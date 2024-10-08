use crate::Tools::Op::interface::{SortType, print_Matriz, Sorting};
use crate::Tools::Op::Constantes::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn sort(sorting: &mut Sorting) {
    let start = Instant::now();
    quickSort(sorting, 0, sorting.array.len() - 1);
    // Finalizar com o array ordenado
    for i in 0..sorting.array.len() {
        print_Matriz(sorting, SortType::RangeUnique(i + 1));
    }

}


fn quickSort(array:  &mut Sorting, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let pivot = array.array[(left+right)/2]; // Escolhe o pivô como o elemento do meio
    print_Matriz(array, SortType::RangeUnique((left+right)/2));
    
    let mut i = left;
    let mut j = right;

    while i <= j {
        // Encontra o primeiro elemento à direita menor ou igual ao pivô
        while array.array[i] < pivot {
            i += 1;
            array.operations.comp += 1;
            print_Matriz(array, SortType::TwoRange(i, pivot));
        }

        print_Matriz(array, SortType::TwoRange(i, pivot));

        // Encontra o primeiro elemento à esquerda maior ou igual ao pivô
        while array.array[j] > pivot {
            j -= 1;
            array.operations.comp += 1;
            print_Matriz(array, SortType::TwoRange(j, pivot));
        }

        print_Matriz(array, SortType::TwoRange(j, pivot));

        // Troca os elementos
        if i<=j {
            print_Matriz(array, SortType::TwoRange(i, j));
            array.array.swap(i, j);
            array.operations.movs += 3;
            print_Matriz(array, SortType::TwoRange(i, j));
            i += 1;
            if j != 0{
                j-=1
            }
        }
    }
    if left < j {
        quickSort(array, left, j);
    }
    if i < right {
        quickSort(array, i, right);
    }
}