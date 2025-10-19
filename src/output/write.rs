#![allow(non_ascii_idents)]

use std::any::Any;
//use std::fs::read_to_string;
use crate::lib::{self, Куча_Словарь, Пути_Вывода, Содержимое_папок, Сообщения, Сообщения_для_книги};
use crate::utils::regex::{
    fb2_rtf_mhtml, fb3_epub, md_fs_yml, изображение_расширение_с_точкой
};
use crate::utils::zip::{
    pack_zip_from_memory, zip_архив_в_память, Архив_в_озу
};
use encoding_rs::{
    WINDOWS_1251,
    //    DecoderResult
};
use foldhash::{HashMap, HashSet, HashSetExt, fast::RandomState};
use rust_xlsxwriter::*;
use std::fs::{self, File};
use std::io::{self, BufReader, Cursor, Error, Write};
use std::path::Path;
use std::sync::Mutex;
//use xml::Encoding::Default;
use crate::output::write;
use crate::utils::functions_txt::сравнение_двух_рядов_построчно;
use crate::utils::read::read_utf8;
use crate::utils::stringzilla::sz_найти;
use calamine::*;
use rayon::prelude::*;
use text_changer::Пути_Общие;

pub fn сохранить_книги(
    стопки_книг: &Vec<lib::Книги>,
    mut сообщения: &mut lib::Сообщения,
) -> Result<(), Error> {
    println!(); // Переход на новую строку после завершения
    сообщения.запись_и_чтение = vec![Default::default(); стопки_книг.len()];
    let стопки_сообщений: Mutex<Vec<crate::lib::Сообщения_для_книги>> =
        Mutex::new(vec![Default::default(); стопки_книг.len()]);
    //let сообщения_запись_и_чтение :Mutex< Vec<Сообщения_для_книги>>=Mutex::new(сообщения.запись_и_чтение.clone());
    стопки_книг.par_iter().enumerate().for_each(|(i, книга)| {
        //for i in 0..стопки_книг.len() {
        let сообщения_запись_и_чтение: Mutex<crate::lib::Сообщения_для_книги> =
            Mutex::new(Default::default());
        //имя книге присваиваем для вывода
        сообщения_запись_и_чтение.lock().unwrap().имя_книги = format!(
            "{}.{}",
            стопки_книг[i].название_книги, стопки_книг[i].расширение
        );
        //путь до вывода
        let пути_общие: lib::Пути_Общие = Default::default();
        //println!("путь до книги: {}",стопки_книг[i].путь);
        let путь_сохранения: String = стопки_книг[i]
            .путь
            .replace(&пути_общие.книги, &пути_общие.вывод_книги);
        //вывод книги
        //если это не архивная книга
        if fb2_rtf_mhtml(&стопки_книг[i].путь) || md_fs_yml(&стопки_книг[i].путь)
        {
            //перебор книг
            for гл_указатель in 0..стопки_книг[i].вложения.len() {
                //сравнение образов
                match запись_если_есть_разница(
                    &путь_сохранения,
                    &стопки_книг[i].вложения[гл_указатель].содержимое,
                    &mut сообщения_запись_и_чтение.lock().unwrap().сообщения,
                    true, //вывод на экран
                ) {
                    //файл существует - но надо перезаписать
                    Ok(true) => {
                        //println!("Внешне: Перезапись")
                    }
                    //файл существует - перезапись не нужна
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
            let пустая_стопка_hashmap: foldhash::HashMap<String, Vec<u8>> =
                foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
            let mut вторичная_fs_в_озу: Архив_в_озу = пустая_стопка_hashmap;
            //перебор содержимого архива
            for k in 0..стопки_книг[i].вложения.len() {
                //1 -имя, 2 - содержимое в hashmap
                let mut содержимое_байты: Vec<u8> = Vec::new();
                //перебор содержимого книги из String в UTF8
                //если это рисунок
                if изображение_расширение_с_точкой(
                    &стопки_книг[i].вложения[k].имя,
                ) || sz_найти(&стопки_книг[i].вложения[k].имя, ".ttf")
                {
                    содержимое_байты = стопки_книг[i].вложения[k].содержимое_в_байтах.clone();
                }
                //если это не картинки
                else {
                    //println!("файл: {} количество строк:{}",стопки_книг[i].вложения[k].имя,стопки_книг[i].вложения[k].содержимое.len());
                    for k2 in 0..стопки_книг[i].вложения[k].содержимое.len()
                    {
                        // Добавляем строку с переводом строки
                        содержимое_байты
                            .extend(стопки_книг[i].вложения[k].содержимое[k2].as_bytes());
                        // Добавляем перевод строки после каждой строки, кроме последней
                        if k2 < стопки_книг[i].вложения[k].содержимое.len() - 1
                        {
                            содержимое_байты.extend(b"\n"); // Unix-style

                            /* содержимое_байты.extend(
                                стопки_книг[i].вложения[k].содержимое[k2]
                                    .as_bytes()
                                    .to_vec(),
                            );*/
                        }
                    }
                }
                //вложение в словарь
                вторичная_fs_в_озу.insert(стопки_книг[i].вложения[k].имя.clone(), содержимое_байты);
            }
            //Запаковывает виртуальную файловую систему в Vec<u8>
            let архив_в_виде_байт = match pack_zip_from_memory(&вторичная_fs_в_озу) {
                Ok(путь) => путь,
                Err(причина) => panic!("{:?}", причина),
            };
            //output в файл
            запись_архива_на_накопитель(
                &архив_в_виде_байт,
                &стопки_книг[i],
                &mut сообщения_запись_и_чтение.lock().unwrap().сообщения,
                &вторичная_fs_в_озу,
            )
            .unwrap();
        }
        стопки_сообщений.lock().unwrap()[i] = сообщения_запись_и_чтение.into_inner().unwrap();
    });
    сообщения.запись_и_чтение = стопки_сообщений.into_inner().unwrap();
    Ok(())
}

pub fn запись_архива_на_накопитель(
    озу_книга_байты: &Vec<u8>,
    книга: &lib::Книги,
    mut сообщения: &mut Vec<String>,
    озу_книга_куча: &Архив_в_озу,
) -> Result<(), Error> {
    use std::default::Default;
    //путь до вывода
    let пути_общие: lib::Пути_Общие = Default::default();
    //let path = format!("./end/{}.{}",i,book_struct[i].format);
    let путь = format!(
        "{}{}.{}",
        пути_общие.вывод_книги, книга.название_книги, книга.расширение
    );
    //запись
    //сравнение образов
    match запись_если_есть_разница_архив(
        &путь,
        &озу_книга_куча,
        &mut сообщения,
        true, //вывод на экран
    ) {
        //нужна перезапись
        Ok(true) => {
            let mut путь_вывода = File::create(&путь).unwrap();
            //путь_вывода.write_all(озу_книга_байты).unwrap();
            let mut zip = zip::ZipWriter::new(путь_вывода);
            let настройки = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Stored);

            // Записываем все файлы обратно в архив
            for (имя_файла, содержимое) in озу_книга_куча {
                zip.start_file(имя_файла, настройки).unwrap();
                zip.write_all(содержимое).unwrap();
            }
            zip.finish().unwrap();
        }
        //перезапись не нужна
        Ok(false) => {}
        Err(ошибка) => panic!("{}", ошибка),
    }
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
//output главного словаря
pub fn вывод_всех_словарей_в_xls(
    словарь: &lib::Полный_Словарь,
    куча_словарь: &Куча_Словарь,
    //mode: String,           //Стопка из файла .xlsx взята или самостоятельно высчитана
    //path_name_spd: &String, //имя .spd файла
) -> Result<(), rust_xlsxwriter::XlsxError> {
    use std::fs;
    use std::path::Path;
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
    let mut счётчик_шага: usize = 0;
    //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
    //перебор всех словарей
    println!();
    println!("Общий словарь");
    //перебор одиночных слов
    for j in 0..словарь.простое.len() {
        //добавление количества замен
        счётчик_шага += словарь.счётчик_простое[j];
        книга
            .write((j + 1) as u32, 0, &словарь.простое[j].искомое_слово)
            .unwrap();
        книга
            .write((j + 1) as u32, 1, словарь.простое[j].re_образец.to_string())
            .unwrap();
        книга
            .write((j + 1) as u32, 2, словарь.простое[j].замена.to_string())
            .unwrap();
        книга
            .write((j + 1) as u32, 3, словарь.счётчик_простое[j].to_string())
            .unwrap();
        _row_point += 1;
        //println!("{}",&_dictionary.простое[j]);
    }

    println!(
        "Простое, количетсво. Словарь: {}| Куча: {} |, замен: {}",
        словарь.простое.len(),
        куча_словарь.простое.len(),
        &счётчик_шага
    );

    //если длина словаря не равна
    книга
        .write((_row_point + 1) as u32, 0, "Итого замен: ")
        .unwrap();
    книга
        .write((_row_point + 1) as u32, 3, счётчик_шага.to_string())
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
    for j in 0..словарь.составное.len() {
        _count_change += словарь.счётчик_составное[j];
        стр_2
            .write(_row_point, 0, словарь.составное[j].искомое_слово.clone())
            .unwrap();
        стр_2
            .write(_row_point, 1, словарь.составное[j].re_образец.to_string())
            .unwrap();
        стр_2
            .write(_row_point, 2, словарь.составное[j].замена.to_string())
            .unwrap();
        стр_2
            .write(_row_point, 3, словарь.счётчик_составное[j].to_string())
            .unwrap();
        //println!("{}",&_dictionary.complex[j]);
        _row_point += 1;
        //println!("{}",&_dictionary.простое[j]);
    }
    //если количество слов равно
    println!(
        "Сложное, количетсво. Словарь: {}| Куча: {} | , количество замен: {}",
        словарь.составное.len(),
        куча_словарь.составное.len(),
        &_count_change
    );
    //если длина словаря не равна
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
    for j in 0..словарь.вездесущее.len() {
        _count_change += словарь.счётчик_вездесущее[j];
        everywhere
            .write(_row_point, 0, словарь.вездесущее[j].искомое_слово.clone())
            .unwrap();
        everywhere
            .write(_row_point, 1, словарь.вездесущее[j].re_образец.to_string())
            .unwrap();
        everywhere
            .write(_row_point, 2, словарь.вездесущее[j].замена.to_string())
            .unwrap();
        everywhere
            .write(_row_point, 3, словарь.счётчик_вездесущее[j].to_string())
            .unwrap();
        _row_point += 1;
        //println!("{}",&_dictionary.everywhere[j]);
    }
    //если количество слов равно числу замен

    println!(
        "Вездесущее, количетсво. Словарь: {}| Куча: {} |, количество замен: {}",
        словарь.вездесущее.len(),
        куча_словарь.вездесущее.len(),
        &_count_change
    );

    //если длина словаря не равна
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
    let составные_важные = binding3.set_name("Составные слова (в 1 очередь)").unwrap();
    составные_важные.write(0, 0, "Изначальные слова").unwrap();
    составные_важные.write(0, 1, "Regex").unwrap();
    составные_важные.write(0, 2, "Замена").unwrap();
    составные_важные.write(0, 3, "Количество случаев").unwrap();
    составные_важные.write(0, 4, "Строка").unwrap();
    //complex_first.write(0, 5, "Ток потребления").unwrap();
    //complex_first.write(0, 6, "Цепь земли (по умолчанию)").unwrap();
    let mut _row_point: u32 = u32::try_from(1).unwrap().into();
    //общий счётчик замен слов
    let mut _count_change: usize = 0;
    //let column_point: u16 = u16::try_from(i + 1).unwrap().into();
    //перебор всех словарей

    //перебор одиночных слов
    for j in 0..словарь.составное_важное.len() {
        _count_change += словарь.счётчик_составное_важное[j];
        составные_важные
            .write(
                _row_point,
                0,
                словарь.составное_важное[j].искомое_слово.clone(),
            )
            .unwrap();
        составные_важные
            .write(
                _row_point,
                1,
                словарь.составное_важное[j].re_образец.to_string(),
            )
            .unwrap();
        составные_важные
            .write(
                _row_point,
                2,
                словарь.составное_важное[j].замена.to_string(),
            )
            .unwrap();
        составные_важные
            .write(
                _row_point,
                2,
                словарь.счётчик_составное_важное[j].to_string(),
            )
            .unwrap();
        _row_point += 1;
        //println!("{}",&_dictionary.complex_first[j]);
    }
    //если количество слов равно числу замен
    println!(
        "Сложное важное, количетсво. Словарь: {}| Куча: {} |, количество замен: {}",
        словарь.составное_важное.len(),
        куча_словарь.составное_важное.len(),
        &_count_change
    );
    println!();

    //если длина словаря не равна
    составные_важные
        .write((_row_point + 1) as u32, 0, "Итого замен: ")
        .unwrap();
    составные_важные
        .write((_row_point + 1) as u32, 2, _count_change.to_string())
        .unwrap();
    составные_важные
        .autofilter(0, 0, _row_point + 1, 3)
        .unwrap();
    //путь сохранения
    let путь_сохранения: String = format!("{}Все словари вместе.xlsx", пути_общие.вывод_словари);
    стр_2.autofit();
    everywhere.autofit();
    книга.autofit();
    составные_важные.autofit();
    словари.push_worksheet(стр_сложных_слов);
    словари.push_worksheet(binding2);
    словари.push_worksheet(binding3);
    xlsx_сохранить_с_проверкой(&mut словари, &путь_сохранения);

    Ok(())
}
//xlsx перед сохранением на накопителе проверяет есть ли уже такой, если есть то совпадает ли содержимое
pub fn xlsx_сохранить_с_проверкой(
    содержимое: &mut Workbook,
    путь_сохранения: &String,
) {
    //let mut временное_содержимое=содержимое.clone();
    // 2️⃣ Сохраняем в буфер в памяти
    let озу: Vec<u8> = содержимое.save_to_buffer().unwrap();
    // 3️⃣ Читаем существующий файл с диска
    let путь = Path::new(&путь_сохранения);
    let условие = if путь.exists() {
        //let содержимое_с_накопителя = fs::read(путь).unwrap();
        // содержимое_с_накопителя == озу
        let содержимое_буффера = прочитать_xlsx_из_буфера(&озу).unwrap();
        let данные_с_диска = прочитать_xlsx_с_диска(&путь_сохранения).unwrap();
        if сравнить_данные(&содержимое_буффера, &данные_с_диска)
        {
            println!(
                "XLSX файл: {} полностью совпадает с существующим XLSX файлом. Отказ от перезаписи.",
                путь_сохранения
            );
            true
        } else {
            println!(
                "XLSX файл: {} не совпадает с существующим. Перезапись.",
                путь_сохранения
            );
            match содержимое.save(путь_сохранения) {
                Ok(_) => false,
                Err(ошибка) => panic!(
                    "Не удаётся записать файл: {}\r\nпричина:{ошибка}",
                    путь_сохранения
                ),
            }
        }
        // false
    } else {
        println!(
            "XLSX файл: {} не уществует. Создание и запись.",
            путь_сохранения
        );
        match содержимое.save(путь_сохранения) {
            Ok(_) => false,
            Err(ошибка) => panic!(
                "Не удаётся записать файл: {}\r\nпричина:{ошибка}",
                путь_сохранения
            ),
        }
    };
}

pub fn прочитать_xlsx_с_диска(
    путь: &str,
) -> Result<HashMap<String, Vec<Vec<String>>>, Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(путь).unwrap();
    let mut данные: HashMap<String, Vec<Vec<String>>> =
        HashMap::with_hasher(foldhash::fast::RandomState::default());

    for sheet_name in workbook.sheet_names().clone() {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let mut строки_листа = Vec::new();

            for row in range.rows() {
                let ячейки: Vec<String> = row
                    .iter()
                    .map(|cell| match cell {
                        Data::String(s) => s.clone(),
                        Data::Float(f) => f.to_string(),
                        Data::Int(i) => i.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::DateTime(dt) => dt.to_string(),
                        Data::DateTimeIso(s) => s.clone(), // Добавлено
                        Data::DurationIso(s) => s.clone(), // Добавлено
                        Data::Empty => String::new(),
                        Data::Error(e) => format!("Ошибка: {:?}", e),
                    })
                    .collect();
                строки_листа.push(ячейки);
            }
            данные.insert(sheet_name, строки_листа);
        }
    }
    Ok(данные)
}
// 3. Сравнение исходных и прочитанных данных
pub fn сравнить_данные(
    исходные: &HashMap<String, Vec<Vec<String>>>,
    прочитанные: &HashMap<String, Vec<Vec<String>>>,
) -> bool {
    if исходные.len() != прочитанные.len() {
        //   println!("❌ Разное количество листов: {} vs {}", исходные.len(), прочитанные.len());
        return false;
    }
    for (имя_листа, исходные_строки) in исходные {
        match прочитанные.get(имя_листа) {
            Some(прочитанные_строки) => {
                if исходные_строки != прочитанные_строки {
                    println!("❌ Лист '{}': данные различаются", имя_листа);
                    return false;
                }
            }
            None => {
                println!("❌ Лист '{}' отсутствует в прочитанных данных", имя_листа);
                return false;
            }
        }
    }
    //println!("✓ Данные идентичны!");
    true
}
// 2. Чтение из buffer через calamine
pub fn прочитать_xlsx_из_буфера(
    buffer: &[u8],
) -> Result<HashMap<String, Vec<Vec<String>>>, Box<dyn std::error::Error>> {
    let cursor = Cursor::new(buffer);
    let mut workbook: Xlsx<_> = Xlsx::new(cursor).unwrap();
    let mut данные = HashMap::default();

    //println!("Чтение XLSX из буфера...");

    for sheet_name in workbook.sheet_names().clone() {
        // println!("  Обработка листа: {}", sheet_name);

        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let mut строки_листа = Vec::new();
            let mut количество_строк = 0;

            for row in range.rows() {
                let ячейки: Vec<String> = row
                    .iter()
                    .map(|cell| match cell {
                        Data::String(s) => s.clone(),
                        Data::Float(f) => f.to_string(),
                        Data::Int(i) => i.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::DateTime(dt) => dt.to_string(),
                        Data::DateTimeIso(s) => s.clone(), // Добавлено
                        Data::DurationIso(s) => s.clone(), // Добавлено
                        Data::Empty => String::new(),
                        Data::Error(e) => format!("Ошибка: {:?}", e),
                    })
                    .collect();

                строки_листа.push(ячейки);
            }

            данные.insert(sheet_name, строки_листа);
            //   println!("    Прочитано строк: {}", количество_строк);
        } else {
            //     println!("    ❌ Ошибка чтения листа: {}", sheet_name);
        }
    }
    //  println!("✓ XLSX прочитан из буфера, листов: {}", данные.len());
    Ok(данные)
}
//output заголовкой на странице
pub fn вывод_заголовков_на_странице(
    рабочая_страница: &mut Worksheet,
) -> Result<(), rust_xlsxwriter::XlsxError> {
    рабочая_страница.write(0, 0, "Изначальные слова").unwrap();
    рабочая_страница.write(0, 1, "Regex").unwrap();
    рабочая_страница.write(0, 2, "Замена").unwrap();
    рабочая_страница.write(0, 3, "Количество случаев").unwrap();
    рабочая_страница.write(0, 4, "Строка").unwrap();
    Ok(())
}

