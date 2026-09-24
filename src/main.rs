#![allow(non_ascii_idents)]
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
//use crate::sync::mpsc;
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
//use tokio::*;
//use xml::Encoding::Default;

pub mod check;
pub mod dictionary;
pub mod import;
//pub mod lib;
pub mod output;
pub mod test;
pub mod ui_gpui;
pub mod utils;
pub mod xlsx;
//use time::*; //{self,OffsetDateTime};
use crate::output::write;
use crate::utils::functions_add::system_pause;
use console::style;
//use rayon::scope;
//

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());
//
use crate::ui_gpui::gpui_main::*;
//use gpui_kit::assets::Assets;
use gpui_kit::component::{
    //button::Button,
    //h_flex,
    //switch::Switch,
    //text::{TextView, TextViewState},
    //v_flex,
    *,
};
use gpui_kit::*;
fn main() {
    let оболочка = gpui_kit::application().with_assets(gpui_kit::assets::Assets);

    оболочка.run(move |содержимое| {
        // This must be called before using any GPUI Component features.
        gpui_kit::init(содержимое);
        содержимое.activate(true);

        содержимое
            .spawn(async move |cx| {
                cx.open_window(
                    WindowOptions {
                        titlebar: Some(TitlebarOptions {
                            title: Some("Переводчик слова по словарю".into()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    |окно, cx| {
                        let отображение = cx.new(
                            |_| Данные_при_загрузке::default(), /*{
                                                                    включить_перевод: true,
                                                                    включить_разделители: false,
                                                                }*/
                        );
                        // This first level on the window, should be a Root.
                        cx.new(|cx| Root::new(отображение, окно, cx))
                    },
                )
                .expect("Failed to open window");
            })
            .detach();
    });
    println!("Конец");
}

//async
fn main2(
    данные_при_загрузке: Данные_при_загрузке
) -> Result<(), ()> {
    println!("|{:?}|", данные_при_загрузке);
    use Text_Changer::Вид_Словаря;
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
    let _разработчики = env!("CARGO_PKG_DESCRIPTION");
    println!(
        "|{}| Исполнение # {} от {}",
        название,
        исполнение,
        текущая_время_дата.format("%d-%m-%Y время: %H:%M:%S")
    );
    //
    let mut сообщения: Text_Changer::Сообщения = Default::default();
    //подсчёт начала запуска времени
    //начало нового
    let время_отсчёта: Instant = Instant::now();

    //проверка файлов и папок
    check::проверка_содержимого_папок();
    //книги
    //println!("Выделено памяти2(main)3: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    let исходные_книги: Text_Changer::Книги_в_ОЗУ =
        import::read::считать_книги(&mut сообщения, &данные_при_загрузке);
    //словари
    //println!("Выделено памяти(main)3: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    //словарь со словами в виде заглвных букв и маленьких
    let полный_словарь: Text_Changer::Словари_с_кучами =
        xlsx::import_xlsx::загрузка_словарей(
            исходные_книги.размер_в_озу,
            Вид_Словаря::Основной_Словарь,
            &данные_при_загрузке,
        );
    //
    let запасной_словарь: Text_Changer::Словари_с_кучами =
        xlsx::import_xlsx::загрузка_словарей(
            исходные_книги.размер_в_озу,
            Вид_Словаря::Запасной_Словарь,
            &данные_при_загрузке,
        );
    //
    test::сравнить_основной_и_запасной_словари(
        &полный_словарь.сам,
        &запасной_словарь.сам,
    )
    .unwrap();
    //сохранение исходных книг - с разделениями
    let книги_вывод = исходные_книги.книги.clone();
    //замена слов в книге
    //сама замена слов
    //println!("Выделено памяти(main)4: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    let итог_замены_слов_в_книгах: Text_Changer::Прогон_замены =
        dictionary::заменить_слова_в_книге_и_их_вывод(
            полный_словарь,
            исходные_книги.книги,
            сообщения,
            &данные_при_загрузке,
        );
    // .await;
    //let выходные_книги: Vec<Text_Changer::Книги> = итог_замены_слов_в_книгах.0;
    let mut сообщения: Text_Changer::Сообщения = итог_замены_слов_в_книгах.сообщения;
    //
    /*let (tx,mut rx) = mpsc::unbounded_channel();
        let handle = thread::spawn(move|| {
    */
    let итог = write::сохранить_книги_с_разделениями(
        книги_вывод,
        &данные_при_загрузке,
    )
    .unwrap();
    // сообщения.вложить(сообщения2);
    сообщения.вложить(итог);
    //println!("Выделено памяти(main)5: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    //write::сохранить_книги(&выходные_книги, &mut сообщения).unwrap();

    //время затраченное в итоге
    //вывод сообщений
    //println!("Выделено памяти(main)6: {}B, мегов: {}", ALLOCATOR.allocated(),ALLOCATOR.allocated()/1024);
    write::вывод_всей_стопки_сообщений_в_txt(
        сообщения,
        &данные_при_загрузке,
    )
    .unwrap();
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
    Ok(())
}
