#![allow(non_ascii_idents)]

//use std::fs::read_to_string;
use crate::lib;
use crate::lib::Содержимое_папок;
use crate::utils::regex::{
    fb2_rtf_mhtml, fb3_epub, md_fs_yml, изображение_расширение
};
use crate::utils::zip::{VirtualFs, pack_zip_from_memory};
use encoding_rs::{
    WINDOWS_1251,
    //    DecoderResult
};
use rust_xlsxwriter::*;
use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{
    self,
    //BufRead, BufReader,
    Error,
    Write,
};
use std::path::Path;
use std::sync::Mutex;
//use xml::Encoding::Default;
use crate::utils::read::read_utf8;

pub fn сохранить_книгу(
    стопки_книг: &Vec<lib::Книги>,
    mut сообщения:&mut lib::Сообщения,
) -> Result<(), Error> {
    for i in 0..стопки_книг.len() {
        //путь до вывода
        //let path = format!("./end/{}.{}",i,book_struct[i].format);
        let пути_общие: lib::Пути_Общие = Default::default();
        //println!("путь до книги: {}",стопки_книг[i].путь);
        let путь_сохранения: String = стопки_книг[i]
            .путь
            .replace(&пути_общие.книги, &пути_общие.вывод_книги);
        //println!("путь до книги: {}",путь_сохранения);
        /*let путь = format!(
            "{}{}.{}",
            пути_общие.вывод_книги, стопки_книг[i].название_книги, стопки_книг[i].расширение
        );*/

        //output книги
        //если это не архивная книга
        if fb2_rtf_mhtml(&стопки_книг[i].путь) || md_fs_yml(&стопки_книг[i].путь)
        {
            //перебор книг
            for гл_указатель in 0..стопки_книг[i].вложения.len() {
                //сравнение образов
                match запись_если_есть_разница(
                    &путь_сохранения,
                    &стопки_книг[i].вложения[гл_указатель]
                       .содержимое,
                    &mut  сообщения.общие,
                ) {
                    Ok(true) => {
                        //println!("Внешне: Перезапись")
                    }
                    Ok(false) => {
                        //println!("Внешне: Отказ от перезаписи");
                        continue;
                    }
                    Err(e) => panic!("{}", e),
                }
                //указание на output
                let mut вывод = match File::create(&путь_сохранения) {
                    Ok(сообщение) => сообщение,
                    Err(e) => panic!(
                        "Не удалось создать файл книги: {} по причине: {}",
                        путь_сохранения, e
                    ),
                };
                //перебор содержимого - строк
                for строка in стопки_книг[i].вложения[гл_указатель].содержимое.iter()
                {
                    //если это rtf
                    if стопки_книг[i].расширение.contains("rtf") {
                        let (_windows_1251_bytes, _, _) = WINDOWS_1251.encode(&строка);
                        // Преобразование UTF-8 → Windows-1251
                        let windows1251_bytes = utf8_to_windows1251(&строка);
                        let (_s, _, had_errors) = WINDOWS_1251.decode(&windows1251_bytes);
                        if had_errors {
                            println!("Были ошибки декодирования");
                        }
                        вывод.write_all(&windows1251_bytes).unwrap();
                    }
                    //если не RTF расширение
                    else {
                        writeln!(вывод, "{}", строка).unwrap();
                    }
                }
            }
        }
        //если это архивное разрешение
        else if fb3_epub(&стопки_книг[i].путь) {
            //вторая FS virtual чтобы собрать в файл .zip в виде Vec<u8>
            let mut вторичная_fs_в_озу: VirtualFs = HashMap::new();
            //перебор содержимого архива
            for k in 0..стопки_книг[i].вложения.len() {
                //println!("имя: {}",book_struct[i].file[k].name );
                //1 -имя, 2 - содержимое в hashmap
                let mut содержимое_байты: Vec<u8> = Vec::new();
                //перебор содержимого книги из String в UTF8
                //если это рисунок
                if изображение_расширение(&стопки_книг[i].вложения[k].имя)
                {
                    содержимое_байты = стопки_книг[i].вложения[k].изображение.clone();
                }
                //если это не картинки
                else {
                    for k2 in 0..стопки_книг[i].вложения[k].содержимое.len()
                    {
                        содержимое_байты.extend(
                            стопки_книг[i].вложения[k].содержимое[k2]
                                .as_bytes()
                                .to_vec(),
                        );
                    }
                }
                //вложение в словарь
                вторичная_fs_в_озу.insert(стопки_книг[i].вложения[k].имя.clone(), содержимое_байты);
            }

            //Запаковывает виртуальную файловую систему в Vec<u8>
            let временный_путь = match pack_zip_from_memory(&вторичная_fs_в_озу) {
                Ok(путь) => путь,
                Err(причина) => panic!("{:?}", причина),
            };
            //output в файл
            write_book_fs(&временный_путь, &стопки_книг[i]).unwrap();
        }
    }
    Ok(())
}

