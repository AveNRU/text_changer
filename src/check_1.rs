use crate::lib;
use rayon::prelude::*;
use std::sync::{Mutex, atomic::{AtomicU64, Ordering,AtomicUsize}};
pub fn проверка_содержимого() {
    use crate::output::functions::проверка_наличия_папок_в_случае_их_отсутствия_создать_папки;
    use crate::utils::read::попытка_открыть_файл;
    use std::fs::{
        self,
        //metadata, File
    };
    use std::path::Path;
    let пути_общие: lib::Пути_Общие = Default::default();
    //двумерный вектор. 1-й для хранения прямых путей, 2 - после пути добавляется косая черта / (linux)
    let mut пути: Vec<String> = vec![
            //test
            пути_общие.вывод.to_string(),
            пути_общие.книги.to_string(),
            пути_общие.словари.to_string(),
            пути_общие.вывод_книги.to_string(),
            пути_общие.вывод_словари.to_string(),
    ];
    пути.par_iter().enumerate().for_each(|(указатель, строка)|{
    //for i in 0..пути.len() {
            //проверка наличия папок
            //если не создано - создать
            проверка_наличия_папок_в_случае_их_отсутствия_создать_папки(&строка);
    }
    );
}
