//use std::thread;

use crate::Tools::Op::Matriz::{SortType, print_Matriz, Operations};
use crate::Tools::Op::Constantes::*;

// use std::time::Duration;

pub fn sort(array: &mut [usize; width], matriz: &mut [[&str; width]; height]) {
    let mut op = Operations { time: 0, movs: 0, comp: 0 };
    let start: std::time::Instant = std::time::Instant::now();
    merge_sort(array, 0, array.len() - 1, matriz, &mut op,start);
    op.time = start.elapsed().as_millis();
    for i in 0..array.len() {
        print_Matriz(matriz, array,  SortType::RangeUnique(i + 1), &op);
    }
}

fn merge_sort(array: &mut [usize; width], l: usize, r: usize, matriz: &mut [[&str; width]; height], op: &mut Operations,start: std::time::Instant) {
    if l < r {
        let m = l + (r - l) / 2;
        merge_sort(array, l, m, matriz, op,start);
        merge_sort(array, m + 1, r, matriz, op,start);
        merge(array, l, m, r, matriz, op,start);
    }
}

fn merge(array: &mut [usize; width], l: usize, m: usize, r: usize, matriz: &mut [[&str; width]; height], op: &mut Operations,start: std::time::Instant) {
    let n1 = m - l + 1;
    let n2 = r - m;

    let mut L = vec![0; n1];
    let mut R = vec![0; n2];

    for i in 0..n1 {
        L[i] = array[l + i];
    }
    for i in 0..n2 {
        R[i] = array[m + 1 + i];
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < n1 && j < n2 {
        if L[i] <= R[j] {
            array[k] = L[i];
            i += 1;
        } else {
            array[k] = R[j];
            j += 1;
        }
        k += 1;
        op.movs += 1;
        op.comp += 1;
        op.time = start.elapsed().as_millis();
        if k % delay == 0 {
            print_Matriz(matriz, array,  SortType::RangeUnique(k), op);
        }
    }

    while i < n1 {
        array[k] = L[i];
        i += 1;
        k += 1;
        op.time = start.elapsed().as_millis();
        op.movs += 1;
        if k % delay == 0 {
            print_Matriz(matriz, array,  SortType::RangeUnique(k), op);
        }
    }

    while j < n2 {
        array[k] = R[j];
        j += 1;
        k += 1;
        op.time = start.elapsed().as_millis();
        op.movs += 1;
        if k % delay == 0 {
            print_Matriz(matriz, array,  SortType::RangeUnique(k), op);
        }
    }
}
