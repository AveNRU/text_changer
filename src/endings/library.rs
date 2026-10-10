use Text_Changer::{
    Вид_Окончания, Вид_ошибки_при_поиске_в_торжке, *
};
use console::style;
use rapidhash::RapidHashSet;
//use rayon::prelude::*;
use regex::Regex;
use std::sync::LazyLock;
//use std::sync::atomic::{AtomicBool, Ordering};
#[allow(non_camel_case_types)]
pub struct Ложные_окончания_с_исключениями {
    pub ряд_re: &'static [Regex],
    pub ряд_исключений: &'static rapidhash::fast::RapidHashSet<&'static str>,
}
pub static СТОПКА_ЛОЖНЫХ_ОКОНЧАНИЙ: LazyLock<
    [Ложные_окончания_с_исключениями; 31],
> = LazyLock::new(|| {
    [
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ТИ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ТИ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ВЕР,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ВЕР,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ВАН,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ВАН,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕН,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ЕН,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ОН,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ОН,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЁН,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ЁН,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Н,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Н,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_СТ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_СТ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АС,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_АС,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АН,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_АН,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Щ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Щ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_И,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_И,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЫИ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ЫИ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ОМ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ОМ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕМ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ЕМ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ц,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Ц,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Л,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Л,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ю,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Ю,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ж,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Ж,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ш,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Ш,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_К,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_К,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_М,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_М,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ч,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Ч,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_У,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_У,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕТ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ЕТ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АРН,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_АРН,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АЦ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_АЦ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Т,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Т,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Д,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Д,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Р,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Р,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Х,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_Х,
        },
    ]
});
pub static RE_ИСКЛЮЧЕНИЯ_ВЕР: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ВЕР: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)вер").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_ТИ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["гарантией"]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ТИ: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)тией$").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Х: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["шерсть", "шерстью"]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Х: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)хошл").unwrap(),
        Regex::new(r"(?i)цаз").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Р: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["шерсть", "шерстью"]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Р: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)рсть").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Д: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Д: LazyLock<[Regex; 0]> = LazyLock::new(|| {
    [//Regex::new(r"(?i)дрилей$").unwrap()
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_АЦ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АЦ: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)ацие$").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Т: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Т: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)ттом$").unwrap(),
        Regex::new(r"(?i)ттю$").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_М: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_М: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)муюсь$").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_АРН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АРН: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)арыми$").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_У: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["статуи", "сабантуи", "буржуи"]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_У: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)уи$").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_ЕТ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕТ: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)етется$").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Ч: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ч: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)чев$").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_К: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_К: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)кы$").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Ш: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["афишы"]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ш: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)шы$").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Ж: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ж: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)жы$").unwrap(),
        Regex::new(r"(?i)жев$").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Ю: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ю: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)юися").unwrap()]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Л: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Л: LazyLock<[Regex; 16]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)льнная").unwrap(),
        Regex::new(r"(?i)льнную").unwrap(),
        Regex::new(r"(?i)льнна").unwrap(),
        Regex::new(r"(?i)льнно").unwrap(),
        Regex::new(r"(?i)льнны").unwrap(),
        Regex::new(r"(?i)льнные").unwrap(),
        Regex::new(r"(?i)льнным").unwrap(),
        Regex::new(r"(?i)льнными").unwrap(),
        Regex::new(r"(?i)льнных").unwrap(),
        Regex::new(r"(?i)льнный").unwrap(),
        Regex::new(r"(?i)льнном").unwrap(),
        Regex::new(r"(?i)льнное").unwrap(),
        Regex::new(r"(?i)льнного").unwrap(),
        Regex::new(r"(?i)льнной").unwrap(),
        Regex::new(r"(?i)льнному").unwrap(),
        Regex::new(r"(?i)леная").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Ц: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ц: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)цеяев").unwrap(),
        Regex::new(r"(?i)цеяем").unwrap(),
    ]
    //----------------------------------
});

//
pub static RE_ИСКЛЮЧЕНИЯ_ЕМ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕМ: LazyLock<[Regex; 9]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)ем$").unwrap(),
        Regex::new(r"(?i)еми$").unwrap(),
        Regex::new(r"(?i)емуемся").unwrap(),
        Regex::new(r"(?i)емует").unwrap(),
        Regex::new(r"(?i)емуетя").unwrap(),
        Regex::new(r"(?i)емуй").unwrap(),
        Regex::new(r"(?i)емуюсь").unwrap(),
        Regex::new(r"(?i)емуют").unwrap(),
        Regex::new(r"(?i)емуются").unwrap(),
    ]
    //----------------------------------
});
//

pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЫИ: LazyLock<[Regex; 25]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)аых").unwrap(),
        Regex::new(r"(?i)ыванно").unwrap(),
        Regex::new(r"(?i)ыванными").unwrap(),
        Regex::new(r"(?i)ыванных").unwrap(),
        Regex::new(r"(?i)ыванным").unwrap(),
        Regex::new(r"(?i)ыванный").unwrap(),
        Regex::new(r"(?i)ыванные").unwrap(),
        Regex::new(r"(?i)ыванную").unwrap(),
        Regex::new(r"(?i)ыванностям").unwrap(),
        Regex::new(r"(?i)ыванностями").unwrap(),
        Regex::new(r"(?i)ыванностях").unwrap(),
        Regex::new(r"(?i)ыванностью").unwrap(),
        Regex::new(r"(?i)ыванность").unwrap(),
        Regex::new(r"(?i)ыванности").unwrap(),
        Regex::new(r"(?i)ыванностей").unwrap(),
        Regex::new(r"(?i)ыванной").unwrap(),
        Regex::new(r"(?i)ыванное").unwrap(),
        Regex::new(r"(?i)ыванного").unwrap(),
        Regex::new(r"(?i)ыванная").unwrap(),
        Regex::new(r"(?i)ыго$").unwrap(),
        Regex::new(r"(?i)ыого$").unwrap(),
        Regex::new(r"(?i)ыи$").unwrap(),
        Regex::new(r"(?i)ымм$").unwrap(),
        Regex::new(r"(?i)уый$").unwrap(),
        Regex::new(r"(?i)ыым").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_ЫИ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ОМ: LazyLock<[Regex; 5]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)ойм$").unwrap(),
        Regex::new(r"(?i)омю").unwrap(),
        Regex::new(r"(?i)оу$").unwrap(),
        Regex::new(r"(?i)оую$").unwrap(),
        Regex::new(r"(?i)ою$").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_ОМ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "шоу",
            "конвою",
            "супергерою",
            "герою",
            "главгерою",
            "антигерою",
            "строю",
            "сбою",
            "застою",
            "сухостою",
            "бою",
        ])
    });
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_И: LazyLock<[Regex; 15]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)иенами$").unwrap(),
        Regex::new(r"(?i)иев$").unwrap(),
        Regex::new(r"(?i)иархы").unwrap(),
        Regex::new(r"(?i)иатов").unwrap(),
        Regex::new(r"(?i)иану").unwrap(),
        Regex::new(r"(?i)ианы").unwrap(),
        Regex::new(r"(?i)иаты").unwrap(),
        Regex::new(r"(?i)ийи").unwrap(),
        // Regex::new(r"(?i)ийями").unwrap(),
        Regex::new(r"(?i)ийям").unwrap(),
        Regex::new(r"(?i)мзм").unwrap(),
        Regex::new(r"(?i)ианым").unwrap(),
        Regex::new(r"(?i)илей").unwrap(),
        Regex::new(r"(?i)иеная").unwrap(),
        //
        Regex::new(r"(?i)ивено").unwrap(),
        Regex::new(r"(?i)иру$").unwrap(),
        //Regex::new(r"(?i)ичн\w{0,2}$").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_И: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "автомобилей",
            "пролетариатов",
            "пролетариаты",
            "опиаты",
            "опиатов",
            "турниру",
            "текстилей",
            "профилей",
            "медианы",
            "ювелиру",
            "юбилей",
            "тиру",
            "стилей",
            "телеэфиру",
            "сувениру",
            "секретариатов",
            "секретариаты",
            "рэкетиру",
            "пунктиру",
            "противо-миру",
            "макромиру",
            "кумиру",
            "дезертиру",
            "вампиру",
            "бригадиру",
            "просимианы",
            "просимиану",
            "плагиатов",
            "плагиаты",
            "меридианы",
            "меридиану",
            "гигиенами",
            "гениев",
            "кафетериев",
            "колумбариев",
            "комментариев",
            "критериев",
            "соляриев",
            "сценариев",
            "пассионариев",
            "санаториев",
            "глоссариев",
            "репозиториев",
            "париев",
            "антиквариатов",
            "антиквариаты",
            "антимиру",
            "миру",
            "банкиру",
            "командиру",
            "конвоиру",
            "квартиру",
            "кашмиру",
            "кашемиру",
            "кассиру",
            "мундиру",
            "ориентиру",
            "пассажиру",
            "пиру",
            "радиоэфиру",
            "эфиру",
            "трактиру",
            "транжиру",
            "эликсиру",
        ])
    });
