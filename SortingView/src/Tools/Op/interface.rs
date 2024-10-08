use crate::Tools::Op::Constantes::*;
use rand::Rng;
use std::{io::{self, Write}, thread, vec};

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

pub enum CharacterType{
    Red,
    White,
    Green,
    Cyan,
    None
}

/**
 * # Struct Operations
 *  it's a struct that contains the time, movs and comp operations
 */
pub struct Operations{
    pub time: u128,
    pub movs: u64,
    pub comp: u64,
}
impl Operations{
    pub fn reset(&mut self){
        self.time = 0;
        self.movs = 0;
        self.comp = 0;
    }

    fn new() -> Operations{
        Operations{
            time:0,
            movs:0,
            comp:0
        }
    }
}
pub struct Sorting{
    pub operations: Operations,
    pub array : Vec<usize>,
    pub matriz : Vec<Vec<CharacterType> >,
    delay: usize,
}

impl Sorting{
    pub fn new(width:usize, height:usize) -> Sorting{
        let mut sort =  Sorting{
            operations : Operations::new(),
            array: Vec::new(),
            matriz: Vec::new(),
            delay: 1
            };
        sort.inicialize_matriz(width, height);
        sort.array = Sorting::generate_random_array(width, height);
        sort
    }

    fn inicialize_matriz(&mut self, width: usize, height: usize){
        for _ in 0..height{
            let mut row = Vec::new();
            for _ in 0..width{
                row.push(CharacterType::None);
            }
            self.matriz.push(row);
        }
    }

    fn generate_random_array(len: usize, max:usize) -> Vec<usize>{
        let mut array: Vec<usize> = Vec::new();
        let mut gen = rand::thread_rng();
        for i in 0..len{
            array.push(gen.gen_range(1..max));
        }
        
        array
    }
    pub fn set_delay(&mut self, delay: usize){
        self.delay = delay;
    }

    pub fn get_delay(&mut self)-> usize{
        self.delay
    }
   

}



pub fn print_Matriz(sorting: &mut Sorting, sort_type: SortType) {
    let mut buffer = format!(
        "⏱ Time: \x1b[1;34m[{} mls]\x1b[0m   🚀 Moves: \x1b[1;32m[{}]\x1b[0m   🎯 Comparisons: \x1b[1;33m[{}]\x1b[0m",
        sorting.operations.time, sorting.operations.movs, sorting.operations.comp
    );
    let height = sorting.matriz.len();
    let width = sorting.array.len();

    for j in 0..width {
        let value = sorting.array[j];
        let grey_area = height - value;
        match sort_type {
            SortType::RangeUnique(special) => {
                for i in grey_area..height {
                    sorting.matriz[i][j] = if j == special { CharacterType::Red } else { CharacterType::White };
                }
            }
            SortType::TwoRange(index, temp) => {
                for i in grey_area..height {
                    sorting.matriz[i][j] = if j == index {
                        CharacterType::Red
                    } else if j == temp {
                        CharacterType::Cyan
                    } else {
                        CharacterType::White
                    };
                }
            }
            SortType::Void => {
                for i in grey_area..height {
                    sorting.matriz[i][j] = CharacterType::White;
                }
            }
        }
        for i in 0..grey_area {
            sorting.matriz[i][j] = CharacterType::None; // Espaços vazios
        }
    }

    // Converte os enums em seus respectivos caracteres ou cores para imprimir
    for i in 0..height {
        for j in 0..width {
            buffer += match sorting.matriz[i][j] {
                CharacterType::Red => digit_red,
                CharacterType::Cyan => digit_cyan,
                CharacterType::White => digit,
                CharacterType::None => " ", // Espaço vazio
                _ => " ", // Valor padrão para outros tipos
            };
        }
        buffer += "\n";
    }

    print!("{}", buffer);

    buffer.clear(); // Limpar buffer

    for _ in 0..height + 1 {
        buffer += back_up;
    }
    print!("{}\r", buffer);
}


pub fn shuffle(sorting: &mut Sorting) {
    let mut gen = rand::thread_rng();
    for i in 0..sorting.array.len() {
        let j = gen.gen_range(0..sorting.array.len());
        sorting.array.swap(i, j);
        if i % sorting.get_delay() == 0 {
            print_Matriz(sorting, SortType::RangeUnique(i));
        }
    }
    print_Matriz(sorting, SortType::Void);
}



