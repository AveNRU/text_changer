use regex::Regex;
use std::sync::LazyLock;
#[allow(non_camel_case_types)]
pub struct Ложные_окончания_с_исключениями {
    pub ряд_re: &'static [Regex],
    pub ряд_исключений: &'static rapidhash::fast::RapidHashSet<&'static str>,
}
pub static СТОПКА_ЛОЖНЫХ_ОКОНЧАНИЙ: LazyLock<
    [Ложные_окончания_с_исключениями; 38],
> = LazyLock::new(|| {
    [
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ОСТ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ОСТ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_СЕ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_СЕ,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_НИЕ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_НИЕ,
        },
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
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ВА,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ВА,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕЧН,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ЕЧН,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕН,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ЕН,
        },
        Ложные_окончания_с_исключениями {
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ОВ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_ОВ,
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
            ряд_re: &*RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АМ,
            ряд_исключений: &RE_ИСКЛЮЧЕНИЯ_АМ,
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
pub static RE_ИСКЛЮЧЕНИЯ_ОВ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "вызовов",
            "словов",
            "бого-словов",
            "островов",
            "покровов",
            "слововед",
            "слововеда",
            "слововедам",
            "слововедами",
            "слововедах",
            "слововеде",
            "слововеду",
            "слововеды",
            "слововедом",
        ])
    });
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_НИЕ: LazyLock<[Regex; 3]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)ниема").unwrap(),
        //Regex::new(r"(?i)ниемая$").unwrap(),
        // Regex::new(r"(?i)ниемая$").unwrap(),
        Regex::new(r"(?i)ниемо").unwrap(),
        //  Regex::new(r"(?i)ниемост").unwrap(),
        // Regex::new(r"(?i)ниемом$").unwrap(),
        //   Regex::new(r"(?i)ниемому$").unwrap(),
        //   Regex::new(r"(?i)ниемой$").unwrap(),
        //    Regex::new(r"(?i)ниемую$").unwrap(),
        //    Regex::new(r"(?i)ниемого$").unwrap(),
        Regex::new(r"(?i)ниемы").unwrap(),
    ]
});
pub static RE_ИСКЛЮЧЕНИЯ_НИЕ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
//
pub static RE_ИСКЛЮЧЕНИЯ_ОСТ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ОСТ: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)остямы").unwrap(),
        //Regex::new(r"(?i)сецам").unwrap(),
    ]
});
//
pub static RE_ИСКЛЮЧЕНИЯ_СЕ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_СЕ: LazyLock<[Regex; 6]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)сецу").unwrap(),
        //Regex::new(r"(?i)сецам").unwrap(),
        Regex::new(r"(?i)сеца").unwrap(),
        Regex::new(r"(?i)сецо").unwrap(),
        Regex::new(r"(?i)сеце").unwrap(),
        Regex::new(r"(?i)сецы").unwrap(),
        Regex::new(r"(?i)сеци").unwrap(),
    ]
});
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ОВ: LazyLock<[Regex; 1]> =
    LazyLock::new(|| [Regex::new(r"(?i)овов").unwrap()]);
pub static RE_ИСКЛЮЧЕНИЯ_ЕЧН: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕЧН: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)ечнов").unwrap(),
        Regex::new(r"(?i)ечнам").unwrap(),
    ]
});
pub static RE_ИСКЛЮЧЕНИЯ_АМ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АМ: LazyLock<[Regex; 1]> =
    LazyLock::new(|| [Regex::new(r"(?i)гамим").unwrap()]);
pub static RE_ИСКЛЮЧЕНИЯ_ВЕР: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ВЕР: LazyLock<[Regex; 0]> = LazyLock::new(|| {
    [
        //Regex::new(r"(?i)вер").unwrap()
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_ТИ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["гарантией"]));
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ТИ: LazyLock<[Regex; 0]> = LazyLock::new(|| {
    [
       // Regex::new(r"(?i)тией$").unwrap()
    ]
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Д: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [Regex::new(r"(?i)даож").unwrap()]
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_М: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)муюсь$").unwrap(),
        Regex::new(r"(?i)мрст$").unwrap(),
    ]
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_У: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)уи$").unwrap(),
        Regex::new(r"(?i)иую$").unwrap(),
    ]
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_К: LazyLock<[Regex; 3]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)киому").unwrap(),
        Regex::new(r"(?i)кы$").unwrap(),
        Regex::new(r"(?i)коост").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_Ш: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter(["афишы", "кушы", "кэшы"]));
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Ю: LazyLock<[Regex; 2]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)юися").unwrap(),
        Regex::new(r"(?i)аюте").unwrap(),
    ]
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕМ: LazyLock<[Regex; 8]> = LazyLock::new(|| {
    [
        //Regex::new(r"(?i)щем$").unwrap(),
        //Regex::new(r"(?i)щеми$").unwrap(),
        Regex::new(r"(?i)емуемся").unwrap(),
        Regex::new(r"(?i)емует").unwrap(),
        Regex::new(r"(?i)емуетя").unwrap(),
        Regex::new(r"(?i)емуй").unwrap(),
        Regex::new(r"(?i)емуюсь").unwrap(),
        Regex::new(r"(?i)емуют").unwrap(),
        Regex::new(r"(?i)емуются").unwrap(),
        Regex::new(r"(?i)еамом").unwrap(),
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
            "обойм",
        ])
    });
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_И: LazyLock<[Regex; 16]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)игнут").unwrap(),
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_АН: LazyLock<[Regex; 7]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)аннией$").unwrap(),
        //Regex::new(r"(?i)анией$").unwrap(),
        Regex::new(r"(?i)аности$").unwrap(),
        Regex::new(r"(?i)анны$").unwrap(),
        Regex::new(r"(?i)аыми").unwrap(),
        Regex::new(r"(?i)анн$").unwrap(),
        Regex::new(r"(?i)анны$").unwrap(),
        Regex::new(r"(?i)анно$").unwrap(),
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_Н: LazyLock<[Regex; 26]> = LazyLock::new(|| {
    [
        Regex::new(r"(?i)ноно$").unwrap(),
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
        Regex::new(r"(?i)нв$").unwrap(),
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
            "преисподни",
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_СТ: LazyLock<[Regex; 15]> = LazyLock::new(|| {
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
        Regex::new(r"(?i)встью").unwrap(),
    ]
    //----------------------------------
});
pub static RE_ИСКЛЮЧЕНИЯ_СТ: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| {
        rapidhash::fast::RapidHashSet::from_iter([
            "нарост",
            "прирост",
            "рост",
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
            "пост",
            "хвост",
            "хост",
            "тост",
            "репост",
            "помост",
        ])
    });
//
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ВА: LazyLock<[Regex; 1]> = LazyLock::new(|| {
    [
        //----------------------------------
        //ван
        //
        Regex::new(r"(?i)вавать").unwrap(),
    ]
});
pub static RE_ИСКЛЮЧЕНИЯ_ВА: LazyLock<rapidhash::fast::RapidHashSet<&'static str>> =
    LazyLock::new(|| rapidhash::fast::RapidHashSet::from_iter([]));
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
pub static RE_ЛОЖНЫЕ_ОКОНЧАНИЯ_ЕН: LazyLock<[Regex; 23]> = LazyLock::new(|| {
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
        Regex::new(r"(?i)енией").unwrap(),
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
            "репоста",
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
