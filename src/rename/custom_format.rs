use crate::rename::custom_format::Token::{TDelimBegin, TDelimEnd};

#[derive(Debug, PartialEq)]
pub enum Property
{
    Number,
}

static MY_PROPS: [(&str, Property, &str); 1] = [("NN", Property::Number, "The files sequential number")];

#[derive(Debug, PartialEq, Clone)]
pub enum FormatPart
{
    Const(String),
    Derived(&'static Property),
}

fn lookup_prop(n: &str) -> Option<&'static Property>
{
    for (pn, p, _) in MY_PROPS.iter() {
        if n.eq(*pn) { return Some(p)}
    }
    None
}
type ParseErr = String;

type Res<T> = Result<T, ParseErr>;
pub fn parse(s: &str) -> Result<Vec<FormatPart>, ParseErr>
{
    let mut elements = elements_of(tokenize(s))?;
    let mut ret_val = Vec::with_capacity(elements.len());
    for elem in elements.drain(..) {
        match elem {
            Element::EConst(s) => ret_val.push(FormatPart::Const(s)),
            Element::ERef(pn) => {
                match lookup_prop(&pn) {
                    Some(p) => ret_val.push(FormatPart::Derived(p)),
                    None => return Err(format!("Not a property: {}", &pn))
                }
            }
        }
    }
    if ret_val.is_empty() {
        Err("Empty file name format".to_string())
    } else {
        Ok(ret_val)
    }
}


#[derive(Debug, PartialEq)]
pub enum Element
{
    EConst(String),
    ERef(String),
}

enum ElemParseState
{
    Outside,
    HasRefDelimBegin,
    HasRef,
}
fn elements_of(mut tokens: Vec<(usize, Token)>) -> Res<Vec<Element>>
{
    let mut ret_val = Vec::with_capacity(tokens.len());
    let mut status = ElemParseState::Outside;
    for (_idx, token) in tokens.drain(..) {
        match token {
            Token::TConst(s) => {
                match status {
                    ElemParseState::Outside => ret_val.push(Element::EConst(s)),
                    ElemParseState::HasRefDelimBegin => {
                        ret_val.push(Element::ERef(s));
                        status = ElemParseState::HasRef;
                    },
                    ElemParseState::HasRef => { panic!("This state should be impossible") },
                }
            },
            Token::TDelimBegin  => match status {
                ElemParseState::Outside => status = ElemParseState::HasRefDelimBegin,
                _ => return Err(format!("Expecting property name, found {}", tok_to_str(&token))),
            },
            Token::TDelimEnd => {
                match status {
                    ElemParseState::HasRef => status = ElemParseState::Outside,
                    ElemParseState::Outside => return Err("Unexpected }".to_owned()),
                    ElemParseState::HasRefDelimBegin => return Err("Missing property name after {".to_owned()),
                    }
                },
            }
        }
    match status {
        ElemParseState::HasRef => { return Err("Missing }".to_string()) },
        ElemParseState::HasRefDelimBegin => { return Err("Missing property name after {".to_string()) },
        ElemParseState::Outside => {}
    }
    Ok(ret_val)
}


#[derive(Debug, PartialEq)]
enum Token
{
    TConst(String),
    TDelimBegin,
    TDelimEnd,
}

fn tok_to_str(token: &Token) -> &str
{
    match token {
        Token::TConst(s) => s.as_str(),
        Token::TDelimBegin => "{",
        Token::TDelimEnd => "}",
    }
}
fn tokenize(s: &str) -> Vec<(usize, Token)>
{
    let mut ret_val = Vec::new();
    let mut start: usize = 0;
    for (idx, ch) in s.char_indices() {
        if ch == '{' || ch == '}' {
            if idx != start {
                ret_val.push((start, Token::TConst(s[start..idx].to_string())));
            }
            let part = if ch == '{' { TDelimBegin } else { TDelimEnd };
            ret_val.push((idx, part));
            start = idx + 1;
        }
    }
    if start != s.len() {
        ret_val.push((start, Token::TConst(s[start..].to_string())));
    }
    ret_val
}

#[test]
fn test_parse()
{
    fn c(s: &str) -> FormatPart
    {
        FormatPart::Const(s.to_string())
    }

    fn r(p: &'static Property) -> FormatPart
    {
        FormatPart::Derived(p)
    }

    fn parse_from_str(s: &str) -> Res<Vec<FormatPart>>
    {
        parse(s)
    }

    fn err(s: &str) -> Res<Vec<FormatPart>>
    {
        Err(s.to_string())
    }

    assert_eq!(parse_from_str(""), err("Empty file name format"));
    assert_eq!(parse_from_str("no delim characters"), Ok(vec!(c("no delim characters"))));
    assert_eq!(parse_from_str("{"), err("Missing property name after {"));
    assert_eq!(parse_from_str("{{"), err("Expecting property name, found {"));
    assert_eq!(parse_from_str("}"),  err("Unexpected }"));
    assert_eq!(parse_from_str("}}"), err("Unexpected }"));
    assert_eq!(parse_from_str("{NN}"), Ok(vec!(r(&Property::Number))));
    assert_eq!(parse_from_str("abc-{NN}-def"), Ok(vec!(c("abc-"), r(&Property::Number), c("-def"))));
    assert_eq!(parse_from_str("{fp"),      err("Missing }"));
    assert_eq!(parse_from_str("}fp{"),     err("Unexpected }"));
    assert_eq!(parse_from_str("abc{}def"), err("Missing property name after {"));
    assert_eq!(parse_from_str("abc{non_exist_prop}def"), err("Not a property: non_exist_prop"));
}

#[test]
fn test_elements_of()
{
    fn c(s: &str) -> Element
    {
        Element::EConst(s.to_string())
    }

    fn r(s: &str) -> Element
    {
        Element::ERef(s.to_string())
    }

    fn el_tok(s: &str) -> Res<Vec<Element>>
    {
        elements_of(tokenize(s))
    }
    assert_eq!(el_tok(""), Ok(Vec::new()));
    assert_eq!(el_tok("no delim characters"), Ok(vec!(c("no delim characters"))));
    assert_eq!(el_tok("{"), Err(String::from("Missing property name after {")));
    assert_eq!(el_tok("{{"), Err(String::from("Expecting property name, found {")));
    assert_eq!(el_tok("}"), Err(String::from("Unexpected }")));
    assert_eq!(el_tok("}}"), Err(String::from("Unexpected }")));
    assert_eq!(el_tok("{fp}"), Ok(vec!(r("fp"))));
    assert_eq!(el_tok("{fp"), Err(String::from("Missing }")));
    assert_eq!(el_tok("}fp{"), Err(String::from("Unexpected }")));
    assert_eq!(el_tok("abc{}def"), Err(String::from("Missing property name after {")));
    assert_eq!(el_tok("abc{prop}def"), Ok(vec!(c("abc"), r("prop"), c("def"))));
}

#[test]
fn test_tokenize()
{
    fn sp(n: usize, s: &str) -> (usize, Token)
    {
        (n, Token::TConst(s.to_string()))
    }

    assert_eq!(tokenize(""), Vec::new());
    assert_eq!(tokenize("no delim characters"), vec!(sp(0, "no delim characters")));
    assert_eq!(tokenize("{"), vec!((0, TDelimBegin)));
    assert_eq!(tokenize("{{"), vec!((0, TDelimBegin), (1, TDelimBegin)));
    assert_eq!(tokenize("}"), vec!((0, TDelimEnd)));
    assert_eq!(tokenize("}}"), vec!((0, TDelimEnd), (1, TDelimEnd)));
    assert_eq!(tokenize("{fp}"), vec!((0, TDelimBegin), sp(1, "fp"), (3, TDelimEnd)));
    assert_eq!(tokenize("}fp{"), vec!((0, TDelimEnd), sp(1, "fp"), (3, TDelimBegin)));
    assert_eq!(tokenize("abc{}def"), vec!(sp(0, "abc"), (3, TDelimBegin), (4, TDelimEnd), sp(5, "def")));
}