//
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Щ: LazyLock<[Regex; 3]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)щею$").unwrap(),
        Regex::new(r"(?i)щюю$").unwrap(),
        Regex::new(r"(?i)щни\w+$").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Щ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АН: LazyLock<[Regex; 5]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)аннией$").unwrap(),
        Regex::new(r"(?i)анией$").unwrap(),
        Regex::new(r"(?i)аности$").unwrap(),
        Regex::new(r"(?i)анны$").unwrap(),
        Regex::new(r"(?i)аыми").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_АН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АС: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)ася$").unwrap(),
        Regex::new(r"(?i)ауюся$").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_АС: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Н: LazyLock<[Regex; 24]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)ннкой").unwrap(),
        Regex::new(r"(?i)наыми").unwrap(),
        Regex::new(r"(?i)нкю").unwrap(),
        Regex::new(r"(?i)наую").unwrap(),
        Regex::new(r"(?i)ныую").unwrap(),
        Regex::new(r"(?i)нныого").unwrap(),
        Regex::new(r"(?i)нныо").unwrap(),
        Regex::new(r"(?i)ннн").unwrap(),
        Regex::new(r"(?i)ныи").unwrap(),
        Regex::new(r"(?i)нм$").unwrap(),
        Regex::new(r"(?i)ни$").unwrap(),
        Regex::new(r"(?i)нвми").unwrap(),
        Regex::new(r"(?i)ностех").unwrap(),
        Regex::new(r"(?i)нины").unwrap(),
        Regex::new(r"(?i)ноая").unwrap(),
        Regex::new(r"(?i)нао").unwrap(),
        Regex::new(r"(?i)нгося").unwrap(),
        Regex::new(r"(?i)ноого").unwrap(),
        Regex::new(r"(?i)ннией").unwrap(),
        //
        Regex::new(r"(?i)нскосту").unwrap(),
        Regex::new(r"(?i)нскости").unwrap(),
        Regex::new(r"(?i)нскостей").unwrap(),
        Regex::new(r"(?i)нскостя").unwrap(),
        Regex::new(r"(?i)нскость").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Н: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "волосо-брильни",
            "дворянины",
            "героини",
            "мишени",
            "кухни",
            "барышни",
            "болезни",
            "песни",
            "гортани",
            "ровни",
            "знамени",
            "дни",
            "ткани",
            "времени",
            "камни",
            "купальни",
            "харчёвни",
            "огни",
            "отмени",
            "парни",
            "святыни",
            "стеклоткани",
            "уровни",
            "степени",
            "ступени",
            "семени",
            "сходни",
            "раздевальни",
            "перечни",
            "гавани",
            "деревни",
            "бойни",
            "брильни",
            "древни",
            "поварни",
            "дву-сторонни",
            "потусторонни",
            "одно-сторонни",
        ])
    });
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_СТ: LazyLock<[Regex; 14]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)ост$").unwrap(),
        Regex::new(r"(?i)си$").unwrap(),
        Regex::new(r"(?i)стй").unwrap(),
        Regex::new(r"(?i)стю$").unwrap(),
        Regex::new(r"(?i)стяях").unwrap(),
        Regex::new(r"(?i)стех").unwrap(),
        Regex::new(r"(?i)ствы").unwrap(),
        Regex::new(r"(?i)стьям").unwrap(),
        Regex::new(r"(?i)стьях").unwrap(),
        Regex::new(r"(?i)стьей").unwrap(),
        Regex::new(r"(?i)рстей").unwrap(),
        Regex::new(r"(?i)рстях").unwrap(),
        Regex::new(r"(?i)рстям").unwrap(),
        Regex::new(r"(?i)ствов$").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_СТ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "звуко-записи",
            "захолустьям",
            "захолустьями",
            "захолустьях",
            "шерстей",
            "шерстями",
            "шерстях",
            "шерстям",
            "народо-описи",
            "наклонно-писи",
            "господствовать",
            "аудиозаписи",
            "гостю",
            "ипостаси",
            "пси",
            "такси",
            "шасси",
            "ереси",
            "записи",
            "летописи",
            "лжелетописи",
            "надписи",
            "рукописи",
            "описи",
            "живописи",
            "смеси",
            "помеси",
            "росписи",
            "писи",
            "искоси",
        ])
    });
//
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ВАН: LazyLock<[Regex; 39]> = LazyLock::new(|| {
    [
        //----------------------------------
        //ван
        //
        Regex::new(r"(?i)авыван$").unwrap(),
        //
        //а
        Regex::new(r"(?i)ваная$").unwrap(),
        //одиночн
        Regex::new(r"(?i)ванн$").unwrap(),
        Regex::new(r"(?i)ванние$").unwrap(),
        Regex::new(r"(?i)ваннием$").unwrap(),
        Regex::new(r"(?i)ванния$").unwrap(),
        Regex::new(r"(?i)ваннию$").unwrap(),
        Regex::new(r"(?i)ваннии$").unwrap(),
        Regex::new(r"(?i)ванний$").unwrap(),
        Regex::new(r"(?i)ванниям$").unwrap(),
        Regex::new(r"(?i)ванниями$").unwrap(),
        Regex::new(r"(?i)ванниях$").unwrap(),
        Regex::new(r"(?i)ваннях$").unwrap(),
        Regex::new(r"(?i)ванней$").unwrap(),
        Regex::new(r"(?i)ваннью$").unwrap(),
        Regex::new(r"(?i)ванням$").unwrap(),
        Regex::new(r"(?i)ваннь$").unwrap(),
        Regex::new(r"(?i)ваннями$").unwrap(),
        //
        Regex::new(r"(?i)ванна$").unwrap(),
        Regex::new(r"(?i)ванно$").unwrap(),
        Regex::new(r"(?i)ванны$").unwrap(),
        //ую
        Regex::new(r"(?i)ваную$").unwrap(),
        //о
        Regex::new(r"(?i)ваное$").unwrap(),
        Regex::new(r"(?i)ваного$").unwrap(),
        //глаголы
        Regex::new(r"(?i)ваною$").unwrap(),
        Regex::new(r"(?i)ваном$").unwrap(),
        Regex::new(r"(?i)ваному$").unwrap(),
        Regex::new(r"(?i)ваность$").unwrap(),
        Regex::new(r"(?i)ваностю$").unwrap(),
        Regex::new(r"(?i)ваности$").unwrap(),
        Regex::new(r"(?i)ваностей$").unwrap(),
        Regex::new(r"(?i)ваностям$").unwrap(),
        Regex::new(r"(?i)ваностями$").unwrap(),
        Regex::new(r"(?i)ваностях$").unwrap(),
        //ыа
        Regex::new(r"(?i)ваным$").unwrap(),
        Regex::new(r"(?i)ваными$").unwrap(),
        Regex::new(r"(?i)ваных$").unwrap(),
        Regex::new(r"(?i)ваные$").unwrap(),
        Regex::new(r"(?i)ваный$").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_ВАН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["диваном"]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕН: LazyLock<[Regex; 22]> = LazyLock::new(|| {
    [
        //ен
        //а
        Regex::new(r"(?i)еая$").unwrap(),
        Regex::new(r"(?i)еная$").unwrap(),
        //одиночн
        //Regex::new(r"(?i)енна$").unwrap(),
        //Regex::new(r"(?i)енно$").unwrap(),
        //Regex::new(r"(?i)енны$").unwrap(),
        Regex::new(r"(?i)енн$").unwrap(),
        Regex::new(r"(?i)енее$").unwrap(),
        //Regex::new(r"(?i)енна$").unwrap(),
        //Regex::new(r"(?i)енно$").unwrap(),
        //Regex::new(r"(?i)енны$").unwrap(),
        //ую
        Regex::new(r"(?i)еную$").unwrap(),
        //о
        Regex::new(r"(?i)еное$").unwrap(),
        Regex::new(r"(?i)еного$").unwrap(),
        //глаголы
        Regex::new(r"(?i)еном$").unwrap(),
        Regex::new(r"(?i)еному$").unwrap(),
        Regex::new(r"(?i)еность$").unwrap(),
        Regex::new(r"(?i)еностью$").unwrap(),
        Regex::new(r"(?i)ености$").unwrap(),
        Regex::new(r"(?i)еностей$").unwrap(),
        Regex::new(r"(?i)еностям$").unwrap(),
        Regex::new(r"(?i)еностями$").unwrap(),
        Regex::new(r"(?i)еностях$").unwrap(),
        //ы
        Regex::new(r"(?i)еным$").unwrap(),
        Regex::new(r"(?i)еными$").unwrap(),
        Regex::new(r"(?i)еных$").unwrap(),
        Regex::new(r"(?i)еные$").unwrap(),
        Regex::new(r"(?i)енаж$").unwrap(),
        Regex::new(r"(?i)веной$").unwrap(),
        //----------------------------------
    ]
});
pub static RE_ИСКЛЮЧЕНИЯ_ЕН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "звеном",
            "галлюциногеном",
            "рекордсменом",
            "орденом",
            "манекеном",
            "геному",
            "бизнесменом",
            "барменом",
            "аборигеном",
            "рекордсмен",
            "манекен",
            "орден",
            "концераген",
            "геном",
            "бизнесмен",
            "бармен",
            "антенн",
            "суперменом",
            "феноменом",
            "шатеном",
            "экзаменом",
            "доменом",
            "джентльменом",
            "голоценом",
            "авиазвеном",
            "промоушеном",
            "военна",
            "военно",
            "гипоаллергенно",
            "аборигенна",
            "аборигенно",
            "аборигенны",
            "антивоенна",
            "бедственна",
            "бедственно",
        ])
    });

