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

use Tools::CocktailShaker;
use Tools::InserctionSort;
use Tools::MergeSort;
use Tools::Op::Constantes::RED;
use Tools::Op::Matriz::shuffle;
use Tools::Op::Constantes::{width, height, AMBER, RESET_COLOR, hide_cursor};
use Tools::QuickSort;

use Tools::SelectionSort;
use Tools::StalinSort;

fn clear(){
    print!("\x1B[2J\x1B[1;1H");
    
}

fn main() {
    let mut matriz : [[&str;width];height] = [[" ";width];height];
    let mut array = generate_random_array();
    

    loop {

        println!("\nChoose an Option:");
        println!("-0. Bogo Sort (Don't do it !!!!!)");
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
                println!("Shuffle");
                shuffle(&mut array, &mut matriz);
            }
            "1" => {
                println!("Selection Sort Selected!");
                SelectionSort::sort(&mut array, &mut matriz)
            }
            "2" => {
                println!("Insertion Sort Selected!");
                InserctionSort::sort(&mut array, &mut matriz)
            }
            "-0" =>{
                println!("Bogo Sort Selected!");
                BogoSort::sort(&mut array, &mut matriz);
            }
            "3" => {
                println!("Bubble Sort Selected!");
                BubbleSort::sort(&mut array, &mut matriz);
            }
            "4" => {
                println!("Quick Sort Selected!");
                QuickSort::sort(&mut array, &mut matriz);
            }
            "5" => {
                println!("Stalin Sort Selected!");
                StalinSort::sort(&mut array, &mut matriz);
            }
            "6" => {
                println!("Cocktail Shaker Sort Selected!");
                CocktailShaker::sort(&mut array, &mut matriz);
            }
            "7" => {
                println!("Merge Sort Selected!");
                MergeSort::sort(&mut array, &mut matriz);
            }
            "q" => {
                println!("Saindo...");
                break;
            }
            _ => println!("{} is invalid! {}",RED,RESET_COLOR),
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


