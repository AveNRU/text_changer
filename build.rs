#[path = "src/dictionary_builder.rs"]
mod dictionary_builder;

use dictionary_builder::создать_словарь_разделителей;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/dictionary_builder.rs");
    println!("cargo:rerun-if-changed=src/");

    // Создаём словарь
    let dict = match создать_словарь_разделителей() {
        Ok(d) => {
            println!("✅ Словарь создан успешно");
            d
        }
        Err(e) => {
            eprintln!("❌ Ошибка создания словаря: {}", e);
            std::process::exit(1);
        }
    };

    println!("📊 Количество элементов: {}", dict.содержимое.len());

    // Сериализуем в MessagePack
    let data = match rmp_serde::to_vec(&dict) {
        Ok(d) => {
            println!("📦 Сериализация успешна, размер: {} байт", d.len());
            d
        }
        Err(e) => {
            eprintln!("❌ Ошибка сериализации: {}", e);
            std::process::exit(1);
        }
    };

    // Сохраняем в OUT_DIR
    let out_dir = env::var("OUT_DIR").unwrap();
    let dict_path = Path::new(&out_dir).join("dictionary.bin");

    match std::fs::write(&dict_path, data) {
        Ok(_) => {
            println!("💾 Словарь сохранён: {}", dict_path.display());
            println!("cargo:rustc-env=DICT_SIZE={}", dict.содержимое.len());
            println!("cargo:rustc-cfg=has_dictionary");
        }
        Err(e) => {
            eprintln!("❌ Ошибка сохранения словаря: {}", e);
            std::process::exit(1);
        }
    }
}
