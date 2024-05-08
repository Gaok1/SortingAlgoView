use crate::Tools::Op::Constantes::*;
use rand::Rng;
use std::{io::{self, Write}, thread};


pub enum SortType {
    Range, //cor unica
    Range_Unique(usize), // a cor varia de 0 a range e um especial (caso de inserctionSort)
    two_range(usize,usize),
}

pub struct Operations{
    pub time: u128,
    pub movs: u64,
    pub comp: u64,
}

fn get_index(sort_t:&SortType,sorted_mark:&usize) -> usize{
    match sort_t{
        SortType::Range => *sorted_mark,
        SortType::Range_Unique(index) => *index,
        SortType::two_range(index,temp ) => *index
    }
}

pub fn print_Matriz(matriz: &mut [[&str;width];height], array: &[usize;width] , sorted_mark:usize, sort_type:SortType, data:&Operations){
    
    let index = get_index(&sort_type,&sorted_mark);
     let mut buffer =  format!(
            "⏱ Tempo: \x1b[1;34m[{} mls]\x1b[0m   🚀 Movimentações: \x1b[1;32m[{}]\x1b[0m   🎯 Comparações: \x1b[1;33m[{}]\x1b[0m",
            data.time, data.movs, data.comp
        );
    if(index <width){
       buffer+= format!("Array[index]: \x1b[1;32m[{}]\x1b[0m",array[index]).as_str();
    }
    
    for j in 0..width{ //
        let color:&str = digit;
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
                        matriz[i][j] = digit_red;
                    }
                }
            }
            SortType::two_range(index,temp ) =>{
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
                        matriz[i][j] = color;
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
            print_Matriz(matriz, array, i+1, SortType::Range, &op);
        }
    }
    print_Matriz(matriz, array, array.len(), SortType::Range, &op);
}



