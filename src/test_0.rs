//use clap::builder::Str;
//use convert_case::{Case, Casing};
//use rapidhash::*;
use rayon::prelude::*;
pub fn сравнить_основной_и_запасной_словари(
    основной_словарь: &Text_Changer::Полный_Словарь,
    запасной_словарь: &Text_Changer::Полный_Словарь,
) -> Result<(), ()> {
    println!(
        "Длина простых слов: Основной - |{}| Запасной - |{}|",
        основной_словарь.простое.len(),
        запасной_словарь.простое.len(),
    );
    //
    let куча_простых_слов: rapidhash::fast::RapidHashSet<&String> = основной_словарь
        .простое
        .par_iter()
        .filter_map(|ячейка| Some(&ячейка.искомое_слово))
        .collect::<rapidhash::fast::RapidHashSet<&String>>();
    //
    println!("Длина кучи простых слов - |{}|", куча_простых_слов.len());
    //
    let куча_указателей_на_словарь_запасной: rapidhash::fast::RapidHashSet<usize> =
        запасной_словарь
            .простое
            .par_iter()
            .enumerate() //.filter(|(указатель,ячейка)|!ячейка.искомое_слово.is_empty())
            .filter_map(|(указатель, ячейка)| {
                //если нет искомого слова из запасного словаря - то добавить его указатель в кучу
                let ряд_знаков: Vec<char> = ячейка.искомое_слово.chars().collect();
                if ряд_знаков[0].is_uppercase() {
                    return None;
                }
                //
                if !куча_простых_слов.contains(&ячейка.искомое_слово)
                {
                    Some(указатель)
                } else {
                    None
                }
            })
            .collect::<rapidhash::fast::RapidHashSet<usize>>();
    //
    println!(
        "Количество слов запасного словаря, которые отсутствуют в основном - |{}|",
        куча_указателей_на_словарь_запасной.len(),
    );
    //
    crate::output::write::вывод_запасного_словаря_почищенного(
        &запасной_словарь,
        &куча_указателей_на_словарь_запасной,
    )
    .unwrap();

    Ok(())
}