//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЁН: LazyLock<[Regex; 19]> = LazyLock::new(|| {
    [
        //ён
        //а
        Regex::new(r"(?i)ёная$").unwrap(),
        //одиночн
        Regex::new(r"(?i)ённ$").unwrap(),
        //Regex::new(r"(?i)ённа$").unwrap(),
        //Regex::new(r"(?i)ённо$").unwrap(),
        //Regex::new(r"(?i)ённы$").unwrap(),
        //ую
        Regex::new(r"(?i)ёную$").unwrap(),
        //о
        Regex::new(r"(?i)ёное$").unwrap(),
        Regex::new(r"(?i)ёного$").unwrap(),
        //глаголы
        Regex::new(r"(?i)ёном$").unwrap(),
        Regex::new(r"(?i)ёному$").unwrap(),
        Regex::new(r"(?i)ёность$").unwrap(),
        Regex::new(r"(?i)ёностью$").unwrap(),
        Regex::new(r"(?i)ёности$").unwrap(),
        Regex::new(r"(?i)ёностей$").unwrap(),
        Regex::new(r"(?i)ёностям$").unwrap(),
        Regex::new(r"(?i)ёностями$").unwrap(),
        Regex::new(r"(?i)ёностях$").unwrap(),
        //ы
        Regex::new(r"(?i)ёным$").unwrap(),
        Regex::new(r"(?i)ёными$").unwrap(),
        Regex::new(r"(?i)ёных$").unwrap(),
        Regex::new(r"(?i)ёные$").unwrap(),
        Regex::new(r"(?i)ёены").unwrap(),
        //----------------------------------
        //----------------------------------
    ]
});
//
pub static RE_ИСКЛЮЧЕНИЯ_ЁН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ОН: LazyLock<[Regex; 41]> = LazyLock::new(|| {
    [
        //он
        Regex::new(r"(?i)оеную$").unwrap(),
        Regex::new(r"(?i)оная$").unwrap(),
        //одиночн
        Regex::new(r"(?i)онн$").unwrap(),
        //Regex::new(r"(?i)онна$").unwrap(),
        // Regex::new(r"(?i)онно$").unwrap(),
        //Regex::new(r"(?i)онны$").unwrap(),
        //ую
        Regex::new(r"(?i)оную$").unwrap(),
        //о
        Regex::new(r"(?i)оное$").unwrap(),
        Regex::new(r"(?i)оного$").unwrap(),
        //глаголы
        //Regex::new(r"(?i)оном$").unwrap(),
        Regex::new(r"(?i)оному$").unwrap(),
        Regex::new(r"(?i)оность$").unwrap(),
        Regex::new(r"(?i)оностью$").unwrap(),
        Regex::new(r"(?i)оности$").unwrap(),
        Regex::new(r"(?i)оностей$").unwrap(),
        Regex::new(r"(?i)оностям$").unwrap(),
        Regex::new(r"(?i)оностями$").unwrap(),
        Regex::new(r"(?i)оностях$").unwrap(),
        //ы
        Regex::new(r"(?i)оным$").unwrap(),
        Regex::new(r"(?i)оными$").unwrap(),
        Regex::new(r"(?i)оных$").unwrap(),
        Regex::new(r"(?i)оные$").unwrap(),
        Regex::new(r"(?i)огих$").unwrap(),
        Regex::new(r"(?i)оста$").unwrap(),
        //
        Regex::new(r"(?i)онена$").unwrap(),
        Regex::new(r"(?i)онено$").unwrap(),
        Regex::new(r"(?i)оненого$").unwrap(),
        Regex::new(r"(?i)оненое$").unwrap(),
        Regex::new(r"(?i)оненой$").unwrap(),
        Regex::new(r"(?i)оненом$").unwrap(),
        Regex::new(r"(?i)оненому$").unwrap(),
        Regex::new(r"(?i)онености$").unwrap(),
        Regex::new(r"(?i)оненостей$").unwrap(),
        Regex::new(r"(?i)оненость$").unwrap(),
        Regex::new(r"(?i)оненостью$").unwrap(),
        Regex::new(r"(?i)оненостям$").unwrap(),
        Regex::new(r"(?i)оненостями$").unwrap(),
        Regex::new(r"(?i)оненостях$").unwrap(),
        Regex::new(r"(?i)оненую$").unwrap(),
        Regex::new(r"(?i)онены$").unwrap(),
        Regex::new(r"(?i)оненые$").unwrap(),
        Regex::new(r"(?i)оненый$").unwrap(),
        Regex::new(r"(?i)оненым$").unwrap(),
        Regex::new(r"(?i)онеными$").unwrap(),
        Regex::new(r"(?i)оненых$").unwrap(),
    ]
});
pub static RE_ИСКЛЮЧЕНИЯ_ОН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "узаконена",
            "узаконены",
            "узаконено",
            "хвоста",
            "хоста",
            "тоста",
            "строгих",
            "репоста",
            "проста",
            "роста",
            "поста",
            "скомпоную",
            "астроному",
            "колонн",
            "помоста",
            "дву-ногих",
            "заслонено",
            "заслонена",
            "незаслонены",
            "незаслонена",
            "заслонены",
        ])
    });
//
/*pub static ИСКЛЮЧЕНИЯ_ПОИСКА_ВЫДЕРА: LazyLock<Исключения_поиска_выдера> =
LazyLock::new(|| Исключения_поиска_выдера {
    ван: vec![Исключение_поиска_выдера {
        вид_окончания: Text_Changer::Вид_Окончания::Ван,
        куча_окончаний: rapidhash::fast::RapidHashSet::from_iter(["вано"]),
        куча_обрезков_слов: rapidhash::fast::RapidHashSet::from_iter(["нир"]),
    }],
});*/
//

//

