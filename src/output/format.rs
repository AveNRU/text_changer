use rust_xlsxwriter::*;
use std::sync::LazyLock;
pub static ПО_СЕРЕДИНЕ_БЕЗ_ПЕРЕНОСА: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
});

pub static по_середине_c_переносом: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_text_wrap()
});

pub static ПО_СЕРЕДИНЕ_СЕРАЯ_ЗАЛИВКА: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_background_color("#DDD9C4")
        .set_text_wrap()
});

pub static ВЫРАВНЯТЬ_СЛЕВА: LazyLock<Format> =
    LazyLock::new(|| Format::new().set_align(FormatAlign::Left).set_text_wrap());

pub static _CHAPTER: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border_color("#00B0F0")
        .set_text_wrap()
});

pub static _WRONG: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_font_color("#FF0000")
        .set_text_wrap()
});

pub static НЕТ_ОШИБОК: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_font_color("#54A42C")
        .set_text_wrap()
});

pub static ТРЕВОГА: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_font_color("#FFFF00")
        .set_background_color(Color::RGB(0x974706))
        .set_text_wrap()
});

pub static _WARNING_2: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_font_color("#7B68EE")
        .set_text_wrap()
});

pub static ЖИРНЫЙ_ПО_СЕРЕДИНЕ: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_text_wrap()
});

pub static _BOLD_TEXT_GREY_FONT: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_font_color("#DDD9C4")
        .set_text_wrap()
});

pub static ГОЛУБОЙ_ПОЛУЖИРНЫЙ: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_font_color("#0000FF")
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_border_color("#2F75B5")
        .set_text_wrap()
});

pub static ФИОЛЕТОВЫЙ_ПОЛУЖИРНЫЙ: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_font_color("#7030A0")
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_border_color("#2F75B5")
        .set_text_wrap()
});

pub static ЗЕЛЕНОВАТЫЙ_ПОЛУЖИРНЫЙ: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_font_color("#00B050")
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_border_color("#2F75B5")
        .set_text_wrap()
});

pub static ГОЛУБАЯ_РАМКА: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_italic()
        .set_font_color("#2F75B5")
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_border_color("#2F75B5")
        .set_text_wrap()
});

pub static _КОРИЧНЕВАЯ_РАМКА: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_italic()
        .set_font_color("#0D0D0D")
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Double)
        .set_border_color("#0D0D0D")
        .set_text_wrap()
});

pub static ЗАГОЛОВОК_ЦВЕТ_XLS: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_bold()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_background_color(Color::RGB(0x9ED5EF))
        .set_text_wrap()
});

pub static ОШИБКА_КРАСНЫЙ_ЦВЕТ: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_background_color(Color::RGB(0xFF0000))
        .set_text_wrap()
});

pub static УСПЕХ_ЦВЕТ_XLSX: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_background_color(Color::RGB(0x54A42C))
        .set_text_wrap()
});

pub static ПРЕДУПРЕЖДЕНИЕ_ЖЁЛТЫЙ_ЦВЕТ: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_background_color(Color::RGB(0xFFFF00))
        .set_text_wrap()
});
pub static БЛЕДНО_ЗЕЛЁНЫЙ_ЦВЕТ: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_background_color(Color::RGB(0xD8E4BC))
        .set_text_wrap()
});
pub static ЗЕЛЁНЫЙ_ЦВЕТ: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_background_color(Color::RGB(0x22AA2C))
        .set_text_wrap()
});
pub static БЛЕДНО_РОЗОВЫЙ_ЦВЕТ: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_background_color(Color::RGB(0xF2DCDB))
        .set_text_wrap()
});

pub static FORMAT_CONFIG_1: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_background_color(Color::RGB(0x00FF00))
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
});

pub static FORMAT_CONFIG_0: LazyLock<Format> = LazyLock::new(|| {
    Format::new()
        .set_background_color(Color::RGB(0xFF0000))
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
});