pub fn write_book_fs(_vec_u8: &Vec<u8>, book_struct: &lib::Книги) -> Result<(), Error> {
    use std::default::Default;
    // for i in 0..book_struct.len() {
    //путь до вывода
    let пути_общие:lib::Пути_Общие=Default::default();
    //let path = format!("./end/{}.{}",i,book_struct[i].format);
    let путь = format!(
        "{}{}.{}",
        пути_общие.вывод_книги
        ,book_struct.название_книги, book_struct.расширение
    );
    //let path = format!("./end/1.docx", );
    //указание на output
    let mut output = File::create(путь).unwrap();
    //output книги
   output.write_all(_vec_u8).unwrap();
    Ok(())
}
//из utf8 в Windows 1251 для RTF
fn utf8_to_windows1251(utf8_str: &str) -> Vec<u8> {
    let (итог, _, had_errors) = WINDOWS_1251.encode(utf8_str);
    if had_errors {
        // Обработка символов, которые не могут быть представлены в Windows-1251
        eprintln!("Некоторые символы не могут быть представлены в Windows-1251");
    }
    итог.into_owned()
}

//output словарей
pub fn excel_dictionary_write(
    ряд_словарей: &Vec<lib::Словарь>,
    //mode: String,           //Стопка из файла .xlsx взята или самостоятельно высчитана
    //path_name_spd: &String, //имя .spd файла
) -> Result<(), XlsxError> {
    for i in 0..ряд_словарей.len() {
        // Create a new Excel file object.
        let mut рабочая_книга = Workbook::new();
        // Add a worksheet to the workbook.
        let mut рабочая_страница = рабочая_книга
            .add_worksheet()
            .set_name("Простые слова")
            .unwrap();
        вывод_заголовков_на_странице(&mut рабочая_страница).unwrap();
        //worksheet.write(0, 5, "Ток потребления").unwrap();
        //worksheet.write(0, 6, "Цепь земли (по умолчанию)").unwrap();
        let mut последняя_строка: u32 = u32::try_from(1).unwrap().into();
        //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
        //перебор всех словарей

        //если все слова равны
        if ряд_словарей[i].одиночное.len() == ряд_словарей[i].re_одиночное.len()
            && ряд_словарей[i].одиночное.len() == ряд_словарей[i].замена_одичное.len()
        {
            println!(
                "длина словаря (простого) : {}",
                ряд_словарей[i].одиночное.len()
            );
        }
        //если длина словаря не равна
        else {
            println!("длина слов простых: {}", ряд_словарей[i].одиночное.len());
            println!(
                "длина слов re_простых: {}",
                ряд_словарей[i].re_одиночное.len()
            );
            println!(
                "длина слов замен (простых): {}",
                ряд_словарей[i].замена_одичное.len()
            );
        }
        //перебор одиночных слов
        for j in 0..ряд_словарей[i].одиночное.len() {
            рабочая_страница
                .write(последняя_строка, 0, ряд_словарей[i].одиночное[j].clone())
                .unwrap();
            последняя_строка += 1;
            //println!("{}",&_dictionary[i].простое[j]);
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //output regex
        for j in 0..ряд_словарей[i].re_одиночное.len() {
            рабочая_страница
                .write(_row_point, 1, ряд_словарей[i].re_одиночное[j].to_string())
                .unwrap();
            _row_point += 1;
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //output regex
        for j in 0..ряд_словарей[i].замена_одичное.len() {
            рабочая_страница
                .write(_row_point, 2, ряд_словарей[i].замена_одичное[j].to_string())
                .unwrap();
            _row_point += 1;
        }

        //2-я страница с составными словами
        let mut вкладка = Worksheet::new();
        let mut страница = вкладка.set_name("Сложные слова").unwrap();
        вывод_заголовков_на_странице(&mut страница).unwrap();
        let mut указатель_строки: u32 = u32::try_from(1).unwrap().into();
        //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
        //перебор всех словарей
        if ряд_словарей[i].составное.len() == ряд_словарей[i].замена_составное.len()
        {
            println!(
                "длина словаря (сложного) : {}",
                ряд_словарей[i].составное.len()
            );
        }
        //если длина словаря не равна
        else {
            println!("длина слов сложных: {}", ряд_словарей[i].составное.len());
            println!(
                "длина слов re_сложных: {}",
                ряд_словарей[i].re_составное.len()
            );
            println!(
                "длина слов замен (сложных): {}",
                ряд_словарей[i].замена_составное.len()
            );
        }
        //перебор одиночных слов
        for j in 0..ряд_словарей[i].составное.len() {
            страница
                .write(указатель_строки, 0, ряд_словарей[i].составное[j].clone())
                .unwrap();
            указатель_строки += 1;
            //println!("{}",&_dictionary[i].complex[j]);
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //output regex
        for j in 0..ряд_словарей[i].re_составное.len() {
            страница
                .write(_row_point, 1, ряд_словарей[i].re_составное[j].to_string())
                .unwrap();
            _row_point += 1;
        }
        //обнуление указателя
        let mut указатель_строки: u32 = u32::try_from(1).unwrap().into();
        //output regex
        for j in 0..ряд_словарей[i].замена_составное.len() {
            страница
                .write(
                    указатель_строки,
                    2,
                    ряд_словарей[i].замена_составное[j].to_string(),
                )
                .unwrap();
            указатель_строки += 1;
        }

        //3-я страница с составными словами
        let mut binding2 = Worksheet::new();
        let mut everywhere = binding2.set_name("Вездесущие слова").unwrap();
        вывод_заголовков_на_странице(&mut everywhere).unwrap();
        //everywhere.write(0, 5, "Ток потребления").unwrap();
        //everywhere.write(0, 6, "Цепь земли (по умолчанию)").unwrap();
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
        //перебор всех словарей

        if ряд_словарей[i].вездесушее.len() == ряд_словарей[i].re_вездесушее.len()
            && ряд_словарей[i].вездесушее.len() == ряд_словарей[i].замена_вездесушее.len()
        {
            println!(
                "длина словаря (вездесущего) : {}",
                ряд_словарей[i].вездесушее.len()
            );
        }
        //если длина словаря не равна
        else {
            println!(
                "длина слов вездесущих: {}",
                ряд_словарей[i].вездесушее.len()
            );
            println!(
                "длина слов re_вездесущих: {}",
                ряд_словарей[i].re_вездесушее.len()
            );
            println!(
                "длина слов замен (вездесущих): {}",
                ряд_словарей[i].замена_вездесушее.len()
            );
        }
        //перебор одиночных слов
        for j in 0..ряд_словарей[i].вездесушее.len() {
            everywhere
                .write(_row_point, 0, ряд_словарей[i].вездесушее[j].clone())
                .unwrap();
            _row_point += 1;
            //println!("{}",&_dictionary[i].everywhere[j]);
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //output regex
        for j in 0..ряд_словарей[i].re_вездесушее.len() {
            everywhere
                .write(_row_point, 1, ряд_словарей[i].re_вездесушее[j].to_string())
                .unwrap();
            _row_point += 1;
        }
        //обнуление указателя
        let mut _row_point: u32 = u32::try_from(1).unwrap().into();
        //output regex
        for j in 0..ряд_словарей[i].замена_вездесушее.len() {
            everywhere
                .write(
                    _row_point,
                    2,
                    ряд_словарей[i].замена_вездесушее[j].to_string(),
                )
                .unwrap();
            _row_point += 1;
        }

        //составные в 1 очередь

        //3-я страница с составными словами
        let mut binding3 = Worksheet::new();
        let mut complex_first = binding3.set_name("Составные слова (в 1 очередь)").unwrap();
        вывод_заголовков_на_странице(&mut everywhere).unwrap();
        let mut последняя_строка: u32 = u32::try_from(1).unwrap().into();
        //перебор всех словарей
        if ряд_словарей[i].составное_важное.len() == ряд_словарей[i].re_составное_важное.len()
            && ряд_словарей[i].составное_важное.len()
                == ряд_словарей[i].замена_составное_важное.len()
        {
            println!(
                "длина словаря (сложные слова в 1 очередь)  : {}",
                ряд_словарей[i].составное_важное.len()
            );
        }
        //если длина словаря не равна
        else {
            println!(
                "длина слов сложных (в 1 очередь): {}",
                ряд_словарей[i].составное_важное.len()
            );
            println!(
                "длина слов re_сложных (в 1 очередь): {}",
                ряд_словарей[i].re_составное_важное.len()
            );
            println!(
                "длина слов замен (сложных (в 1 очередь)): {}",
                ряд_словарей[i].замена_составное_важное.len()
            );
        }
        //перебор одиночных слов
        for j in 0..ряд_словарей[i].составное_важное.len() {
            complex_first
                .write(
                    последняя_строка,
                    0,
                    ряд_словарей[i].составное_важное[j].clone(),
                )
                .unwrap();
            последняя_строка += 1;
            //println!("{}",&_dictionary[i].complex_first[j]);
        }
        //обнуление указателя
        let mut последняя_строка: u32 = u32::try_from(1).unwrap().into();
        //output regex
        for j in 0..ряд_словарей[i].re_составное_важное.len() {
            complex_first
                .write(
                    последняя_строка,
                    1,
                    ряд_словарей[i].re_составное_важное[j].to_string(),
                )
                .unwrap();
            последняя_строка += 1;
        }
        //обнуление указателя
        let mut последняя_строка: u32 = u32::try_from(1).unwrap().into();
        //output regex
        for j in 0..ряд_словарей[i].замена_составное_важное.len() {
            complex_first
                .write(
                    последняя_строка,
                    2,
                    ряд_словарей[i].замена_составное_важное[j].to_string(),
                )
                .unwrap();
            последняя_строка += 1;
        }

        //путь сохранения
        let _path: String = format!("./end/dictionary/{}.xlsx", ряд_словарей[i].имя);
        страница.autofit();
        everywhere.autofit();
        рабочая_страница.autofit();
        complex_first.autofit();
        рабочая_книга.push_worksheet(вкладка);
        рабочая_книга.push_worksheet(binding2);
        рабочая_книга.push_worksheet(binding3);
        рабочая_книга.save(_path).unwrap();
    }
    Ok(())
}

//output главного словаря
pub fn вывод_всех_словарей_в_xls(
    _dictionary: &lib::ПолныйСловарь,
    //mode: String,           //Стопка из файла .xlsx взята или самостоятельно высчитана
    //path_name_spd: &String, //имя .spd файла
) -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let пути_общие: lib::Пути_Общие = Default::default();
    let mut словари = Workbook::new();
    // Add a worksheet to the workbook.
    let книга = словари.add_worksheet().set_name("Простые слова").unwrap();
    книга.write(0, 0, "Изначальные слова").unwrap();
    книга.write(0, 1, "Regex").unwrap();
    книга.write(0, 2, "Замена").unwrap();
    книга.write(0, 3, "Количество случаев").unwrap();
    книга.write(0, 4, "Строка").unwrap();
    //worksheet.write(0, 5, "Ток потребления").unwrap();
    //worksheet.write(0, 6, "Цепь земли (по умолчанию)").unwrap();
    let mut _row_point: u32 = u32::try_from(1).unwrap().into();
    //общий счётчик замен слов
    let mut _count_change: usize = 0;
    //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
    //перебор всех словарей
    println!();
    println!("Общий словарь");

    //перебор одиночных слов
    for j in 0.._dictionary.простое.len() {
        //добавление количества замен
        _count_change += _dictionary.простое_счётчик_замен[j];
        книга
            .write((j + 1) as u32, 0, &_dictionary.простое[j])
            .unwrap();
        книга
            .write((j + 1) as u32, 1, _dictionary.re_простое[j].to_string())
            .unwrap();
        книга
            .write(
                (j + 1) as u32,
                2,
                _dictionary.замена_простому_нижнее[j].to_string(),
            )
            .unwrap();
        книга
            .write(
                (j + 1) as u32,
                3,
                _dictionary.простое_счётчик_замен[j].to_string(),
            )
            .unwrap();
        _row_point += 1;
        //println!("{}",&_dictionary.простое[j]);
    }
    //если все слова равны
    if _dictionary.простое.len() == _dictionary.re_простое.len()
        && _dictionary.простое.len() == _dictionary.замена_простому_нижнее.len()
    {
        println!(
            "длина словаря (простого) : {}, замен: {}",
            _dictionary.простое.len(),
            &_count_change
        );
    }
    //если длина словаря не равна
    else {
        println!("длина слов простых: {}", _dictionary.простое.len());
        println!("длина слов re_простых: {}", _dictionary.re_простое.len());
        println!(
            "длина слов замен (простых): {}",
            _dictionary.замена_простому_нижнее.len()
        );
    }
    книга
        .write((_row_point + 1) as u32, 0, "Итого замен: ")
        .unwrap();
    книга
        .write((_row_point + 1) as u32, 3, _count_change.to_string())
        .unwrap();
    книга.autofilter(0, 0, _row_point + 1, 4).unwrap();
    //2-я страница с составными словами
    let mut стр_сложных_слов = Worksheet::new();
    let стр_2 = стр_сложных_слов.set_name("Сложные слова").unwrap();
    стр_2.write(0, 0, "Изначальные слова").unwrap();
    стр_2.write(0, 1, "Regex").unwrap();
    стр_2.write(0, 2, "Замена").unwrap();
    стр_2.write(0, 3, "Количество случаев").unwrap();
    стр_2.write(0, 4, "Строка").unwrap();
    //complex.write(0, 5, "Ток потребления").unwrap();
    //complex.write(0, 6, "Цепь земли (по умолчанию)").unwrap();
    let mut _row_point: u32 = u32::try_from(1).unwrap().into();
    //общий счётчик замен слов
    let mut _count_change: usize = 0;
    //let column_point: u16 = u16::try_from(i + 1).unwrap().into();

    //перебор всех словарей

    //перебор одиночных слов
    for j in 0.._dictionary.составное.len() {
        _count_change += _dictionary.составное_счётчик_замен[j];
        стр_2
            .write(_row_point, 0, _dictionary.составное[j].clone())
            .unwrap();
        стр_2
            .write(_row_point, 1, _dictionary.re_составное[j].to_string())
            .unwrap();
        стр_2
            .write(
                _row_point,
                2,
                _dictionary.замена_составное_нижнее[j].to_string(),
            )
            .unwrap();
        стр_2
            .write(
                _row_point,
                3,
                _dictionary.составное_счётчик_замен[j].to_string(),
            )
            .unwrap();
        //println!("{}",&_dictionary.complex[j]);
        _row_point += 1;
        //println!("{}",&_dictionary.простое[j]);
    }
    //если количество слов равно
    if _dictionary.составное.len() == _dictionary.re_составное.len()
        && _dictionary.составное.len() == _dictionary.замена_составное_нижнее.len()
    {
        println!(
            "длина словаря (сложного) : {}, количество замен: {}",
            _dictionary.составное.len(),
            &_count_change
        );
    }
    //если длина словаря не равна
    else {
        println!("длина слов сложных: {}", _dictionary.составное.len());
        println!("длина слов re_сложных: {}", _dictionary.re_составное.len());
        println!(
            "длина слов замен (сложных): {}",
            _dictionary.замена_составное_нижнее.len()
        );
    }
    стр_2
        .write((_row_point + 1) as u32, 0, "Итого замен: ")
        .unwrap();
    стр_2
        .write((_row_point + 1) as u32, 3, _count_change.to_string())
        .unwrap();
    стр_2.autofilter(0, 0, _row_point + 1, 4).unwrap();
    //3-я страница с составными словами
    let mut binding2 = Worksheet::new();
    let everywhere = binding2.set_name("Вездесущие слова").unwrap();
    everywhere.write(0, 0, "Изначальные слова").unwrap();
    everywhere.write(0, 1, "Regex").unwrap();
    everywhere.write(0, 2, "Замена").unwrap();
    everywhere.write(0, 3, "Количество случаев").unwrap();
    everywhere.write(0, 4, "Строка").unwrap();
    //everywhere.write(0, 5, "Ток потребления").unwrap();
    //everywhere.write(0, 6, "Цепь земли (по умолчанию)").unwrap();
    let mut _row_point: u32 = u32::try_from(1).unwrap().into();
    //общий счётчик замен слов
    let mut _count_change: usize = 0;
    //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
    //перебор всех словарей

    //перебор одиночных слов
    for j in 0.._dictionary.вездесущее.len() {
        _count_change += _dictionary.счётчик_вездесущее[j];
        everywhere
            .write(_row_point, 0, _dictionary.вездесущее[j].clone())
            .unwrap();
        everywhere
            .write(_row_point, 1, _dictionary.re_вездесущее[j].to_string())
            .unwrap();
        everywhere
            .write(
                _row_point,
                2,
                _dictionary.замена_вездесущее_нижнее[j].to_string(),
            )
            .unwrap();
        everywhere
            .write(_row_point, 3, _dictionary.счётчик_вездесущее[j].to_string())
            .unwrap();
        _row_point += 1;
        //println!("{}",&_dictionary.everywhere[j]);
    }
    //если количество слов равно числу замен
    if _dictionary.вездесущее.len() == _dictionary.re_вездесущее.len()
        && _dictionary.вездесущее.len() == _dictionary.замена_вездесущее_нижнее.len()
    {
        println!(
            "длина словаря (вездесущего) : {}, количество замен: {}",
            _dictionary.вездесущее.len(),
            &_count_change
        );
    }
    //если длина словаря не равна
    else {
        println!("длина слов вездесущих: {}", _dictionary.вездесущее.len());
        println!(
            "длина слов re_вездесущих: {}",
            _dictionary.re_вездесущее.len()
        );
        println!(
            "длина слов замен (вездесущих): {}",
            _dictionary.замена_вездесущее_нижнее.len()
        );
    }
    everywhere
        .write((_row_point + 1) as u32, 0, "Итого замен: ")
        .unwrap();
    everywhere
        .write((_row_point + 1) as u32, 3, _count_change.to_string())
        .unwrap();
    everywhere.autofilter(0, 0, _row_point + 1, 4).unwrap();
    //составные в 1 очередь
    //3-я страница с составными словами
    let mut binding3 = Worksheet::new();
    let complex_first = binding3.set_name("Составные слова (в 1 очередь)").unwrap();
    complex_first.write(0, 0, "Изначальные слова").unwrap();
    complex_first.write(0, 1, "Regex").unwrap();
    complex_first.write(0, 2, "Замена").unwrap();
    complex_first.write(0, 3, "Количество случаев").unwrap();
    complex_first.write(0, 4, "Строка").unwrap();
    //complex_first.write(0, 5, "Ток потребления").unwrap();
    //complex_first.write(0, 6, "Цепь земли (по умолчанию)").unwrap();
    let mut _row_point: u32 = u32::try_from(1).unwrap().into();
    //общий счётчик замен слов
    let mut _count_change: usize = 0;
    //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
    //перебор всех словарей

    //перебор одиночных слов
    for j in 0.._dictionary.составное_важное.len() {
        _count_change += _dictionary.счётчик_составное_важное[j];
        complex_first
            .write(_row_point, 0, _dictionary.составное_важное[j].clone())
            .unwrap();
        complex_first
            .write(
                _row_point,
                1,
                _dictionary.re_составное_важное[j].to_string(),
            )
            .unwrap();
        complex_first
            .write(
                _row_point,
                2,
                _dictionary.замена_составное_важное_нижнее[j].to_string(),
            )
            .unwrap();
        complex_first
            .write(
                _row_point,
                2,
                _dictionary.счётчик_составное_важное[j].to_string(),
            )
            .unwrap();
        _row_point += 1;
        //println!("{}",&_dictionary.complex_first[j]);
    }
    //если количество слов равно числу замен
    if _dictionary.составное_важное.len() == _dictionary.re_составное_важное.len()
        && _dictionary.составное_важное.len() == _dictionary.замена_составное_важное_нижнее.len()
    {
        println!(
            "длина словаря (сложного (в 1 очередь) )  : {}, количество замен: {}",
            _dictionary.составное_важное.len(),
            &_count_change
        );
        println!();
    }
    //если длина словаря не равна
    else {
        println!(
            "длина слов сложных (в 1 очередь): {}",
            _dictionary.составное_важное.len()
        );
        println!(
            "длина слов re_сложных (в 1 очередь): {}",
            _dictionary.re_составное_важное.len()
        );
        println!(
            "длина слов замен (сложных (в 1 очередь)): {}",
            _dictionary.замена_составное_важное_нижнее.len()
        );
        println!();
    }
    complex_first
        .write((_row_point + 1) as u32, 0, "Итого замен: ")
        .unwrap();
    complex_first
        .write((_row_point + 1) as u32, 2, _count_change.to_string())
        .unwrap();
    complex_first.autofilter(0, 0, _row_point + 1, 3).unwrap();
    //путь сохранения
    let путь_сохранения: String = format!("{}Все словари вместе.xlsx", пути_общие.вывод_словари);
    стр_2.autofit();
    everywhere.autofit();
    книга.autofit();
    complex_first.autofit();
    словари.push_worksheet(стр_сложных_слов);
    словари.push_worksheet(binding2);
    словари.push_worksheet(binding3);
    словари.save(путь_сохранения).unwrap();

    Ok(())
}
//output заголовкой на странице
pub fn вывод_заголовков_на_странице(
    рабочая_страница: &mut Worksheet,
) -> Result<(), XlsxError> {
    рабочая_страница.write(0, 0, "Изначальные слова").unwrap();
    рабочая_страница.write(0, 1, "Regex").unwrap();
    рабочая_страница.write(0, 2, "Замена").unwrap();
    рабочая_страница.write(0, 3, "Количество случаев").unwrap();
    рабочая_страница.write(0, 4, "Строка").unwrap();
    Ok(())
}

/*
use xml::reader::{EventReader, XmlEvent};
use xml::writer::{EmitterConfig, XmlEvent as WEvent};
use std::io::Cursor;

pub fn edit_xml_with_quick_xm() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
        <root>
            <item>Hello</item>
            <item>World</item>
        </root>
    "#;

    let parser = EventReader::from_str(xml);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(Cursor::new(Vec::new()));

    let mut inside_item = false;
    let mut item_index = 0;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                if name.local_name == "item" {
                    item_index += 1;
                    inside_item = true;
                }
                writer.write(WEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }).unwrap();
            }
            Ok(XmlEvent::Characters(s)) if inside_item && item_index == 2 => {
                // заменяем содержимое второго <item>
                writer.write(WEvent::Characters("Rust")).unwrap();
            }
            Ok(XmlEvent::Characters(s)) => {
                writer.write(WEvent::Characters(&s)).unwrap();
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "item" {
                    inside_item = false;
                }
                writer.write(WEvent::EndElement { name }).unwrap();
            }
            Ok(XmlEvent::StartDocument { .. }) |
            Ok(XmlEvent::EndDocument) => { /* можно игнорировать */ }
            Ok(XmlEvent::Whitespace(s)) => {
                writer.write(WEvent::Characters(&s)).unwrap();
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    let result = writer.into_inner().into_inner();
    println!("{}", String::from_utf8(result).unwrap());
    Ok(())
}*/

pub fn вывод_содержимого_папок_по_умолчанию(
    содержимое_папок: &lib::Содержимое_папок,
    источник: &str,
    mut сообщения:&mut Vec<String>,
) -> Result<(), Error> {
    use crate::output::functions::проверка_наличия_папок_в_случае_их_отсутствия_создать_папки;
    //начало
    let пути_общие: lib::Пути_Общие = Default::default();
    let путь_вывода: String = format!("{}{}", пути_общие.вывод, источник);
    проверка_наличия_папок_в_случае_их_отсутствия_создать_папки(&путь_вывода);
    //ошибки сначала
    let путь: String = format!("{}/ошибки.txt", путь_вывода);
    вывод_содержимого_в_txt(&содержимое_папок.ошибки, &путь,&mut  сообщения).unwrap();
    //сами файлы
    let путь: String = format!("{}/содержимое.txt", путь_вывода);
    вывод_содержимого_в_txt(&содержимое_папок.файлы, &путь,&mut  сообщения).unwrap();
    //сами файлы
    let путь: String = format!("{}/не_вложено.txt", путь_вывода);
    вывод_содержимого_в_txt(&содержимое_папок.не_вложено, &путь,&mut  сообщения).unwrap();
    Ok(())
}

pub fn вывод_содержимого_в_txt(
    ряд: &Vec<String>,
    путь: &String,
    mut сообщения:&mut Vec<String>,
) -> Result<(), Error> {
    //сравнение образов
    match запись_если_есть_разница(
        &путь,
        &ряд,
        &mut  сообщения,
    ) {
        Ok(true) => {
            //println!("Внешне: Перезапись")
        }
        Ok(false) => {
            //println!("Внешне: Отказ от перезаписи");
            return  Ok(())
        }
        Err(e) => panic!("{}", e),
    }
    let mut вывод = match File::create(путь) {
        Ok(итог) => итог,
        Err(ошибка) => panic!("{} , путь: {}", ошибка, путь),
    };
    //let mut вывод = File::create(путь).unwrap();
    for строка in ряд.iter() {
        writeln!(вывод, "{}", строка).unwrap();
    }
    Ok(())
}
/*
pub fn запись_если_hash_различается(
    путь_сохранения: &str,
    новое_содержание: &str,
    новый_ряд_строк:&Vec<String>,
) -> io::Result<bool> {
    // Вычисляем хеш нового содержимого
    let mut hasher = Sha512::new();
    hasher.update(новое_содержание.as_bytes());
    let новый_образ = hasher.finalize();

    // Пытаемся прочитать и вычислить хеш существующего файла
    let существующий_файл_ряд_строк: Vec<String> = {
            //прочитать_содержимое_построчно(&пути_до_книг[i])
            read_utf8(&путь_сохранения.to_string()) //чтение файла в UTF-8
        
    };
    if  *новый_ряд_строк==существующий_файл_ряд_строк{ 
        println!("Сравнение: Строки совпадают");
    } 
    
    let существующий_образ = match fs::read(путь_сохранения) {
        Ok(содержимое) => {
            let mut hasher = Sha512::new();
            hasher.update(&содержимое);
            hasher.finalize()
        }
        Err(ошибка) if ошибка.kind() == io::ErrorKind::NotFound => {
            // Файл не существует - записываем новый
            fs::write(путь_сохранения, новое_содержание).unwrap();
            return Ok(true);
        }
        Err(ошибка) => return Err(ошибка),
    };

    // Сравниваем хеши
    if существующий_образ[..] == новый_образ[..] {
        println!("внутри: отказ от перезаписи");
        Ok(false)
    } else {
        println!("существ:{:?}",
                 существующий_образ);
        println!("новый_образ:{:?}",
                 новый_образ);
        println!("внутри: перезапись");
        fs::write(путь_сохранения, новое_содержание).unwrap();
        Ok(true)
    }
}
*/

fn запись_если_есть_разница(
    путь: &String,
    новый_ряд_строк:&Vec<String>,
    mut сообщения:&mut Vec<String>,
) -> io::Result<bool> {
    use crate::utils::functions::{вывод_сообщения_на_экран_и_вложение_в_ряд};
    // Читаем существующий файл как байты
    match File::open(&путь) {
        Ok(содержимое) => (),
        Err(ошибка) if ошибка.kind() == io::ErrorKind::NotFound => {
            // Файл не существует - записываем новый
            //fs::write(&путь, новое_содержимое).unwrap();
            вывод_сообщения_на_экран_и_вложение_в_ряд(format!("Запись:  книга {} не существует файл. Запись",путь),
            &mut  сообщения);
         
            return Ok(true);
        }
        Err(ошибка) => return Err(ошибка),
    };
    // Пытаемся прочитать и вычислить хеш существующего файла
  
    let существующий_файл_ряд_строк: Vec<String> = 
        //прочитать_содержимое_построчно(&пути_до_книг[i])
     read_utf8(&путь) //чтение файла в UTF-8
        ;
    //сравнение строк
    if  *новый_ряд_строк==существующий_файл_ряд_строк{
        вывод_сообщения_на_экран_и_вложение_в_ряд(format!("Запись: книга {}. Полное совпадение. Отказ от перезаписи",путь),
                                                  &mut  сообщения);
        Ok(false)
    } else {
        вывод_сообщения_на_экран_и_вложение_в_ряд(format!("Запись:  книга {}. Не соответствие содержимого. Перезапись",путь),
                                                  &mut  сообщения);
        //fs::write(&путь, новое_содержимое).unwrap();
        Ok(true)
    }

}