/*pub struct Regex_с_заменой {
    поиск: Regex,
    замена: Regex,
}*/
/*
pub static RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ: LazyLock<
    [RE_исключающее_окончание;63],
> = LazyLock::new(|| {
    [

//под конец уже идут исключения
//
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)ш$").unwrap(),
          окончание_строка: "ш",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)ша$").unwrap(),
          окончание_строка: "ша",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шая$").unwrap(),
          окончание_строка: "шая",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шую$").unwrap(),
          окончание_строка: "шую",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)ше$").unwrap(),
          окончание_строка: "ше",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)ши$").unwrap(),
          окончание_строка: "ши",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шем$").unwrap(),
          окончание_строка: "шем",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шею$").unwrap(),
          окончание_строка: "шею",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шему$").unwrap(),
          окончание_строка: "шему",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шего$").unwrap(),
          окончание_строка: "шего",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шее$").unwrap(),
          окончание_строка: "шее",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шей$").unwrap(),
          окончание_строка: "шей",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шесть$").unwrap(),
          окончание_строка: "шесть",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шестью$").unwrap(),
          окончание_строка: "шестью",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шести$").unwrap(),
          окончание_строка: "шести",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шестей$").unwrap(),
          окончание_строка: "шестей",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шестям$").unwrap(),
          окончание_строка: "шестям",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шестями$").unwrap(),
          окончание_строка: "шестями",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шестях$").unwrap(),
          окончание_строка: "шестях",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шим$").unwrap(),
          окончание_строка: "шим",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шими$").unwrap(),
          окончание_строка: "шими",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)ший$").unwrap(),
          окончание_строка: "ший",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)шие$").unwrap(),
          окончание_строка: "шие",
          вид_окончания: Вид_Окончания::_Ш,
      },
      RE_исключающее_окончание {
          re_образец: Regex::new(r"(?i)ших$").unwrap(),
          окончание_строка: "ших",
          вид_окончания: Вид_Окончания::_Ш,
      },
      //-------------

//------------------то
/*RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)та$").unwrap(),
    окончание_строка: "та",
    вид_окончания: Вид_Окончания::То,
},
/*RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)т$").unwrap(),
    окончание_строка: "т",
    вид_окончания: Вид_Окончания::То,
},*/
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тая$").unwrap(),
    окончание_строка: "тая",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тую$").unwrap(),
    окончание_строка: "тую",
    вид_окончания: Вид_Окончания::То,
},*/
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тое$").unwrap(),
    окончание_строка: "тое",
    вид_окончания: Вид_Окончания::То,
},
/*RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)ты$").unwrap(),
    окончание_строка: "ты",
    вид_окончания: Вид_Окончания::Ты,
},*/
//
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)том$").unwrap(),
    окончание_строка: "том",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тому$").unwrap(),
    окончание_строка: "тому",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)то$").unwrap(),
    окончание_строка: "то",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)того$").unwrap(),
    окончание_строка: "того",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тое$").unwrap(),
    окончание_строка: "тое",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)той$").unwrap(),
    окончание_строка: "той",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тость$").unwrap(),
    окончание_строка: "тость",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тостью$").unwrap(),
    окончание_строка: "тостью",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тости$").unwrap(),
    окончание_строка: "тости",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тостей$").unwrap(),
    окончание_строка: "тостей",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тостям$").unwrap(),
    окончание_строка: "тостям",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тостями$").unwrap(),
    окончание_строка: "тостями",
    вид_окончания: Вид_Окончания::То,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)тостях$").unwrap(),
    окончание_строка: "тостях",
    вид_окончания: Вид_Окончания::То,
},
//ыт
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ыт$").unwrap(),
   окончание_строка: "ыт",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ыта$").unwrap(),
   окончание_строка: "ыта",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытая$").unwrap(),
   окончание_строка: "ытая",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытую$").unwrap(),
   окончание_строка: "ытую",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ыто$").unwrap(),
   окончание_строка: "ыто",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытое$").unwrap(),
   окончание_строка: "ытое",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытом$").unwrap(),
   окончание_строка: "ытом",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытому$").unwrap(),
   окончание_строка: "ытому",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытой$").unwrap(),
   окончание_строка: "ытой",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытого$").unwrap(),
   окончание_строка: "ытого",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытость$").unwrap(),
   окончание_строка: "ытость",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытостью$").unwrap(),
   окончание_строка: "ытостью",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытости$").unwrap(),
   окончание_строка: "ытости",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытостей$").unwrap(),
   окончание_строка: "ытостей",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытостям$").unwrap(),
   окончание_строка: "ытостям",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытостями$").unwrap(),
   окончание_строка: "ытостями",
   вид_окончания: Вид_Окончания::Ыт,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытостях$").unwrap(),
   окончание_строка: "ытостях",
   вид_окончания: Вид_Окончания::Ыт,
},
 RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)ыты$").unwrap(),
    окончание_строка: "ыты",
    вид_окончания: Вид_Окончания::_Ы,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытые$").unwrap(),
   окончание_строка: "ытые",
   вид_окончания: Вид_Окончания::_Ы,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытым$").unwrap(),
   окончание_строка: "ытым",
   вид_окончания: Вид_Окончания::_Ы,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытыми$").unwrap(),
   окончание_строка: "ытыми",
   вид_окончания: Вид_Окончания::_Ы,
},
RE_исключающее_окончание {
   re_образец: Regex::new(r"(?i)ытых$").unwrap(),
   окончание_строка: "ытых",
   вид_окончания: Вид_Окончания::_Ы,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)ытый$").unwrap(),
    окончание_строка: "ытый",
    вид_окончания: Вид_Окончания::_Ы,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)ытые$").unwrap(),
    окончание_строка: "ытые",
    вид_окончания: Вид_Окончания::_Ы,
},
RE_исключающее_окончание {
    re_образец: Regex::new(r"(?i)ытее$").unwrap(),
    окончание_строка: "ытее",
    вид_окончания: Вид_Окончания::_Ы,
},
]
});*/
//
/*pub static RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ: LazyLock<
    [RE_исключающее_окончание; 306],
> = LazyLock::new(|| {
    [
    //----------------В
    //ем
           //----------------
         RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)в$").unwrap(),
               окончание_строка: "в",
               вид_окончания: Вид_Окончания::Ва_Во,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)ве$").unwrap(),
               окончание_строка: "ве",
               вид_окончания: Вид_Окончания::Ва_Во,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)ва$").unwrap(),
               окончание_строка: "ва",
               вид_окончания: Вид_Окончания::Ва_Во,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вая$").unwrap(),
               окончание_строка: "вая",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вую$").unwrap(),
               окончание_строка: "вую",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)во$").unwrap(),
               окончание_строка: "во",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вы$").unwrap(),
               окончание_строка: "вы",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вом$").unwrap(),
               окончание_строка: "вом",
               вид_окончания: Вид_Окончания::Ва_Во,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вому$").unwrap(),
               окончание_строка: "вому",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)во$").unwrap(),
               окончание_строка: "во",
               вид_окончания: Вид_Окончания::Ва_Во,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вого$").unwrap(),
               окончание_строка: "вого",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вое$").unwrap(),
               окончание_строка: "вое",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вой$").unwrap(),
               окончание_строка: "вой",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вость$").unwrap(),
               окончание_строка: "вость",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)востью$").unwrap(),
               окончание_строка: "востью",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вости$").unwrap(),
               окончание_строка: "вости",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)востей$").unwrap(),
               окончание_строка: "востей",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)востям$").unwrap(),
               окончание_строка: "востям",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)востями$").unwrap(),
               окончание_строка: "востями",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)востях$").unwrap(),
               окончание_строка: "востях",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вым$").unwrap(),
               окончание_строка: "вым",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)выми$").unwrap(),
               окончание_строка: "выми",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вый$").unwrap(),
               окончание_строка: "вый",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вые$").unwrap(),
               окончание_строка: "вые",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вых$").unwrap(),
               окончание_строка: "вых",
               вид_окончания: Вид_Окончания::_В,
           },
           RE_исключающее_окончание {
               re_образец: Regex::new(r"(?i)вее$").unwrap(),
               окончание_строка: "вее",
               вид_окончания: Вид_Окончания::_В,
           },
           //--------------------
    //---------------Щ
    //
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щ$").unwrap(),
              окончание_строка: "щ",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)ща$").unwrap(),
              окончание_строка: "ща",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щая$").unwrap(),
              окончание_строка: "щая",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щую$").unwrap(),
              окончание_строка: "щую",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)ще$").unwrap(),
              окончание_строка: "ще",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щи$").unwrap(),
              окончание_строка: "щи",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щем$").unwrap(),
              окончание_строка: "щем",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щему$").unwrap(),
              окончание_строка: "щему",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щее$").unwrap(),
              окончание_строка: "щее",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щего$").unwrap(),
              окончание_строка: "щего",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щее$").unwrap(),
              окончание_строка: "щее",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щей$").unwrap(),
              окончание_строка: "щей",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щесть$").unwrap(),
              окончание_строка: "щесть",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щестью$").unwrap(),
              окончание_строка: "щестью",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щести$").unwrap(),
              окончание_строка: "щести",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щестей$").unwrap(),
              окончание_строка: "щестей",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щестям$").unwrap(),
              окончание_строка: "щестям",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щестями$").unwrap(),
              окончание_строка: "щестями",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щестях$").unwrap(),
              окончание_строка: "щестях",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щим$").unwrap(),
              окончание_строка: "щим",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щими$").unwrap(),
              окончание_строка: "щими",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щий$").unwrap(),
              окончание_строка: "щий",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щие$").unwrap(),
              окончание_строка: "щие",
              вид_окончания: Вид_Окончания::_Щ,
          },
          RE_исключающее_окончание {
              re_образец: Regex::new(r"(?i)щих$").unwrap(),
              окончание_строка: "щих",
              вид_окончания: Вид_Окончания::_Щ,
          },
          //-------------

        //----------------Л
        //Л
       /* RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)л$").unwrap(),
            окончание_строка: "л",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)лся$").unwrap(),
            окончание_строка: "лся",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)ло$").unwrap(),
            окончание_строка: "ло",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)лось$").unwrap(),
            окончание_строка: "лось",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)ли$").unwrap(),
            окончание_строка: "ли",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)лись$").unwrap(),
            окончание_строка: "лись",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)ль$").unwrap(),
            окончание_строка: "ль",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)ль$").unwrap(),
            окончание_строка: "ль",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)ль$").unwrap(),
            окончание_строка: "ль",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)лью$").unwrap(),
            окончание_строка: "лью",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)лей$").unwrap(),
            окончание_строка: "лей",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)ли$").unwrap(),
            окончание_строка: "ли",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)лям$").unwrap(),
            окончание_строка: "лям",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)лями$").unwrap(),
            окончание_строка: "лями",
            вид_окончания: Вид_Окончания::_Л,
        },
        RE_исключающее_окончание {
            re_образец: Regex::new(r"(?i)лях$").unwrap(),
            окончание_строка: "лях",
            вид_окончания: Вид_Окончания::_Л,
        },*/

        //-------------

        //-------------------------------------ён






        //--------------------

        //
    ]
});*/

