use std::fs::{
    self,
    //metadata,
    File,
};
use std::io::{
    //self,
    BufRead,
    BufReader,
    Error,
    Read, //Write
};
use std::path::Path;
use std::{str, time::Instant};

pub fn проверка_наличия_папок_в_случае_их_отсутствия_создать_папки(
    путь: &String,
) {
    //проверка наличия папок
    if !Path::new(путь.as_str()).exists() {
        println!("успешно создана папка: {}", путь);
        fs::create_dir(путь.as_str()).unwrap(); // создаем папку
    }
    //чтение содержимого папки
    match fs::read_dir(путь.as_str()) {
        //если ошибка - вывод почему
        Err(причина) => println!("! {:?}", причина.kind()),
        //если успех - получение списка содержимого
        Ok(пути) => for путь2 in пути {},
    }
}
