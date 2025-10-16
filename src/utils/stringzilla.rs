use std::collections::HashSet;
use stringzilla::sz;
use rayon::prelude::*;
use std::sync::{Mutex, atomic::{AtomicU64, Ordering,AtomicUsize}};

pub fn sz_найти(строка: &String, образец: &str) -> bool {
    if let Some(указатель) = sz::find(&строка, образец) {
        return true;
    }
    return false;
}

pub fn sz_найти_в_ряде(ряд: &Vec<String>, образец: &str) -> bool {
    ряд.into_par_iter().enumerate().find_map_any(|(указатель,строка)|
        {
            if sz_найти(строка, образец) {
                Some(true) // возвращаем Some с любым значением, важно что не None
            } else {
                None
            }
        }
    ).is_some()
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