//
pub static ПУСТОЙ_РЯД_RE: LazyLock<[Regex; 0]> = LazyLock::new(|| Default::default());
//
/*pub static RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ЕН: LazyLock<[Regex; 42]> = LazyLock::new(|| {
    [
        //ен
        //а
        Regex::new(r"(?i)еная$").unwrap(),
        //одиночн
        Regex::new(r"(?i)енн$").unwrap(),
        Regex::new(r"(?i)енна$").unwrap(),
        Regex::new(r"(?i)енно$").unwrap(),
        Regex::new(r"(?i)енны$").unwrap(),
        //Regex::new(r"(?i)еннее$").unwrap(),
        //ую
        Regex::new(r"(?i)еную$").unwrap(),
        //о
        Regex::new(r"(?i)еное$").unwrap(),
        Regex::new(r"(?i)еного$").unwrap(),
        //глаголы
        Regex::new(r"(?i)еном$").unwrap(),
        Regex::new(r"(?i)еному$").unwrap(),
        Regex::new(r"(?i)еность$").unwrap(),
        Regex::new(r"(?i)еностью$").unwrap(),
        Regex::new(r"(?i)ености$").unwrap(),
        Regex::new(r"(?i)еностей$").unwrap(),
        Regex::new(r"(?i)еностям$").unwrap(),
        Regex::new(r"(?i)еностями$").unwrap(),
        Regex::new(r"(?i)еностях$").unwrap(),
        //ы
        Regex::new(r"(?i)еным$").unwrap(),
        Regex::new(r"(?i)еными$").unwrap(),
        Regex::new(r"(?i)еных$").unwrap(),
        Regex::new(r"(?i)еные$").unwrap(),
        //----------------------------------);
        //ён
        //а
        Regex::new(r"(?i)ёная$").unwrap(),
        //одиночн
        Regex::new(r"(?i)ённ$").unwrap(),
        Regex::new(r"(?i)ённа$").unwrap(),
        Regex::new(r"(?i)ённо$").unwrap(),
        Regex::new(r"(?i)ённы$").unwrap(),
        //ую
        Regex::new(r"(?i)ёную$").unwrap(),
        //о
        Regex::new(r"(?i)ёное$").unwrap(),
        Regex::new(r"(?i)ёного$").unwrap(),
        //глаголы
        Regex::new(r"(?i)ёном$").unwrap(),
        Regex::new(r"(?i)ёному$").unwrap(),
        Regex::new(r"(?i)ёность$").unwrap(),
        Regex::new(r"(?i)ёностью$").unwrap(),
        Regex::new(r"(?i)ёности$").unwrap(),
        Regex::new(r"(?i)ёностей$").unwrap(),
        Regex::new(r"(?i)ёностям$").unwrap(),
        Regex::new(r"(?i)ёностями$").unwrap(),
        Regex::new(r"(?i)ёностях$").unwrap(),
        //ы
        Regex::new(r"(?i)ёным$").unwrap(),
        Regex::new(r"(?i)ёными$").unwrap(),
        Regex::new(r"(?i)ёных$").unwrap(),
        Regex::new(r"(?i)ёные$").unwrap(),
        //----------------------------------);
        //
    ]
});*/
//
/*pub static RE_НАЧАЛЬНЫЕ_ОКОНЧАНИЯ_ВАН: LazyLock<[Re_образцы; 23]> = LazyLock::new(|| {
    [
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ван)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ван)$").unwrap(),
                окончание_строка: "ван",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(вана)$").unwrap(),
                re_замена: Regex::new(r"(?i)(вана)$").unwrap(),
                окончание_строка: "вана",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(вано)$").unwrap(),
                re_замена: Regex::new(r"(?i)(вано)$").unwrap(),
                окончание_строка: "вано",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ваны)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ваны)$").unwrap(),
                окончание_строка: "ваны",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванная)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванная)$").unwrap(),
                окончание_строка: "ванная",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванную)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванную)$").unwrap(),
                окончание_строка: "ванную",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванной)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванной)$").unwrap(),
                окончание_строка: "ванной",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванное)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванное)$").unwrap(),
                окончание_строка: "ванное",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванного)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванного)$").unwrap(),
                окончание_строка: "ванного",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванном)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванном)$").unwrap(),
                окончание_строка: "ванном",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванному)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванному)$").unwrap(),
                окончание_строка: "ванному",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванность)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванность)$").unwrap(),
                окончание_строка: "ванность",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванностью)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванностью)$").unwrap(),
                окончание_строка: "ванностью",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванности)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванности)$").unwrap(),
                окончание_строка: "ванности",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванностей)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванностей)$").unwrap(),
                окончание_строка: "ванностей",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванностям)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванностям)$").unwrap(),
                окончание_строка: "ванностям",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванностями)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванностями)$").unwrap(),
                окончание_строка: "ванностями",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванностях)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванностях)$").unwrap(),
                окончание_строка: "ванностях",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванным)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванным)$").unwrap(),
                окончание_строка: "ванным",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванными)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванными)$").unwrap(),
                окончание_строка: "ванными",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванных)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванных)$").unwrap(),
                окончание_строка: "ванных",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванные)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванные)$").unwrap(),
                окончание_строка: "ванные",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
        Re_образцы {
            содержимое: RE_Окончания_Поиск {
                re_поиска: Regex::new(r"(?i)\w+(ванный)$").unwrap(),
                re_замена: Regex::new(r"(?i)(ванный)$").unwrap(),
                окончание_строка: "ванный",
                вид_окончания: Вид_Окончания::Ван,
            },
            исключения_первой_очереди:
                &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ПЕРВОЙ_ОЧЕРЕДИ,
                исключения_второй_очереди:
                    &*RE_ОКОНЧАНИЯ_В_ЗАМЕНАХ_ИСКЛЮЧЕНИЯ_ВТОРОЙ_ОЧЕРЕДИ,
        },
    ]
});*/
//
/*pub const КОНЕЧНЫЕ_ОКОНЧАНИЯ_ВАН: [&'static str; 24] = [
    "ван",
    "вана",
    "вано",
    "ваны",
    "ваннее",
    //
    "ванная",
    "ванную",
    //о
    "ванной",
    "ванное",
    "ванного",
    //глаголы
    "ванном",
    "ванному",
    //ст
    "ванность",
    "ванностью",
    "ванности",
    "ванностей",
    "ванностям",
    "ванностями",
    "ванностях",
    //ы
    "ванными",
    "ванным",
    "ванных",
    "ванные",
    "ванный",
];*/
//
pub static ТОРЖОК_ОКОНЧАНИЙ: LazyLock<[Окончаний_Торжка; 25]> = LazyLock::new(|| {
    [
        //
        Окончаний_Торжка {
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ван", "ен", "ён", "он", "ан", "ит", "ем", "ыт", "н", "в",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "в",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(в)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ван",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ван)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ен",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(ен)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ем",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(ем)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ён",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ён)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "он",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(он)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ан",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(ан)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ит",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(ит)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ыт",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ыт)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                /*Вид_Окончания_со_строкой {
                    строка: "т",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(т)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "",
                    вид_окончания: Вид_Окончания::То,
                    re_выдер: Regex::new(r"(?i)()$").unwrap(),
                },*/
                Вид_Окончания_со_строкой {
                    строка: "н",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(н)$").unwrap(),
                },
            ],
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
                Text_Changer::Вид_Окончания::Авш,
            ]),
        },
        //вана
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
                Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "вана", "ена", "ема", "она", "ана", "на", "ита", "ыта", "та", "ва",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ва",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(ва)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "вана",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(вана)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ена",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(ена)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ема",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(ема)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ена",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ена)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "она",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(она)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ана",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(ана)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ита",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(ита)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ыта",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ыта)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "та",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(та)$").unwrap(),
                },
                /*Вид_Окончания_со_строкой {
                    строка: "та",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(та)$").unwrap(),
                },*/
                Вид_Окончания_со_строкой {
                    строка: "на",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(на)$").unwrap(),
                },
            ],
        },
        //
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
                //Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ваннее",
                "еннее",
                "емее",
                "ённее",
                "оннее",
                "аннее",
                "нее",
                "итее",
                "ытее",
                "тее",
                "щее",
                "авшее",
                "шее",
                "стее",
                "лее",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "авшее",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "щее",
                    вид_окончания: Вид_Окончания::_Щ,
                    re_выдер: Regex::new(r"(?i)(щее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ваннее",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ваннее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "еннее",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(еннее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённее",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(ённее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емее",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "оннее",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(оннее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "аннее",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(аннее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итее",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытее",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "стее",
                    вид_окончания: Вид_Окончания::Ст,
                    re_выдер: Regex::new(r"(?i)(стее)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "лее",
                    вид_окончания: Вид_Окончания::_Л,
                    re_выдер: Regex::new(r"(?i)(лее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "шее",
                    вид_окончания: Вид_Окончания::_Ш,
                    re_выдер: Regex::new(r"(?i)(шее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тее",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тее)$").unwrap(),
                },
                /*Вид_Окончания_со_строкой {
                    строка: "тее",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тее)$").unwrap(),
                },*/
                Вид_Окончания_со_строкой {
                    строка: "нее",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(нее)$").unwrap(),
                },
            ],
        },
        //
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
                Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "вано", "ено", "емо", "оно", "ано", "но", "ито", "ыто", "то",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "вано",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(вано)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ено",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(ено)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емо",
                    вид_окончания: Вид_Окончания::Ем,

                    re_выдер: Regex::new(r"(?i)(емо)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ено",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ено)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "оно",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(оно)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ано",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(ано)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ито",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(ито)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ыто",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ыто)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "то",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(то)$").unwrap(),
                },
                /*Вид_Окончания_со_строкой {
                    строка: "то",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(то)$").unwrap(),
                },*/
                Вид_Окончания_со_строкой {
                    строка: "но",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(но)$").unwrap(),
                },
            ],
        },
        //оны
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
                Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ваны", "ены", "емы", "оны", "аны", "ны", "иты", "ыты", "те", "ва",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ва",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(ва)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ваны",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ваны)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ены",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(ены)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ены",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ены)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емы",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емы)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "оны",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(оны)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "аны",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(аны)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "иты",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(иты)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ыты",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ыты)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "те",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(те)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ты",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(ты)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ны",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ны)$").unwrap(),
                },
            ],
        },
        //ванный
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
               // Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванный",
                "енный",
                "емый",
                "ённый",
                "онный",
                "анный",
                "ный",
                "итый",
                "ытый",
                "авший",
                "той",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванный",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванный)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "енный",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енный)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емый",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённый",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онный",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онный)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анный",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анный)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итый",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытый",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авший",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авший)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "той",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(той)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "тый",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ный",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ный)$").unwrap(),
                },
            ],
        },
        //ванных
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
              //  Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванных",
                "венных",
                "енных",
                "емых",
                "ённых",
                "онных",
                "анных",
                "ных",
                "итых",
                "авших",
                "ытых",
                "тех",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванных",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванных)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емых",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емых)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онных",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онных)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анных",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анных)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итых",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итых)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытых",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытых)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авших",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авших)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тех",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тех)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венных",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венных)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ённых",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённых)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "енных",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енных)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тых",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тых)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ных",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ных)$").unwrap(),
                },
            ],
        },
        //ванным
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
               // Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванным",
                "венным",
                "енным",
                "емым",
                "ённым",
                "онным",
                "анным",
                "ным",
                "итым",
                "авшим",
                "ытым",
                "том",
                "тым",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванным",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванным)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емым",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емым)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онным",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онным)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анным",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анным)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ным",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ным)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итым",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итым)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшим",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшим)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытым",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытым)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "том",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(том)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венным",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венным)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енным",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енным)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённым",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённым)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тым",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тым)$").unwrap(),
                },
            ],
        },
        //ванными
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
               // Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванными",
                //"венными",
                "енными",
                "емыми",
                "ёнными",
                "онными",
                "анными",
                "ными",
                "итыми",
                "авшими",
                "ытыми",
                "тыми",
                //"венными",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванными",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванными)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ёнными",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ёнными)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онными",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онными)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анными",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анными)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итыми",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итыми)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшими",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшими)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытыми",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытыми)$").unwrap(),
                },
                /*Вид_Окончания_со_строкой {
                    строка: "венными",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венными)$").unwrap(),
                },*/
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енными",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енными)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емыми",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емыми)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тыми",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тыми)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тыми",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тыми)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ными",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ными)$").unwrap(),
                },
            ],
        },
        //ванный
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
               // Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванный",
                "енный",
                "венный",
                "емый",
                "ённый",
                "онный",
                "анный",
                "ный",
                "итый",
                "авший",
                "ытый",
                "тый",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванный",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванный)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емый",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онный",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онный)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анный",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анный)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итый",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авший",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авший)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытый",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венный",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венный)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енный",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енный)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённый",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тый",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тый)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ный",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ный)$").unwrap(),
                },
            ],
        },
        //ванная
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
              //  Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванная",
                "енная",
                "емая",
                "ённая",
                "онная",
                "анная",
                "ная",
                "итая",
                "авшая",
                "ытая",
                "тая",
                "венная",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванная",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванная)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венная",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венная)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емая",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емая)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онная",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онная)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анная",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анная)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итая",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итая)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшая",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшая)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытая",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытая)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енная",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енная)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённая",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённая)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тая",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тая)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тая",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тая)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ная",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ная)$").unwrap(),
                },
            ],
        },
        //ванную
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
              //  Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванную",
                "енную",
                "емую",
                "ённую",
                "онную",
                "анную",
                "ную",
                "итую",
                "авшую",
                "ытую",
                "тую",
                "венную",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "венную",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венную)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ванную",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванную)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емую",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емую)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онную",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онную)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анную",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анную)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итую",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итую)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшую",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшую)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытую",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытую)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енную",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енную)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённую",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённую)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тую",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тую)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тую",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тую)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ную",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ную)$").unwrap(),
                },
            ],
        },
        //ванном
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
               // Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванном",
                "венном",
                "енном",
                "емом",
                "ённом",
                "онном",
                "анном",
                "ном",
                "итом",
                "авшем",
                "ытом",
                "том",
                "тым",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванном",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванном)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емом",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емом)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онном",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онном)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анном",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анном)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итом",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итом)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшем",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшем)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытом",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытом)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венном",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венном)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енном",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енном)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённом",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённом)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "том",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(том)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тым",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тым)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ном",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ном)$").unwrap(),
                },
            ],
        },
        //ванному
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
               // Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванному",
                "венному",
                "енному",
                "емому",
                "ённому",
                "онному",
                "анному",
                "ному",
                "итому",
                "авшему",
                "ытому",
                "тому",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванному",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванному)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емому",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емому)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онному",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онному)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анному",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анному)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итому",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итому)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшему",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшему)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытому",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытому)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венному",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венному)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енному",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енному)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённому",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённому)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тому",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тому)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тому",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тому)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ному",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ному)$").unwrap(),
                },
            ],
        },
        //ванного
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
               // Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванного",
                "венного",
                "енного",
                "емого",
                "ённого",
                "онного",
                "анного",
                "ного",
                "итого",
                "авшего",
                "ытого",
                "того",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванного",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванного)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емого",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емого)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онного",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онного)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анного",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анного)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итого",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итого)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшего",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшего)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытого",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытого)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венного",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венного)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енного",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енного)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённого",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённого)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "того",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(того)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "того",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(того)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ного",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ного)$").unwrap(),
                },
            ],
        },
        //ванное
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
               // Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванное",
                "венное",
                "енное",
                "емое",
                "ённое",
                "онное",
                "анное",
                "ное",
                "итое",
                "авшее",
                "ытое",
                "тое",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванное",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванное)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емое",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емое)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онное",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онное)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анное",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анное)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итое",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итое)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшее",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшее)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытое",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытое)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венное",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венное)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енное",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енное)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённое",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённое)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тое",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тое)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тое",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тое)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ное",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ное)$").unwrap(),
                },
            ],
        },
        //ванные
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
              //  Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванные",
                "венные",
                "енные",
                "емые",
                "ённые",
                "онные",
                "анные",
                "ные",
                "итые",
                "авшие",
                "ытые",
                "тые",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванные",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванные)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емые",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емые)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онные",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онные)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анные",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анные)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итые",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итые)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшие",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшие)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытые",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытые)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венные",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венные)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енные",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енные)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённые",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённые)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ные",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ные)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тые",
                    вид_окончания: Вид_Окончания::Ты,
                    re_выдер: Regex::new(r"(?i)(тые)$").unwrap(),
                },
            ],
        },
        //ванной
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::from_iter([
              //  Text_Changer::Вид_Окончания::Авш,
            ]),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванной",
                "венной",
                "енной",
                "емой",
                "ённой",
                "онной",
                "анной",
                "ной",
                "итой",
                "авшей",
                "ытой",
                "той",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "венной",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венной)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ванной",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванной)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емой",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емой)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онной",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онной)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анной",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анной)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итой",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итой)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшей",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытой",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытой)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енной",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енной)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённой",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённой)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "той",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(той)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "той",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(той)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ной",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ной)$").unwrap(),
                },
            ],
        },
        //ванность
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::default(),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванность",
                "венность",
                "енность",
                "емость",
                "ённость",
                "онность",
                "анность",
                "ность",
                "итость",
                "авшесть",
                "ытость",
                "тость",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванность",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванность)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емость",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емость)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онность",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онность)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анность",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анность)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итость",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итость)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшесть",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшесть)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытость",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытость)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венность",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венность)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енность",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енность)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённость",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённость)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тость",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тость)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тость",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тость)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ность",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ность)$").unwrap(),
                },
            ],
        },
        //ванностью
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::default(),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванностью",
                "венностью",
                "енностью",
                "емостью",
                "ённостью",
                "онностью",
                "анностью",
                "ностью",
                "итостью",
                "авшестью",
                "ытостью",
                "тостью",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванностью",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванностью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емостью",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емостью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онностью",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онностью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анностью",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анностью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итостью",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итостью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшестью",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшестью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытостью",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытостью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венностью",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венностью)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енностью",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енностью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённостью",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённостью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостью",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостью",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостью)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ностью",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ностью)$").unwrap(),
                },
            ],
        },
        //ванности
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::default(),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванности",
                "емости",
                "венности",
                "енности",
                "ённости",
                "онности",
                "анности",
                "ности",
                "итости",
                "авшести",
                "ытости",
                "тости",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванности",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванности)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емости",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емости)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онности",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онности)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анности",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анности)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итости",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итости)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшести",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшести)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытости",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытости)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венности",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венности)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енности",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енности)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённости",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённости)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тости",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тости)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тости",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тости)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ности",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ности)$").unwrap(),
                },
            ],
        },
        //ванностей
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::default(),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванностей",
                "венностей",
                "енностей",
                "емостей",
                "ённостей",
                "онностей",
                "анностей",
                "ностей",
                "итостей",
                "авшестей",
                "ытостей",
                "тостей",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванностей",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванностей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емостей",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емостей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онностей",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онностей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анностей",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анностей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итостей",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итостей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшестей",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшестей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытостей",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытостей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венностей",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венностей)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енностей",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енностей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённостей",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённостей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостей",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостей",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостей)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ностей",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ностей)$").unwrap(),
                },
            ],
        },
        //ванностям
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::default(),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванностям",
                "венностям",
                "енностям",
                "емостям",
                "ённостям",
                "онностям",
                "анностям",
                "ностям",
                "итостям",
                "авшестям",
                "ытостям",
                "тостям",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванностям",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванностям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емостям",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емостям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онностям",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онностям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анностям",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анностям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итостям",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итостям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшестям",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшестям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытостям",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытостям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венностям",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венностям)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енностям",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енностям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённостям",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённостям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостям",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостям",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостям)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ностям",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ностям)$").unwrap(),
                },
            ],
        },
        //ванностями
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::default(),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванностями",
                "енностями",
                "венностями",
                "емостями",
                "ённостями",
                "онностями",
                "анностями",
                "ностями",
                "итостями",
                "авшестями",
                "ытостями",
                "тостями",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванностями",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванностями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емостями",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емостями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онностями",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онностями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анностями",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анностями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итостями",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итостями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшестями",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшестями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытостями",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытостями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венностями",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венностями)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енностями",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енностями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённостями",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённостями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостями",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостями",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостями)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ностями",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ностями)$").unwrap(),
                },
            ],
        },
        //ванностях
        Окончаний_Торжка {
            окончания_исключения: rapidhash::fast::RapidHashSet::default(),
            куча_окончаний: rapidhash::fast::RapidHashSet::from_iter([
                "ванностях",
                "венностях",
                "енностях",
                "емостях",
                "ённостях",
                "онностях",
                "анностях",
                "ностях",
                "итостях",
                "авшестях",
                "ытостях",
                "тостях",
            ]),
            первая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "ванностях",
                    вид_окончания: Вид_Окончания::Ван,
                    re_выдер: Regex::new(r"(?i)(ванностях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "емостях",
                    вид_окончания: Вид_Окончания::Ем,
                    re_выдер: Regex::new(r"(?i)(емостях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "венностях",
                    вид_окончания: Вид_Окончания::Ва_Во,
                    re_выдер: Regex::new(r"(?i)(венностях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "онностях",
                    вид_окончания: Вид_Окончания::Он,
                    re_выдер: Regex::new(r"(?i)(онностях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "анностях",
                    вид_окончания: Вид_Окончания::Ан,
                    re_выдер: Regex::new(r"(?i)(анностях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "итостях",
                    вид_окончания: Вид_Окончания::Ит,
                    re_выдер: Regex::new(r"(?i)(итостях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "авшестях",
                    вид_окончания: Вид_Окончания::Авш,
                    re_выдер: Regex::new(r"(?i)(авшестях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ытостях",
                    вид_окончания: Вид_Окончания::Ыт,
                    re_выдер: Regex::new(r"(?i)(ытостях)$").unwrap(),
                },
            ],
            вторая_очередь: vec![
                Вид_Окончания_со_строкой {
                    строка: "енностях",
                    вид_окончания: Вид_Окончания::Ен,
                    re_выдер: Regex::new(r"(?i)(енностях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ённостях",
                    вид_окончания: Вид_Окончания::Ён,
                    re_выдер: Regex::new(r"(?i)(ённостях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостях",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "тостях",
                    вид_окончания: Вид_Окончания::Та_то,
                    re_выдер: Regex::new(r"(?i)(тостях)$").unwrap(),
                },
                Вид_Окончания_со_строкой {
                    строка: "ностях",
                    вид_окончания: Вид_Окончания::_Н,
                    re_выдер: Regex::new(r"(?i)(ностях)$").unwrap(),
                },
            ],
        },
    ]
});
#[allow(non_camel_case_types)]
pub trait Возможности_торжка {
    fn найти_окончание_выдера(
        &self,
        исключающее_окончание: &RE_полное_окончание,
        изначальное_окончание: &RE_полное_окончание,
    ) -> Result<&Вид_Окончания_со_строкой, Text_Changer::Вид_ошибки_при_поиске_выдера_в_торжке>;
    //
    fn найти_окончание_к_слову_замене(
        &self,
        искомое_окончание: &str,
        ячейка_выдера: &Вид_Окончания_со_строкой,
    ) -> Result<&Вид_Окончания_со_строкой, Text_Changer::Вид_ошибки_при_поиске_в_торжке>;
    //
}
impl Возможности_торжка for [Окончаний_Торжка] {
    /*fn проверить_на_соблюдение_правил(&self) -> Result<(), ()> {
        //прогон
        for раздел in self.iter() {}
    }*/
    //
    fn найти_окончание_к_слову_замене(
        &self,
        искомое_окончание: &str,
        ячейка_выдера: &Вид_Окончания_со_строкой,
    ) -> Result<&Вид_Окончания_со_строкой, Вид_ошибки_при_поиске_в_торжке> {
        //let mut найден_в_куче_окончаий: bool = false;
        //перебор внутри
        for раздел in self.iter() {
            //если содержит окончание  в куче
            if раздел.куча_окончаний.contains(искомое_окончание)
            {
                //    найден_в_куче_окончаий = true;
                //если исключение
                if раздел
                    .окончания_исключения
                    .contains(&ячейка_выдера.вид_окончания)
                {
                    return Err(Вид_ошибки_при_поиске_в_торжке::Исключение);
                }
                //перебор ячеек в разделе на предмет соответствия вида окончания
                for ячейка in раздел.первая_очередь.iter() {
                    if ячейка.вид_окончания == ячейка_выдера.вид_окончания
                    {
                        return Ok(&ячейка);
                    }
                }
                //перебор ячеек в разделе на предмет соответствия вида окончания
                for ячейка in раздел.вторая_очередь.iter() {
                    if ячейка.вид_окончания == ячейка_выдера.вид_окончания
                    {
                        return Ok(&ячейка);
                    }
                }
            }
        }
        //перебор внутри
        for раздел in self.iter() {
            //если содержит окончание  в куче
            /*println!(
                "найти_окончание_к_слову_замене -ищем окончание |{}|  в разделе |{:?}|",
                искомое_окончание,
                // искомое_окончание.as_bytes(),
                раздел.куча_окончаний,
            );*/
            /*for ячейка in раздел.куча_окончаний.iter() {
                println!("окончание |{}| побайтово |{:?}|", ячейка, ячейка.as_bytes());
            }*/
            if раздел.куча_окончаний.contains(искомое_окончание)
            {
                //
                println!(
                    "{}{}{}{}",
                    style(format!("найти_окончание_к_слову_замене - окончание ",)),
                    style(format!("|{}|", искомое_окончание)).cyan(),
                    style(format!(" найдено в разделе ",)),
                    style(format!("|{:?}|", раздел.куча_окончаний)).blue(),
                    /*"найти_окончание_к_слову_замене - окончание |{}| найдено в разделе |{:?}|",
                    искомое_окончание, раздел.куча_окончаний*/
                );
                let mut строка: String = String::new();
                //перебор ячеек в разделе на предмет соответствия вида окончания
                for ячейка in раздел.первая_очередь.iter() {
                    if ячейка.вид_окончания == ячейка_выдера.вид_окончания
                    {
                        return Ok(&ячейка);
                    }
                    строка = format!("{}+{}", строка, ячейка.вид_окончания.to_string());
                }
                //перебор ячеек в разделе на предмет соответствия вида окончания
                for ячейка in раздел.вторая_очередь.iter() {
                    if ячейка.вид_окончания == ячейка_выдера.вид_окончания
                    {
                        return Ok(&ячейка);
                    }
                    строка = format!("{}+{}", строка, ячейка.вид_окончания.to_string());
                }
                //
                println!(
                    "{}{}{}{}",
                    style(format!("не нашло  вид исходного окончания ",)),
                    style(format!("|{}|", ячейка_выдера.вид_окончания)).cyan(),
                    style(format!(" в ряде ",)),
                    style(format!("|{}|", строка)).blue(),
                    /*"не нашло  вид исходного окончания |{}| в ряде |{}|",
                    ячейка_выдера.вид_окончания,
                    строка*/
                );
                return Err(
                    Text_Changer::Вид_ошибки_при_поиске_в_торжке::Ошибка(format!(
                        "{}{}",
                        style(format!(
                            "найти_окончание_к_слову_замене - не нашло окончание для ",
                        )),
                        style(format!("|{}|", искомое_окончание)).cyan(),
                    )),
                );
            }
        }
        Err(
            Text_Changer::Вид_ошибки_при_поиске_в_торжке::Ошибка(
                format!(
                    "{}{}",
                    style(format!(
                        "найти_окончание_к_слову_замене - не нашло окончание для ",
                    )),
                    style(format!("|{}|", искомое_окончание)).cyan(),
                ),
            ),
        )
        //
    }
    //
    fn найти_окончание_выдера(
        &self,
        исключающее_окончание: &RE_полное_окончание,
        изначальное_окончание: &RE_полное_окончание,
    ) -> Result<&Вид_Окончания_со_строкой, Text_Changer::Вид_ошибки_при_поиске_выдера_в_торжке>
    {
        let mut найден_в_куче_окончаий: bool = false;
        //перебор внутри
        for раздел in self.iter() {
            //если содержит окончание  в куче
            if раздел
                .куча_окончаний
                .contains(&изначальное_окончание.окончание_строка)
            {
                найден_в_куче_окончаий = true;
                //перебор ячеек в разделе на предмет соответствия вида окончания
                for ячейка in раздел.первая_очередь.iter() {
                    if ячейка.вид_окончания == исключающее_окончание.вид_окончания
                    {
                        return Ok(&ячейка);
                    }
                }
                //
                for ячейка in раздел.вторая_очередь.iter() {
                    if ячейка.вид_окончания == исключающее_окончание.вид_окончания
                    {
                        return Ok(&ячейка);
                    }
                }
            }
        }
        //перебор внутри
        /*for раздел in self.iter() {
            /*println!(
                "найти_окончание_выдера - ищем окончание |{}| в разделе |{:?}|",
                изначальное_окончание.окончание_строка, раздел.куча_окончаний,
            );*/
            //если содержит окончание  в куче
            if раздел
                .куча_окончаний
                .contains(&изначальное_окончание.окончание_строка)
            {
                //
                /*println!(
                    "найти_окончание_выдера - окончание |{}| найдено в разделе |{:?}|",
                    изначальное_окончание.окончание_строка, раздел.куча_окончаний
                );*/
                //перебор ячеек в разделе на предмет соответствия вида окончания
                for ячейка in раздел.первая_очередь.iter() {
                    if ячейка.вид_окончания == исключающее_окончание.вид_окончания
                    {
                        return Ok(&ячейка);
                    }
                }
                //перебор ячеек в разделе на предмет соответствия вида окончания
                for ячейка in раздел.вторая_очередь.iter() {
                    if ячейка.вид_окончания == исключающее_окончание.вид_окончания
                    {
                        return Ok(&ячейка);
                    }
                }
            }
        }*/
        return match найден_в_куче_окончаий{
            true=>  Err(Text_Changer::Вид_ошибки_при_поиске_выдера_в_торжке::Не_найдено_окончание_в_разделе) ,
            false=> Err( Text_Changer::Вид_ошибки_при_поиске_выдера_в_торжке::Не_найдено_окончание_в_разделе)
        };
        /* Err(format!(
            "Найти окончание выдера - не нашло окончание для |{}|",
            изначальное_окончание.re_поиска
        ))*/
    }
}

// счётчик_входа.fetch_add(1, Ordering::Relaxed);

/*pub fn проверка_конечных_окончаний_re_образцов() {
    static СЧЁТЧИК_ПРОВЕРКИ: AtomicBool = AtomicBool::new(false);
    if !СЧЁТЧИК_ПРОВЕРКИ.swap(true, Ordering::SeqCst) {
        проверка_среза_строк_на_неповтор(
            &КОНЕЧНЫЕ_ОКОНЧАНИЯ_ВАН,
            "КОНЕЧНЫЕ_ОКОНЧАНИЯ_ВАН",
        );
    }

    // счётчик_входа.fetch_add(1, Ordering::Relaxed);
}*/
pub fn проверка_среза_строк_на_неповтор(
    срез: &[&'static str],
    имя_кучи: &str,
) {
    let mut куча: rapidhash::fast::RapidHashSet<&str> = rapidhash::fast::RapidHashSet::default();
    //сама проверка
    for ячейка in срез.iter() {
        if !куча.contains(ячейка) {
            куча.insert(ячейка);
        } else {
            println!("Куча |{}| повторно содержит образец |{}|", имя_кучи, ячейка)
        }
    }
}
//
pub static МЕСТОИМЕНИЯ: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        // --- Личные (я, ты, мы, вы) ---
        "я",
        "меня",
        "мне",
        "мной",
        "мною",
        "обо мне",
        "ты",
        "тебя",
        "тебе",
        "тобой",
        "тобою",
        "о тебе",
        "мы",
        "нас",
        "нам",
        "нами",
        "о нас",
        "вы",
        "вас",
        "вам",
        "вами",
        "о вас",
        // --- Личные 3-го лица (он, она, оно, они) ---
        "он",
        "его",
        "ему",
        "им",
        "нем",
        "о нем",
        "она",
        "ее",
        "ей",
        "ею",
        "ней",
        "о ней",
        "оно",
        "его",
        "ему",
        "им",
        "нем",
        "о нем",
        "они",
        "их",
        "им",
        "ими",
        "них",
        "о них",
        // --- Возвратное (себя) ---
        "себя",
        "себе",
        "собой",
        "собою",
        "о себе",
    ])
});
//
pub static ДОПЫ_РУССКОГО_ЯЗЫКА: LazyLock<RapidHashSet<&'static str>> =
    LazyLock::new(|| RapidHashSet::from_iter([]));
