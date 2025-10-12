use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Вины" => "Err",
        "Хорошо" => "Ok",
        "Цепь" => "String",
        "Словарь" => "HashMap",
        "Дефолт" => "Default",
        "Вина" => "Error",
        "Авось" => "Option",
        "Некоторые" => "Some",
        "Никто" => "None",
        "Результат" => "Result",
        "Себя" => "Self",
        "афиша" => "println",
        "перерыв" => "break",
        "асинхронный" => "async",
        "ждет" => "await",
        "петля" => "loop",
        "движение" => "move",
        "ящик" => "crate",
        "недоступный_код" => "unreachable_code",
        "как" => "as",
        "постоянный" => "const",
        "конвенция" => "trait",
        "опасный" => "unsafe",
        "из" => "in",
        "поскольку" => "from",
        "динамичный" => "dyn",
        "распаковать" => "unwrap",
        "дефолт" => "default",
        "в_ссылке" => "as_ref",
        "внешний" => "extern",
        "ложный" => "false",
        "функция" => "fn",
        "классно" => "super",
        "включать" => "insert",
        "читать" => "get",
        "законный" => "allow",
        "блять" | "блин" | "упс" => "panic",
        "модуль" => "mod",
        "изменчивый" => "mut",
        "новый" => "new",
        "куда" => "where",
        "для" => "for",
        "взять_или_вставить_с" => "get_or_insert_with",
        "главный" => "main",
        "общественный" => "pub",
        "какой" => None?,
        "возвратить" => "return",
        "реализация" => "impl",
        "ссылка" => "ref",
        "по" => "match",
        "если" => "if",
        "иначе" => "else",
        "себя" => "self",
        "есть" => "let",
        "статический" => "static",
        "состав" => "struct",
        "предполагаемый" => "expect",
        "пока" => "while",
        "использовать" => "use",
        "к" => "into",
        "истинный" => "true",
        "перечисление" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rzhavchina(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
