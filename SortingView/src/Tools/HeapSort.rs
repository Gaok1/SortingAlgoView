use crate::Tools::Op::interface::{renderInterface, SortType, Sorting};

use std::{
    thread::sleep,
    time::{Duration, Instant},
};

// Função para ajustar a posição para cima (Heapify Up)
fn check_position_up(sorting: &mut Sorting, idx: usize, start: &Instant) {
    if idx == 0 {
        return;
    }
    let mut current_idx = idx;
    while current_idx > 0 {
        let father_idx = (current_idx - 1) / 2;
        sorting.operations.comp += 1;

        if sorting.array[father_idx] < sorting.array[current_idx] {
            sorting.array.swap(father_idx, current_idx);
            sorting.operations.movs += 3; // swap conta como 3 movimentos
            current_idx = father_idx;
            sorting.operations.time = start.elapsed().as_millis();
            renderInterface(sorting, SortType::RangeUnique(current_idx));
            sleep(Duration::from_millis(sorting.get_delay()));
        } else {
            break;
        }
    }
}

// Função para ajustar a posição para baixo (Heapify Down)
fn check_position_down(sorting: &mut Sorting, idx: usize, last_idx: usize, start: &Instant) {
    let mut current_idx = idx;
    loop {
        let son1_idx = 2 * current_idx + 1;
        let son2_idx = 2 * current_idx + 2;
        let mut max_idx = current_idx;

        if son1_idx < last_idx {
            sorting.operations.comp += 1;
            if sorting.array[son1_idx] > sorting.array[max_idx] {
                max_idx = son1_idx;
            }
        }
        if son2_idx < last_idx {
            sorting.operations.comp += 1;
            if sorting.array[son2_idx] > sorting.array[max_idx] {
                max_idx = son2_idx;
            }
        }

        if max_idx != current_idx {
            sorting.array.swap(current_idx, max_idx);
            sorting.operations.movs += 3; // swap conta como 3 movimentos
            sorting.operations.time = start.elapsed().as_millis();

            // Verifica se deve imprimir a matriz

            renderInterface(sorting, SortType::RangeUnique(max_idx));
            sleep(Duration::from_millis(sorting.get_delay()));

            current_idx = max_idx;
        } else {
            break;
        }
    }
}

// Função para construir o Heap máximo
fn heapify(sorting: &mut Sorting, start: &Instant) {
    for i in 0..sorting.array.len() {
        check_position_up(sorting, i, start);
    }
}

// Função principal de Heap Sort adaptada
pub fn sort(sorting: &mut Sorting) {
    let start = Instant::now();

    heapify(sorting, &start);

    let mut last = sorting.array.len();
    while last > 1 {
        last -= 1;
        sorting.array.swap(0, last);
        sorting.operations.movs += 3; // swap conta como 3 movimentos
        sorting.operations.time = start.elapsed().as_millis();

        // Exibir a matriz após cada swap
        renderInterface(sorting, SortType::RangeUnique(last));
        sleep(Duration::from_millis(sorting.get_delay()));
        check_position_down(sorting, 0, last, &start);
    }

    for i in 0..sorting.array.len() {
        renderInterface(sorting, SortType::RangeUnique(i + 1));
    }
}
