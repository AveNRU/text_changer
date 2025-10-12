use std::collections::HashSet;
use stringzilla::sz;

pub fn sz_найти(строка: &String, образец: &str) -> bool {
    if let Some(указатель) = sz::find(&строка, образец) {
        return true;
    }
    return false;
}

pub fn sz_найти_в_ряде(ряд: &Vec<String>, образец: &str) -> bool {
    for строка in ряд.iter() {
        if sz_найти(&строка, &образец) {
            return true;
        }
    }
    return false;
}

pub fn sz_пусто(строка: &String) -> bool {
    if let Some(указатель) = sz::find(&строка, "Пусто") {
        return true;
    }
    return false;
}

pub fn sz_упорядочить_ряд_строк(ряд: Vec<String>) -> Vec<String> {
    let mut порядок: Vec<usize> = vec![0; ряд.len()];
    sz::argsort_permutation(&ряд, &mut порядок).unwrap();
    let mut новый_ряд: Vec<String> = Vec::new();
    for число in порядок.into_iter() {
        новый_ряд.push(ряд[число].clone());
    }
    return новый_ряд;
}
