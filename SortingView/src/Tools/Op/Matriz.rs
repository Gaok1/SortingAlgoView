use crate::Tools::Op::Constantes::*;
use rand::Rng;
use std::io::{self, Write};


pub enum SortType {
    Range, //cor unica
    Range_Unique(usize), // a cor varia de 0 a range e um especial (caso de inserctionSort)
}

pub struct Operations{
    pub time: u128,
    pub movs: u64,
    pub comp: u64,
}


pub fn print_Matriz(matriz: &mut [[&str;width];height], array: &[usize;width] , sorted_mark:usize, sort_type:SortType, data:&Operations){
    print!("Tempo: [{} mls] Movimentações: [{}] Comparações: [{}] ",data.time, data.movs, data.comp);
    
    for j in 0..width{ //
        let color:&str;
        if j < sorted_mark{
            color = GREEN;
        }else{
            color = WHITE;
        }
        let value = array[j];
        let grey_area = height - value;
        match sort_type{
            SortType::Range =>{
                for i in grey_area..height{
                    matriz[i][j] = color;
                }
            }
            SortType::Range_Unique(special)=>{
                if j != special{
                    for i in grey_area..height{
                        matriz[i][j] = color;
                    }
                }else{
                    for i in grey_area..height{
                        matriz[i][j] = CYAN;
                    }
                }
            }
        }
        for i in 0..grey_area {
            matriz[i][j] = " ";
        }
           
    }
    for i in 0..height{
        for j in 0..width{
            print!("{}",matriz[i][j]);
        }
        print!("\n");       
    }

    for i in 0..height{
        print!("{}",back_up);
    }
    print!("\r");

    io::stdout().flush().unwrap();
    
}



pub fn shuffle(array: &mut [usize;width], matriz: &mut [[&str;width];height]){
    let op = Operations{time:0, movs:0, comp:0};
    let mut gen = rand::thread_rng();
    for i in 0..array.len(){
        let j = gen.gen_range(0..array.len());
        array.swap(i,j);
            print_Matriz(matriz, array, i+1, SortType::Range, &op);
    }
}


