use std::fs::read_to_string;

//output паузы для windows - нажмите любую клавишу
pub fn system_pause() {
    // use std::process::Command;
    // let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
pub fn прочитать_содержимое_построчно(
    путь: &String
) -> Vec<String> {
    let mut строки: Vec<String> = vec![];

    for строка in read_to_string(&путь).unwrap().lines() {
        строки.push(строка.to_string())
    }
    return строки;
}
