use std::process::Command;
use std::vec;

use text_io::scan;

//random
use rand::Rng;

mod Tools{
    pub mod Op{
        pub mod Constantes;
        pub mod interface;
    }
    pub mod SelectionSort;
    pub mod InserctionSort;
    pub mod BogoSort;
    pub mod BubbleSort;
    pub mod QuickSort;
    pub mod StalinSort;
    pub mod CocktailShaker;
    pub mod MergeSort;
    pub mod HeapSort;
}

use Tools::BogoSort;
use Tools::BubbleSort;

use Tools::CocktailShaker;
use Tools::InserctionSort;
use Tools::MergeSort;
use Tools::Op::Constantes::RED;
use Tools::Op::interface::*;
use Tools::Op::Constantes::{AMBER, RESET_COLOR, hide_cursor};
use Tools::QuickSort;
use Tools::HeapSort;
use Tools::SelectionSort;
use Tools::StalinSort;


fn clear(){
    print!("\x1B[2J\x1B[1;1H");
    
}

const STANDARD_WIDTH: usize = 200;
const STANDARD_HEIGHT: usize = 50;

fn config_menu(sorting : &mut Sorting){
 //change width(quantity of numbers), height (max number value), delay
    println!("{}Status of Structure{}",AMBER,RESET_COLOR);
    println!("Width: {}",sorting.array.len());
    println!("Height: {}",sorting.matriz.len());
    println!("Delay: {}",sorting.get_delay());
    println!("Config Menu");
    println!("1. Change Width");
    println!("2. Change Height");
    println!("3. Change Delay");
    println!("5. Reset");
    println!("6. Back");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => {
            println!("Change Width");
            let mut width = String::new();
            std::io::stdin().read_line(&mut width).unwrap();
            let width: usize = width.trim().parse().unwrap();
            *sorting = Sorting::new(width, sorting.matriz.len());
        }
        "2" => {
            println!("Change Height");
            let mut height = String::new();
            std::io::stdin().read_line(&mut height).unwrap();
            let height: usize = height.trim().parse().unwrap();
            *sorting = Sorting::new(sorting.array.len(), height);
        }
        "3" => {
            println!("Change Delay");
            let mut delay = String::new();
            std::io::stdin().read_line(&mut delay).unwrap();
            let delay: u64 = delay.trim().parse().unwrap();
            sorting.set_delay(delay);
        }
        "5" => {
            println!("Reset");
            *sorting = Sorting::new(STANDARD_WIDTH, STANDARD_HEIGHT);
        }
        "6" => {
            println!("Backing...");
        }
        _ => println!("{} is invalid! {}",RED,RESET_COLOR),
    }
}
    

fn main() {
    let height = 50;
    let width = 200;
    print!("{hide_cursor}");
    let mut sorting_interface: Sorting = Sorting::new(width, height);
    
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
        println!(" 8. Heap Sort");
        println!(" *. config menu");
        println!(" q. Sair");
        
        let mut input = String::new();
        scan!("{}\n",input);
        clear();
        print!("{}",AMBER);
        match input.trim() {
            "0" =>{
                println!("Shuffle");
                shuffle(&mut sorting_interface);
            }
            "1" => {
                println!("Selection Sort Selected!");
                SelectionSort::sort(&mut sorting_interface)
            }
            "2" => {
                println!("Insertion Sort Selected!");
                InserctionSort::sort(&mut sorting_interface)
            }
            "-0" =>{
                println!("Bogo Sort Selected!");
                BogoSort::sort(&mut sorting_interface);
            }
            "3" => {
                println!("Bubble Sort Selected!");
                BubbleSort::sort(&mut sorting_interface);
            }
            "4" => {
                println!("Quick Sort Selected!");
                QuickSort::sort(&mut sorting_interface);
            }
            "5" => {
                println!("Stalin Sort Selected!");
                StalinSort::sort(&mut sorting_interface);
            }
            "6" => {
                println!("Cocktail Shaker Sort Selected!");
                CocktailShaker::sort(&mut sorting_interface);
            }
            "7" => {
                println!("Merge Sort Selected!");
                MergeSort::sort(&mut sorting_interface);
            }
            "8" => {
                println!("Heap Sort Selected!");
                HeapSort::sort(&mut sorting_interface);
            }
            "*"=>{
                config_menu(&mut sorting_interface);
            }
            "q" => {
                println!("Saindo...");
                break;
            }
            _ => println!("{} is invalid! {}",RED,RESET_COLOR),
        }

        print!("{}",RESET_COLOR);
        //limpar stdin
        sorting_interface.operations.reset();
        Command::new("cls").spawn();
    }
}

//delay : 4
//width : 250
//height : 75