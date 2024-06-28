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
    pub mod CocktailShaker;
    pub mod MergeSort;
}

use Tools::BogoSort;
use Tools::BubbleSort;

use crate::Tools::CocktailShaker;
use crate::Tools::InserctionSort;
use crate::Tools::MergeSort;
use crate::Tools::Op::Constantes::RED;
use crate::Tools::Op::Matriz::shuffle;
use crate::Tools::Op::Constantes::{width, height, AMBER, RESET_COLOR, hide_cursor};
use crate::Tools::QuickSort;

use crate::Tools::SelectionSort;
use crate ::Tools::StalinSort;

fn clear(){
    print!("\x1B[2J\x1B[1;1H");
    print!("{}",hide_cursor);
}

fn main() {
    let mut matriz : [[&str;width];height] = [[" ";width];height];
    let mut array = generate_random_array();
    

    loop {
        
        println!("\nEscolha uma opção:");
        println!("-0. Bogo Sort (Não recomendado!!!!!)");
        println!(" 0. Shuffle");
        println!(" 1. Selection Sort");
        println!(" 2. Insertion Sort");
        println!(" 3. Bubble Sort");
        println!(" 4. Quick Sort");
        println!(" 5. Stalin Sort");
        println!(" 6. Cocktail Shaker Sort");
        println!(" 7. Merge Sort");
        println!(" q. Sair");

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
                SelectionSort::sort(&mut array, &mut matriz)
            }
            "2" => {
                println!("Selecionou Insertion Sort");
                InserctionSort::sort(&mut array, &mut matriz)
            }
            "-0" =>{
                println!("Selecionou Bogo Sort");
                BogoSort::sort(&mut array, &mut matriz);
            }
            "3" => {
                println!("Selecionou Bubble Sort");
                BubbleSort::sort(&mut array, &mut matriz);
            }
            "4" => {
                println!("Selecionou Quick Sort");
                QuickSort::sort(&mut array, &mut matriz);
            }
            "5" => {
                println!("Selecionou Stalin Sort");
                StalinSort::sort(&mut array, &mut matriz);
            }
            "6" => {
                println!("Selecionou Cocktail Shaker Sort");
                CocktailShaker::sort(&mut array, &mut matriz);
            }
            "7" => {
                println!("Selecionou Merge Sort");
                MergeSort::sort(&mut array, &mut matriz);
            }
            "q" => {
                println!("Saindo...");
                break;
            }
            _ => println!("{} Opção inválida! {}",RED,RESET_COLOR),
        }
        print!("{}",RESET_COLOR);
        //limpar stdin
    }
}



///Generate a random array of usize with a fixed width
/// ## Based on the width, the array will be generated with random values
fn generate_random_array() -> [usize;width]{
    let mut array = [0;width];
    let mut gen = rand::thread_rng();
    for i in 0..width{
        array[i] = gen.gen_range(1..height);
    }
    array
}


