use crate::Tools::Op::Constantes::*;
use rand::Rng;
use std::{io::{self, Write}, thread};

/// # Enum SortType
/// it's an enum that contains the types of sorting printing
/// - RangeUnique(usize): print a unique index in red
/// - TwoRange(usize,usize): print two indexes in red and cyan
/// - Void: print the matrix without any special index
pub enum SortType {
    RangeUnique(usize), 
    TwoRange(usize,usize),
    Void,
}
/**
 * # Struct Operations
 * it's a struct that contains the time, movs and comp operations
 */
pub struct Operations{
    pub time: u128,
    pub movs: u64,
    pub comp: u64,
}



/// Print a matrix along with additional sorting information.
///
/// # Arguments
///
/// * `matriz`: A mutable reference to a matrix of strings to be printed.
/// * `array`: A reference to an array of integers to be printed.
/// * `sort_type`: The type of printing that will be done (1 index or 2 indexes).
/// * `data`: A reference to operations that will be printed.
///
/// # Example
///
/// ```rust
/// use crate::Tools::Op::Matriz::{SortType, print_Matriz, Operations};
/// use crate::Tools::Op::Constantes::*;
///
/// let mut matriz = [[" "; width]; height];
/// let mut array = [0; width];
/// let data = Operations { time: 0, movs: 0, comp: 0 };
/// print_Matriz(&mut matriz, &array, SortType::Range_Unique(0), &data);
/// ```
///
pub fn print_Matriz(matriz: &mut [[&str;width];height], array: &[usize;width] , sort_type:SortType, data:&Operations){
    
     let mut buffer =  format!(
            "⏱ Time: \x1b[1;34m[{} mls]\x1b[0m   🚀 Moves: \x1b[1;32m[{}]\x1b[0m   🎯 Comparations: \x1b[1;33m[{}]\x1b[0m",
            data.time, data.movs, data.comp
        );
    
    for j in 0..width{ //
        let value = array[j];
        let grey_area = height - value;
        match sort_type{
            SortType::RangeUnique(special)=>{
                if j != special{
                    for i in grey_area..height{
                        matriz[i][j] = digit;
                    }
                }else{
                    for i in grey_area..height{
                        matriz[i][j] = digit_red;
                    }
                }
            }
            SortType::TwoRange(index,temp ) =>{
                if j == index{
                    for i in grey_area..height{
                        matriz[i][j] = digit_red;
                    }
                }
                else if j == temp{
                    for i in grey_area..height{
                        matriz[i][j] = digit_cyan;
                    }
                }
                else {
                    for i in grey_area..height{
                        matriz[i][j] = digit;
                    }
                }
            }
            SortType::Void =>{
                for i in grey_area..height{
                    matriz[i][j] = digit;
                }
            }
        }
        for i in 0..grey_area {
            matriz[i][j] = " ";
        }
           
    }
    for i in 0..height{
        for j in 0..width{
            //concat buffer e matriz
            buffer+=matriz[i][j];
        }
        buffer+="\n";     
    }
    print!("{}",buffer);
    //io::stdout().flush().unwrap();
    buffer = String::from("");

    for i in 0..height+1{
        buffer+= back_up;
    }
    print!("{}\r",buffer);

}



pub fn shuffle(array: &mut [usize;width], matriz: &mut [[&str;width];height]){
    let op = Operations{time:0, movs:0, comp:0};
    let mut gen = rand::thread_rng();
    for i in 0..array.len(){
        let j = gen.gen_range(0..array.len());
        array.swap(i,j);
        if i % delay == 0{
            print_Matriz(matriz, array, SortType::RangeUnique(i), &op);
        }
    }
    print_Matriz(matriz, array, SortType::Void, &op);
}



