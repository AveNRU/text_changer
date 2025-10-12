/*
use std::io::{//self,
    BufRead, BufReader,// Error,
    Read, //Write
};
use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;
//use std::path::Path;
use std::fs::{self,
    //metadata,
    //File
};
//use std::fs::read_to_string;
use crate::lib_1::{self, Dictionary};
use lazy_static::lazy_static;
//use std::collections::HashMap;

use regex::Regex;
*/
/*
//output слов
   for i in 0..dictionary_lib.len() {
       for j in 0..dictionary_lib[i].простое.len() {
          // println!("j: {j}");
           for k in 0..dictionary_lib[i].простое[j].len () {
          // println!("слово: {}",&dictionary_lib[i].простое[j][k]);
           }
       }
   }
   //println!("{:?}",&dictionary_path_vec);
   */

use std::thread;
use std::time::Duration;

fn main() {
    let владение = thread::spawn(|| {
        for i in 1..10 {
            println!("Число {i} вызвано из порожденного потока!");
            thread::sleep(Duration::from_micros(1));
        }
    });

    for i in 1..5 {
        println!("Число {i} вызвано из основного потока!");
        thread::sleep(Duration::from_micros(1));
    }

    владение.join().unwrap();
}
