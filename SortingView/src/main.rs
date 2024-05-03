//random
use rand::Rng;

mod Tools{
    pub mod Op{
        pub mod Constantes;
        pub mod Matriz;
    }
    pub mod SelectionSort;
    pub mod InserctionSort;
    pub mod BogoSort;
    pub mod BubbleSort;
    pub mod QuickSort;
    pub mod StalinSort;
}

use Tools::BogoSort;
use Tools::BubbleSort;

use crate::Tools::Op::Matriz::shuffle;
use crate::Tools::Op::Constantes::{width, height, AMBER, RESET_COLOR, hide_cursor};
use crate ::Tools::SelectionSort::selectionSort;
use crate ::Tools::InserctionSort::inserctionSort;
use crate ::Tools::QuickSort::quickSort;
use crate ::Tools::StalinSort::Sort;


fn clear(){
    print!("\x1B[2J\x1B[1;1H");
    print!("{}",hide_cursor);
}

fn main() {
    let mut matriz : [[&str;width];height] = [[" ";width];height];
    let mut array = generateRandomArray();
    

    loop {
        
        println!("Escolha uma opção:");
        println!("0. Shuffle");
        println!("1. Selection Sort");
        println!("2. Insertion Sort");
        println!("3. Bogo Sort");
        println!("4. Bubble Sort");
        println!("5. Quick Sort");
        println!("6. Stalin Sort");
        println!("9. Sair");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        clear();
        print!("{}",AMBER);
        match input.trim() {
            "0" =>{
                println!("Selecionou Shuffle");
                shuffle(&mut array, &mut matriz);
            }
            "1" => {
                println!("Selecionou Selection Sort");
                selectionSort(&mut array, &mut matriz)
            }
            "2" => {
                println!("Selecionou Insertion Sort");
                inserctionSort(&mut array, &mut matriz)
            }
            "3" =>{
                println!("Selecionou Bogo Sort");
                BogoSort::Sort(&mut array, &mut matriz);
            }
            "4" => {
                println!("Selecionou Bubble Sort");
                BubbleSort::sort(&mut array, &mut matriz);
            }
            "5" => {
                println!("Selecionou Quick Sort");
                quickSort(&mut array, &mut matriz);
            }
            "6" => {
                println!("Selecionou Stalin Sort");
                Sort(&mut array, &mut matriz);
            }
            "9" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida!"),
        }
        print!("{}",RESET_COLOR);
        //limpar stdin
     
    }
}





fn generateRandomArray() -> [usize;width]{
    let mut array = [0;width];
    let mut gen = rand::thread_rng();
    for i in 0..width{
        array[i] = gen.gen_range(1..height);
    }
    array
}