pub fn вывод_содержимого_папок_по_умолчанию(
    содержимое_папок: &lib::Содержимое_папок,
    источник: &str,
    mut сообщения: &mut Vec<String>,
) -> Result<(), Error> {
    use crate::output::functions::проверка_наличия_папок_в_случае_их_отсутствия_создать_папки;
    //начало
    let пути_общие: lib::Пути_Общие = Default::default();
    let путь_вывода: String = format!("{}{}", пути_общие.вывод, источник);
    проверка_наличия_папок_в_случае_их_отсутствия_создать_папки(&путь_вывода);
    //ошибки сначала
    let путь: String = format!("{}/ошибки.txt", путь_вывода);
    вывод_содержимого_в_txt(
        &содержимое_папок.ошибки,
        &путь,
        &mut сообщения,
        false,
    )
    .unwrap();
    //сами файлы
    let путь: String = format!("{}/содержимое.txt", путь_вывода);
    вывод_содержимого_в_txt(&содержимое_папок.файлы, &путь, &mut сообщения, false).unwrap();
    //сами файлы
    let путь: String = format!("{}/не_вложено.txt", путь_вывода);
    вывод_содержимого_в_txt(
        &содержимое_папок.не_вложено,
        &путь,
        &mut сообщения,
        false,
    )
    .unwrap();
    Ok(())
}