//
pub static НАРЕЧИЯ_РУССКОГО_ЯЗЫКА: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        // Возвратное (по-своему)
        "по-своему",
        // Личные (по-моему, по-твоему и т.д.)
        "по-моему",
        "по-твоему",
        "по-нашему",
        "по-вашему",
        // Местоимённые указательные
        "там",
        "тут",
        "сюда",
        "туда",
        "оттуда",
        "отсюда",
        "здесь",
        "так",
        "тогда",
        "потому",
        "поэтому",
        "затем",
        // Местоимённые определительные
        "всегда",
        "везде",
        "всюду",
        "повсюду",
        "по-иному",
        "по-всякому",
        // Вопросительно-относительные
        "как",
        "где",
        "куда",
        "откуда",
        "когда",
        "почему",
        "зачем",
        "отчего",
        // Неопределённые
        "как-то",
        "как-нибудь",
        "кое-как",
        "где-то",
        "где-либо",
        "где-нибудь",
        "кое-где",
        "когда-то",
        "когда-нибудь",
        "когда-либо",
        "зачем-то",
        "почему-то",
        // Отрицательные
        "никак",
        "нигде",
        "негде",
        "ниоткуда",
        "неоткуда",
        "никуда",
        "некуда",
        "никогда",
        "некогда",
        "незачем",
        // Определительные: качественные
        "быстро",
        "медленно",
        "весело",
        "громко",
        "хорошо",
        "дружно",
        "внимательно",
        "грубо",
        "молча",
        "правильно",
        "страшно",
        "чудовищно",
        "ярко",
        "смело",
        "горько",
        // Определительные: количественные
        "много",
        "мало",
        "приблизительно",
        "почти",
        "дважды",
        "вдвое",
        "втрое",
        "чуть-чуть",
        "очень",
        "весьма",
        "гораздо",
        "совершенно",
        "вдоволь",
        "дотла",
        "досыта",
        "крайне",
        "чрезмерно",
        "трижды",
        "вшестером",
        "пополам",
        "втройне",
        // Определительные: образа и способа действия
        "вручную",
        "вполголоса",
        "наизнанку",
        "шагом",
        "вплотную",
        "назубок",
        "искоса",
        "ощупью",
        "вдребезги",
        "кувырком",
        "крест-накрест",
        "басом",
        "вразвалку",
        "вплавь",
        "бегом",
        "верхом",
        "пешком",
        "плашмя",
        "вброд",
        "вприпрыжку",
        // Сравнительно-уподобительные
        "по-весеннему",
        "по-летнему",
        "по-осеннему",
        "по-зимнему",
        "по-детски",
        "по-дружески",
        "по-лисьи",
        "по-медвежьи",
        "по-отцовски",
        "по-матерински",
        "по-братски",
        // Совместности
        "вдвоём",
        "втроём",
        "впятером",
        "по двое",
        "по трое",
        "поочерёдно",
        // Обстоятельственные: места
        "далеко",
        "близко",
        "вблизи",
        "вдали",
        "вдалеке",
        "вдаль",
        "назад",
        "вперёд",
        "впереди",
        "позади",
        "сзади",
        "спереди",
        "слева",
        "справа",
        "налево",
        "направо",
        "вверху",
        "внизу",
        "наверху",
        "вниз",
        "вверх",
        "вбок",
        "набок",
        "вокруг",
        "рядом",
        "около",
        "возле",
        "издали",
        "издалека",
        "отовсюду",
        "оттуда",
        "изнутри",
        "снаружи",
        "поверх",
        "вглубь",
        "вширь",
        "навстречу",
        "посередине",
        "посреди",
        "вдалеке",
        "поблизости",
        "дома",
        "до́ма",
        // Обстоятельственные: времени
        "вчера",
        "сегодня",
        "завтра",
        "утром",
        "днём",
        "вечером",
        "ночью",
        "весной",
        "летом",
        "осенью",
        "зимой",
        "сейчас",
        "теперь",
        "тогда",
        "давно",
        "недавно",
        "издавна",
        "смолоду",
        "раньше",
        "прежде",
        "сперва",
        "сначала",
        "затем",
        "потом",
        "вовремя",
        "иногда",
        "всегда",
        "никогда",
        "допоздна",
        "затемно",
        "досветла",
        "долго",
        "недолго",
        "подолгу",
        "тотчас",
        "сразу",
        "наконец",
        "вскоре",
        "нескоро",
        "засветло",
        "на днях",
        "поутру",
        "вечор",
        "нонече",
        // Обстоятельственные: причины
        "сгоряча",
        "сдуру",
        "спьяну",
        "сослепу",
        "поневоле",
        "недаром",
        "почему",
        "потому",
        "оттого",
        "отчего",
        "волей-неволей",
        "неспроста",
        "спросонья",
        "со зла",
        "сослепу",
        // Обстоятельственные: цели
        "нарочно",
        "специально",
        "назло",
        "наперекор",
        "в шутку",
        "умышленно",
        "неумышленно",
        "в насмешку",
        "насмех",
        "невзначай",
        "напоказ",
        "зря",
        "бесцельно",
        "безрезультатно",
        // Производные от деепричастий
        "лёжа",
        "сидя",
        "стоя",
        "молча",
        "шутя",
        "нехотя",
        // Производные от наречий (с суффиксами)
        "рановато",
        "поздновато",
        "сыровато",
        "хорошенечко",
        "ранёхонько",
        "тихонько",
        "давненько",
        "поближе",
        "покрепче",
        "поменьше",
        "давным-давно",
        "полным-полно",
        // Непроизводные
        "вдруг",
        "еле-еле",
        "едва",
        "прежде",
        "весьма",
    ])
});

