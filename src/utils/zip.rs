use crate::utils::functions::вывод_сообщения_на_экран_и_вложение_в_ряд;
use foldhash::{HashMap, HashSet, HashSetExt, fast::RandomState, quality::FixedState};
use std::io::{Cursor, SeekFrom};
use std::time::Instant;
use std::{fmt, fs};
use zip::{ZipArchive, ZipWriter};

pub type Архив_в_озу = HashMap<String, Vec<u8>>;

#[derive(Debug)]
enum ZipsError {
    FileNotFound,
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
    Пустойфайл,
}
impl From<std::io::Error> for ZipsError {
    fn from(e: std::io::Error) -> Self {
        ZipsError::IoError(e)
    }
}
impl From<zip::result::ZipError> for ZipsError {
    fn from(e: zip::result::ZipError) -> Self {
        ZipsError::ZipError(e)
    }
}
struct Zips {
    указатель: Cursor<Vec<u8>>,
    хранение_в_озу: Архив_в_озу,
}
impl fmt::Display for ZipsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZipsError::FileNotFound => write!(f, "File not found"),
            ZipsError::IoError(e) => write!(f, "IO error: {}", e),
            ZipsError::ZipError(e) => write!(f, "ZIP error: {}", e),
            ZipsError::Пустойфайл=> write!(f, "Пустое содержимое",),
        }
    }
}
impl std::error::Error for ZipsError {}

impl Zips {
    // Конструктор: читает файл в память, создаёт Cursor и пустой виртуальный FS
    fn new(путь: &str) -> Result<Self, ZipsError> {
        let данные = fs::read(путь).map_err(|_| ZipsError::FileNotFound).unwrap();
        let пустая_стопка_hashmap: foldhash::HashMap<String, Vec<u8>> =
            foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
        Ok(Self {
            указатель: Cursor::new(данные),
            хранение_в_озу: пустая_стопка_hashmap,
        })
    }
    // Распаковка архива fs
    fn распаковать_архив_в_озу(&mut self) -> Result<(), ZipsError> {
        use std::io::{Read, Seek};

        self.указатель.seek(SeekFrom::Start(0)).unwrap();
        if self.указатель.clone().into_inner().len() == 0 {
            return Err(ZipsError::Пустойфайл);
        }
        let mut архив = ZipArchive::new(&mut self.указатель).unwrap();

        for i in 0..архив.len() {
            let mut файл = архив.by_index(i).unwrap();
            let имя = файл.mangled_name().to_string_lossy().into_owned();

            if !файл.is_dir() {
                let mut содержимое = Vec::with_capacity(файл.size() as usize);
                файл.read_to_end(&mut содержимое).unwrap();
                self.хранение_в_озу.insert(имя, содержимое);
            }
        }
        Ok(())
    }
}
pub fn zip_архив_в_память(
    путь: &String,
    virt_fs: &mut Архив_в_озу,
) -> Result<(), Box<dyn std::error::Error>> {
    //HashMap<String, Vec<u8>>{
    let mut zips = Zips::new(путь).unwrap();
    match zips.распаковать_архив_в_озу() {
        Ok(zip) => (),
        Err(ZipsError::Пустойфайл) => return Err(format!("Пустой файл").into()),
        Err(ошибка)=>panic!("Ошибка при распаковке файла в архив: {путь}. Ошибка: {ошибка}"),
    }
    for (путь, содержимое) in zips.хранение_в_озу.iter() {
        virt_fs.insert(путь.clone(), содержимое.clone());
    }
    Ok(())
}

/// Запаковывает виртуальную файловую систему в Vec<u8>
pub fn pack_zip_from_memory(virtual_fs: &Архив_в_озу) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use std::io::Write;

    //println!("запуск запаковки");
    //подсчёт начала запуска времени
    let _начало_времени = Instant::now();

    let mut выводные_данные = Vec::new();
    let указатель = Cursor::new(&mut выводные_данные);
    let mut zip_упаковщик = ZipWriter::new(указатель);

    for (имя_файла, содержимое) in virtual_fs {
        zip_упаковщик
            .start_file(имя_файла, zip::write::SimpleFileOptions::default())
            .unwrap();
        zip_упаковщик.write_all(содержимое).unwrap();
    }

    zip_упаковщик.finish().unwrap();
    //output времени затраченного в итоге
    //println!("Время занятое на запаковку: {:?}", duration);
    Ok(выводные_данные)
}
