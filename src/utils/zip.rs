
use std::io::{Cursor, SeekFrom};
use std::time::Instant;
use std::{fmt, fs};
use zip::{ZipArchive, ZipWriter};
use foldhash::{HashSet, HashSetExt, quality::FixedState, fast::RandomState, HashMap};
pub type VirtualFs = HashMap<String, Vec<u8>>;

#[derive(Debug)]
enum ZipsError {
    FileNotFound,
    IoError(std::io::Error),
    ZipError(zip::result::ZipError),
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
    хранение_в_озу: VirtualFs,
}
impl fmt::Display for ZipsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZipsError::FileNotFound => write!(f, "File not found"),
            ZipsError::IoError(e) => write!(f, "IO error: {}", e),
            ZipsError::ZipError(e) => write!(f, "ZIP error: {}", e),
        }
    }
}
impl std::error::Error for ZipsError {}

impl Zips {
    // Конструктор: читает файл в память, создаёт Cursor и пустой виртуальный FS
    fn new(путь: &str) -> Result<Self, ZipsError> {
        let данные = fs::read(путь).map_err(|_| ZipsError::FileNotFound).unwrap();
        let пустая_стопка_hashmap: foldhash::HashMap<String, Vec<u8>> = foldhash::HashMap::with_hasher(foldhash::fast::RandomState::default());
        Ok(Self {
            указатель: Cursor::new(данные),
            хранение_в_озу: пустая_стопка_hashmap,
        })
    }
    // Распаковка архива fs
    fn распаковать_архив_в_озу(&mut self) -> Result<(), ZipsError> {
        use std::io::{Read, Seek};

        self.указатель.seek(SeekFrom::Start(0)).unwrap();
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
    строка: &String,
    virt_fs: &mut VirtualFs,
) -> Result<(), Box<dyn std::error::Error>> {
    //HashMap<String, Vec<u8>>{
    let mut zips = Zips::new(строка).unwrap();

    zips.распаковать_архив_в_озу().unwrap();
    for (путь, содержимое) in zips.хранение_в_озу.iter() {
        virt_fs.insert(путь.clone(), содержимое.clone());
    }
    Ok(())
}

/// Запаковывает виртуальную файловую систему в Vec<u8>
pub fn pack_zip_from_memory(virtual_fs: &VirtualFs) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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