//
pub static ПРЕДЛОГИ_РУССКОГО_ЯЗЫКА: LazyLock<RapidHashSet<&'static str>> = LazyLock::new(|| {
    RapidHashSet::from_iter([
        // Непроизводные (первообразные) — ~25 единиц
        "без",
        "безо",
        "близ",
        "в",
        "во",
        "вне",
        "вопреки",
        "для",
        "до",
        "за",
        "из",
        "изо",
        "из-за",
        "из-под",
        "к",
        "ко",
        "кроме",
        "меж",
        "между",
        "на",
        "над",
        "надо",
        "о",
        "об",
        "обо",
        "от",
        "ото",
        "по",
        "под",
        "подо",
        "при",
        "про",
        "ради",
        "с",
        "со",
        "среди",
        "у",
        "через",
        "чрез",
        // Производные от наречий — наречные
        "вблизи",
        "вглубь",
        "вдоль",
        "взамен",
        "вместо",
        "внутри",
        "внутрь",
        "возле",
        "вокруг",
        "впереди",
        "навстречу",
        "напротив",
        "насквозь",
        "около",
        "поверх",
        "позади",
        "помимо",
        "поперёк",
        "после",
        "посреди",
        "посредине",
        "против",
        "сверх",
        "свыше",
        "сзади",
        "сквозь",
        "снизу",
        "согласно",
        "сообразно",
        "соответственно",
        "соразмерно",
        "спереди",
        "сродни",
        // Производные от существительных — отымённые
        "ввиду",
        "в виде",
        "в зависимости от",
        "в качестве",
        "в лице",
        "в отличие от",
        "в отношении",
        "в преддверии",
        "в продолжение",
        "в роли",
        "в силу",
        "в течение",
        "в целях",
        "вследствие",
        "за вычетом",
        "за исключением",
        "за счёт",
        "наподобие",
        "насчёт",
        "по мере",
        "по поводу",
        "по причине",
        "по случаю",
        "под видом",
        "посредством",
        "путём",
        "с помощью",
        "с точки зрения",
        // Производные от глаголов — отглагольные
        "благодаря",
        "включая",
        "исключая",
        "начиная с",
        "не считая",
        "несмотря на",
        "невзирая на",
        "спустя",
    ])
});
