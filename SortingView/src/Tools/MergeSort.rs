use std::thread::sleep;
use std::time::Duration;

use crate::Tools::Op::interface::{renderInterface, SortType, Sorting};
use crate::Tools::Op::Constantes::*;

// Função principal de ordenação que agora usa a struct `Sorting`
pub fn sort(sorting: &mut Sorting) {
    let start: std::time::Instant = std::time::Instant::now();
    merge_sort(sorting, 0, sorting.array.len() - 1, start);
    sorting.operations.time = start.elapsed().as_millis();
    for i in 0..sorting.array.len() {
        renderInterface(sorting, SortType::RangeUnique(i + 1));
    }
}

// Função merge_sort ajustada para receber a struct `Sorting`
fn merge_sort(sorting: &mut Sorting, l: usize, r: usize, start: std::time::Instant) {
    if l < r {
        let m = l + (r - l) / 2;
        merge_sort(sorting, l, m, start);
        merge_sort(sorting, m + 1, r, start);
        merge(sorting, l, m, r, start);
    }
}

// Função merge ajustada para usar a struct `Sorting`
fn merge(sorting: &mut Sorting, l: usize, m: usize, r: usize, start: std::time::Instant) {
    let n1 = m - l + 1;
    let n2 = r - m;

    let mut L = vec![0; n1];
    let mut R = vec![0; n2];

    for i in 0..n1 {
        L[i] = sorting.array[l + i];
    }
    for i in 0..n2 {
        R[i] = sorting.array[m + 1 + i];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < n1 && j < n2 {
        if L[i] <= R[j] {
            sorting.array[k] = L[i];
            i += 1;
        } else {
            sorting.array[k] = R[j];
            j += 1;
        }
        k += 1;
        sorting.operations.movs += 1;
        sorting.operations.comp += 1;
        sorting.operations.time = start.elapsed().as_millis();

        renderInterface(sorting, SortType::RangeUnique(k));
        sleep(Duration::from_millis(sorting.get_delay()));
    }

    while i < n1 {
        sorting.array[k] = L[i];
        i += 1;
        k += 1;
        sorting.operations.time = start.elapsed().as_millis();
        sorting.operations.movs += 1;

        renderInterface(sorting, SortType::RangeUnique(k));
        sleep(Duration::from_millis(sorting.get_delay()));
    }

    while j < n2 {
        sorting.array[k] = R[j];
        j += 1;
        k += 1;
        sorting.operations.time = start.elapsed().as_millis();
        sorting.operations.movs += 1;

        renderInterface(sorting, SortType::RangeUnique(k));
        sleep(Duration::from_millis(sorting.get_delay()));
    }
}
