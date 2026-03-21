#![allow(non_ascii_idents)]
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
use crate::sync::mpsc;
use chrono::*;
//use unirust::*;
//use std::collections::HashMap;
use cap::Cap;
use std::alloc;
use std::env;
use std::time::{
    //Duration,
    Instant,
};
use tokio::*;
use xml::Encoding::Default;

pub mod check_1;
pub mod dictionary_0;
pub mod import;
pub mod lib;
pub mod output;
pub mod test_0;
pub mod utils;
pub mod xlsx;
//use time::*; //{self,OffsetDateTime};
use crate::output::write;
use crate::utils::functions_add::system_pause;
use console::{Emoji, style};
use rayon::scope;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());
use std::thread;
#[tokio::main] // или #[async_std::main]
async fn main() {
    use std::default::Default;
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    unsafe { env::set_var("RUST_BACKTRACE", "full") };
    unsafe { env::set_var("RUSTFLAGS", "-Awarnings") };
    unsafe { env::set_var("RUSTFLAGS", "-A dead_code") };
    // Текущие дата и время
    let текущая_время_дата: DateTime<Local> = Local::now();
    let исполнение = env!("CARGO_PKG_VERSION");
    let название = env!("CARGO_PKG_DESCRIPTION");
    let разработчики = env!("CARGO_PKG_DESCRIPTION");
    println!(
        "|{}| Исполнение # {} от {}",
        название,
        исполнение,
        текущая_время_дата.format("%d-%m-%Y время: %H:%M:%S")
    );
    let mut сообщения: lib::Сообщения = Default::default();
    //подсчёт начала запуска времени
    //начало нового
    let время_отсчёта: Instant = Instant::now();

    //проверка файлов и папок
    check_1::проверка_содержимого_папок();
    //книги
    //println!("Выделено памяти2(main)3: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    let исходная_книга: (Vec<lib::Книги>, usize) =
        import::read::считать_книги(&mut сообщения);
    //словари
    //println!("Выделено памяти(main)3: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    //словарь со словами в виде заглвных букв и маленьких
    let mut полный_словарь: lib::Полный_Словарь =
        xlsx::import_xlsx::загрузка_словарей(исходная_книга.1);
    //сохранение исходных книг - с разделениями
    let книги_вывод = исходная_книга.0.clone();
    //замена слов в книге
    //сама замена слов
    //println!("Выделено памяти(main)4: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    let итог_замены_слов_в_книгах: (Vec<lib::Книги>, lib::Сообщения) =
        dictionary_0::заменить_слова_в_книге_и_их_вывод(
            полный_словарь,
            исходная_книга.0,
            сообщения,
        )
        .await;
    let выходные_книги: Vec<lib::Книги> = итог_замены_слов_в_книгах.0;
    let mut сообщения: lib::Сообщения = итог_замены_слов_в_книгах.1;
    //
    /*let (tx,mut rx) = mpsc::unbounded_channel();
        let handle = thread::spawn(move|| {
    */
    let результат = write::сохранить_книги_с_разделениями(книги_вывод).unwrap();
    //println!("Прошёл шаг!!!!!!!!");
    /*      tx.send(результат).unwrap_or(());
    });
    let result = handle.join().unwrap();
    let сообщения2=match rx.try_recv() {
        Ok(сообщения) => сообщения,
        Err(ошибка) => {println!("Сохранить книги с разделителями еще не готов : {}",ошибка); Default::default()},
    };*/
    //
    // сообщения.вложить(сообщения2);
    сообщения.вложить(результат);
    //println!("Выделено памяти(main)5: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    //write::сохранить_книги(&выходные_книги, &mut сообщения).unwrap();

    //время затраченное в итоге
    //вывод сообщений
    //println!("Выделено памяти(main)6: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    write::вывод_всей_стопки_сообщений_в_txt(сообщения).unwrap();
    //output времени затраченного в итоге
    println!(
        "{}",
        style(format!(
            "⌚  Время занятое всего выполнения (от начала до конца): {:.2?}",
            время_отсчёта.elapsed()
        ))
        .true_color(154, 136, 252)
        .blink()
    );

    system_pause();
    println!();
}