pub fn вывод_содержимого_в_txt(
    ряд: &Vec<String>,
    путь: &String,
    mut сообщения: &mut Vec<String>,
    условие: bool,
) -> Result<(), Error> {
    if ряд.len() == 0 {
        return Ok(());
    }
    //сравнение образов
    match запись_если_есть_разница(&путь, &ряд, &mut сообщения, условие)
    {
        Ok(true) => {
            //println!("Внешне: Перезапись")
        }
        Ok(false) => {
            //println!("Внешне: Отказ от перезаписи");
            return Ok(());
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
fn запись_если_есть_разница(
    путь: &String,
    новый_ряд_строк: &Vec<String>,
    mut сообщения: &mut Vec<String>,
    условие: bool,
) -> io::Result<bool> {
    use crate::utils::functions::{
        вложить_строку_в_ряд_с_проверкой, вывод_сообщения_на_экран_и_вложение_в_ряд,
    };
    if новый_ряд_строк.len() == 0 {
        let сообщение_вывода: String = format!("Запись10: файл {} Пустой. Записывать нечего", путь);
        if условие {
            вывод_сообщения_на_экран_и_вложение_в_ряд(
                сообщение_вывода,
                &mut сообщения,
            )
        } else {
            вложить_строку_в_ряд_с_проверкой(
                &mut сообщения,
                &сообщение_вывода,
            )
        }
        return Ok(true);
    }
    let путь_сохранения = Path::new(&путь);
    if !путь_сохранения.exists() {
        let сообщение_вывода: String =
            format!("Запись5: книга {}. Файл не существует. Запись", путь);
        if условие {
            вывод_сообщения_на_экран_и_вложение_в_ряд(
                сообщение_вывода,
                &mut сообщения,
            )
        } else {
            вложить_строку_в_ряд_с_проверкой(
                &mut сообщения,
                &сообщение_вывода,
            )
        }
        return Ok(true);
    }
    // Читаем существующий файл как байты
    match File::open(&путь) {
        Ok(содержимое) => (),
        Err(ошибка) if ошибка.kind() == io::ErrorKind::NotFound => {
            // Файл не существует - записываем новый
            //fs::write(&путь, новое_содержимое).unwrap();
            let сообщение_вывода: String = format!(
                "Запись:  книга {} не существует файл. Запись. Условие {условие}",
                путь
            );
            if условие {
                вывод_сообщения_на_экран_и_вложение_в_ряд(
                    сообщение_вывода,
                    &mut сообщения,
                )
            } else {
                вложить_строку_в_ряд_с_проверкой(
                    &mut сообщения,
                    &сообщение_вывода,
                )
            }

            return Ok(true);
        }
        Err(ошибка) => return Err(ошибка),
    };
    // Пытаемся прочитать и вычислить хеш существующего файла

    let существующий_файл_ряд_строк: Vec<String> =
        //прочитать_содержимое_построчно(&пути_до_книг[i])
        read_utf8(&путь); //чтение файла в UTF-8

    //сравнение строк
    //если содержимое совпадает
    if сравнение_двух_рядов_построчно(
        &новый_ряд_строк,
        &существующий_файл_ряд_строк,
        &путь,
    ) {
        //условие вывода на экран или только вложения строки в ряд
        let сообщение_вывода = format!(
            "Запись7: книга {}. Полное совпадение. Отказ от перезаписи",
            путь
        );
        if условие {
            вывод_сообщения_на_экран_и_вложение_в_ряд(
                сообщение_вывода,
                &mut сообщения,
            )
        } else {
            вложить_строку_в_ряд_с_проверкой(
                &mut сообщения,
                &сообщение_вывода,
            )
        }
        Ok(false)
    }
    //если содержимое не совпадает - перезаписать
    else {
        let сообщение_вывода = format!(
            "Запись1:  книга {}. Не соответствие содержимого. Перезапись",
            путь
        );
        if условие {
            вывод_сообщения_на_экран_и_вложение_в_ряд(
                сообщение_вывода,
                &mut сообщения,
            )
        } else {
            вложить_строку_в_ряд_с_проверкой(
                &mut сообщения,
                &сообщение_вывода,
            )
        }
        //fs::write(&путь, новое_содержимое).unwrap();
        Ok(true)
    }
}

fn запись_если_есть_разница_архив(
    путь: &String,
    озу_книга_куча: &Архив_в_озу,
    mut сообщения: &mut Vec<String>,
    условие: bool,
) -> io::Result<bool> {
    use crate::output::check::сравнить_2_архива_из_озу;
    use crate::utils::functions::{
        вложить_строку_в_ряд_с_проверкой, вывод_сообщения_на_экран_и_вложение_в_ряд,
    };
    use fs::*;
    use std::io::{Cursor, Read, Write};
    let путь_сохранения = Path::new(&путь);
    //если не существует файл
    if !путь_сохранения.exists() {
        let сообщение_вывода: String =
            format!("Запись6: книга {}. Файл не существует. Запись", путь);
        if условие {
            вывод_сообщения_на_экран_и_вложение_в_ряд(
                сообщение_вывода,
                &mut сообщения,
            )
        } else {
            вложить_строку_в_ряд_с_проверкой(
                &mut сообщения,
                &сообщение_вывода,
            )
        }
        return Ok(true);
    }
    // Читаем существующий файл как байты
    match File::open(&путь) {
        Ok(содержимое) => (),
        Err(ошибка) if ошибка.kind() == io::ErrorKind::NotFound => {
            // Файл не существует - записываем новый
            //fs::write(&путь, новое_содержимое).unwrap();
            let сообщение_вывода: String =
                format!("Запись!:  книга {} Ошибка чтения. Перезапись", путь);
            if условие {
                вывод_сообщения_на_экран_и_вложение_в_ряд(
                    сообщение_вывода,
                    &mut сообщения,
                )
            } else {
                вложить_строку_в_ряд_с_проверкой(
                    &mut сообщения,
                    &сообщение_вывода,
                )
            }
            return Ok(true);
        }
        Err(ошибка) => return Err(ошибка),
    };
    // Пытаемся прочитать и вычислить хеш существующего файла
    //let mut существующий_файл = File::open(путь_сохранения).unwrap();
    //  let mut байты_из_накопителя =Vec::new();
    //существующий_файл.read_to_end(&mut байты_из_накопителя).unwrap();
    //чтение как архива
    let архив: foldhash::HashMap<String, Vec<u8>> =
        foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
    let mut книга_из_файла: Архив_в_озу = архив;

    //прочитать_содержимое_построчно(&пути_до_книг[i])
    //read_utf8(&путь) //чтение файла в UTF-8
    match zip_архив_в_память(&путь, &mut книга_из_файла) {
        Ok(успех) => успех,
        Err(ошибка) => {
            if sz_найти(&ошибка.to_string(), "Пустой файл") {
                вывод_сообщения_на_экран_и_вложение_в_ряд(
                    format!("Запись2:  книга {}. Пустое содержимого. Перезапись", путь),
                    &mut сообщения,
                );
                return Ok(true);
            } else {
                panic!("Ошибка при распаковке файла в архив: {путь}")
            }
        }
    }
    if сравнить_2_архива_из_озу(
        &озу_книга_куча,
        &книга_из_файла,
        &путь,
        условие,
        &mut сообщения,
    ) {
        //совпадает содержимое - не перезаписывать
        Ok(false)
    } else {
        if условие {
            вывод_сообщения_на_экран_и_вложение_в_ряд(
                format!(
                    "Запись3:  книга {}. Не соответствие содержимого из ОЗУ с существующим файлом. Перезапись",
                    путь
                ),
                &mut сообщения,
            )
        } else {
            вложить_строку_в_ряд_с_проверкой(
                &mut сообщения,
                &format!(
                    "Запись4:  книга {}. Не соответствие содержимого из ОЗУ с существующим файлом. Перезапись",
                    путь
                ),
            )
        }
        //fs::write(&путь, новое_содержимое).unwrap();
        Ok(true)
    }
}

//проверка .xlsx файла

fn сравнить_xlsx_файлы(
    mut первый: Sheets<BufReader<File>>,
    mut второй: Sheets<BufReader<File>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    //let mut первый: Sheets<BufReader<File>> = open_workbook_auto(path1).unwrap();
    //let mut второй: Sheets<BufReader<File>>  = open_workbook_auto(path2).unwrap();

    let sheets1 = первый.sheet_names().to_owned();
    let sheets2 = второй.sheet_names().to_owned();

    if sheets1 != sheets2 {
        println!("❌ Разные наборы листов: {:?} vs {:?}", sheets1, sheets2);
        return Ok(false);
    }

    for sheet_name in sheets1 {
        let range1 = первый.worksheet_range(&sheet_name).unwrap();
        let range2 = второй.worksheet_range(&sheet_name).unwrap();

        if range1.get_size() != range2.get_size() {
            println!("❌ Разные размеры листа '{}'", sheet_name);
            return Ok(false);
        }

        for (row, col, val1) in range1.cells() {
            let val2 = range2
                .get_value((row.try_into().unwrap(), col.try_into().unwrap()))
                .unwrap_or(&Data::Empty);

            if val1 != val2 {
                println!(
                    "❌ Разное значение в листе '{}' ячейка ({}, {}): {:?} vs {:?}",
                    sheet_name, row, col, val1, val2
                );
                return Ok(false);
            }
        }
    }

    Ok(true)
}

pub fn сравнение_xlsx_файлов_2_пути<P: AsRef<Path>>(
    путь_1: P,
    путь_2: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut первый = open_workbook_auto(путь_1).unwrap();
    let mut второй = open_workbook_auto(путь_2).unwrap();
    let same = сравнить_xlsx_файлы(первый, второй).unwrap();
    if same {
        println!("✅ Файлы идентичны по содержимому");
    } else {
        println!("⚠️  Файлы отличаются");
    }
    Ok(())
}
/*
pub fn сравнение_xlsx_файлов_2_путь_и_озу<P: AsRef<Path>>(
    первый: &Vec<u8>,
    путь_2: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut первый = open_workbook_from_rs(*первый).unwrap();
    let mut второй: Sheets<BufReader<File>> = open_workbook_auto(путь_2).unwrap();
    let same = сравнить_xlsx_файлы(первый, второй).unwrap();
    if same {
        println!("✅ Файлы идентичны по содержимому");
    } else {
        println!("⚠️  Файлы отличаются");
    }
    Ok(())
}

 */

pub fn main2() -> Result<(), Box<dyn std::error::Error>> {
    let первый = "./запас словарей/Главный словарь1.xlsx".to_string();
    let второй = "./запас словарей/Главный словарь2.xlsx".to_string();
    let mut первый = open_workbook_auto(первый).unwrap();
    //
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let buf = cursor.into_inner();
    //Write::write_all(&mut file, &buf).unwrap();
    //
    let mut второй: Sheets<BufReader<File>> = open_workbook_auto(второй).unwrap();

    Ok(())
}

pub fn вывод_всей_стопки_сообщений_в_txt(
    mut сообщения: Сообщения,
) -> Result<bool, Box<dyn std::error::Error>> {
    let пути_вывода: Пути_Вывода = Default::default();
    let пути_общие: Пути_Общие = Default::default();
    let mut сообщения_общего_выполнения: Mutex<Vec<String>> = Mutex::new(Vec::new());
    //вывод сообщений после замен
    сообщения
        .проверка_после_замен
        .into_par_iter()
        .enumerate()
        .for_each(|(указатель, вложение)| {
            вывод_содержимого_в_txt(
                &вложение.сообщения,
                &format!(
                    "{}{}.txt",
                    пути_вывода.вывод_книг_проверки_замен, вложение.имя_книги
                ),
                &mut сообщения_общего_выполнения.lock().unwrap(),
                false,
            )
            .unwrap();
        });

    //вывод сообщений после замен
    сообщения
        .запись_и_чтение
        .into_par_iter()
        .enumerate()
        .for_each(|(указатель, вложение)| {
            вывод_содержимого_в_txt(
                &вложение.сообщения,
                &format!(
                    "{}{}.txt",
                    пути_общие.вывод_книги_запись_и_чтение, вложение.имя_книги
                ),
                &mut сообщения_общего_выполнения.lock().unwrap(),
                true,
            )
            .unwrap();
        });
    //вложение добавленных сообщений
    сообщения
        .общие
        .extend(сообщения_общего_выполнения.into_inner().unwrap());
    //вывод общих сообщений
    вывод_содержимого_в_txt(
        &сообщения.общие,
        &пути_вывода.вывод_сообщений,
        &mut Vec::new(),
        false,
    )
    .unwrap();
    Ok(true)
}
