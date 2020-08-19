mod token;

use crate::*;
use nom::{
  branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
};
pub use token::*;

type LexSpan<'a> = nom_locate::LocatedSpan<&'a str>;
type LexResult<'a, T> = nom::IResult<LexSpan<'a>, T, Error<'a>>;

pub fn lex(s: &str) -> LexResult<Vec<Token>> {
  fold_many0(
    alt((map(parse_ignorable, |_| None), map(parse_token, Some))),
    vec![],
    |mut token_list, token| {
      if let Some(token) = token {
        token_list.push(token);
      }
      token_list
    },
  )(nom_locate::LocatedSpan::new(s))
}

fn parse_ignorable(s: LexSpan) -> LexResult<()> {
  alt((parse_whitespace, parse_comment_line, parse_comment_block))(s)
}

fn parse_whitespace(s: LexSpan) -> LexResult<()> {
  map(many1(is_a(" \t\r\n")), |_| ())(s)
}

fn parse_comment_line(s: LexSpan) -> LexResult<()> {
  let (s, _) = tag("//")(s)?;

  if let Some(line) = s.fragment().lines().next() {
    let (s, _) = take(line.len())(s)?;

    Ok((s, ()))
  } else {
    Ok((s, ()))
  }
}

fn parse_comment_block(s: LexSpan) -> LexResult<()> {
  let (s, _) = tag("/*")(s)?;

  if let Some((size, _)) = s.fragment().match_indices("*/").next() {
    let (s, _) = take(size + 2)(s)?;

    Ok((s, ()))
  } else {
    Ok((LexSpan::new(""), ()))
  }
}

fn parse_token(s: LexSpan) -> LexResult<Token> {
  alt((
    map(parse_string, Token::StringLiteral),
    map(parse_numeric, |numeric| match numeric {
      Numeric::Int(n) => Token::IntLiteral(n),
      Numeric::Float(n) => Token::FloatLiteral(n),
    }),
    map(parse_identifier, |identifier| match identifier {
      Identifier::Unreserved(i) => Token::Identifier(i),
      Identifier::Reserved(i) => Token::Keyword(i),
      Identifier::BooleanLiteral(i) => Token::BooleanLiteral(i),
    }),
    map(parse_symbol, Token::Symbol),
    map(parse_illegal, |_| Token::Illegal),
  ))(s)
}

fn parse_string(s: LexSpan) -> LexResult<String> {
  fn pis(s: LexSpan) -> LexResult<Vec<u8>> {
    let (ss, c) = take(1usize)(s)?;
    match c.fragment().to_owned().as_bytes() {
      b"\"" => Ok((s, vec![])),
      b"\\" => {
        let (s, c) = take(1usize)(ss)?;
        pis(s).map(|(s, done)| {
          (
            s,
            concat_slice_vec(c.fragment().to_owned().as_bytes(), done),
          )
        })
      }
      c => pis(ss).map(|(s, done)| (s, concat_slice_vec(c, done))),
    }
  }

  fn concat_slice_vec(c: &[u8], done: Vec<u8>) -> Vec<u8> {
    let mut new_vec = c.to_vec();
    new_vec.extend(&done);
    new_vec
  }

  fn convert_vec_utf8(v: Vec<u8>) -> std::result::Result<String, std::str::Utf8Error> {
    let slice = v.as_slice();
    std::str::from_utf8(slice).map(|s| s.to_owned())
  }

  map(
    tuple((tag("\""), map_res(pis, convert_vec_utf8), tag("\""))),
    |(_, string, _)| string,
  )(s)
}

#[derive(Debug, PartialEq)]
enum Numeric {
  Int(i64),
  Float(f64),
}

fn parse_numeric(s: LexSpan) -> LexResult<Numeric> {
  let is_digit = |c| matches!(c, '0'..='9');
  let is_nonzero_digit = |c| matches!(c, '1'..='9');
  let sign_to_int = |sign: Option<_>| if sign.is_some() { 1 } else { 0 };
  let len = |s: LexSpan| s.fragment().len();

  let (ss, sign) = map(opt(char('-')), sign_to_int)(s)?;

  let (ss, majority) = alt((
    map(tag("0"), |_| 1),
    map(
      tuple((take_while1(is_nonzero_digit), take_while(is_digit))),
      |(nonzero_digit, digit)| len(nonzero_digit) + len(digit),
    ),
  ))(ss)?;

  let (ss, minority) = opt(map(
    tuple((char('.'), take_while(is_digit))),
    |(_, minority): (_, LexSpan)| 1 + len(minority),
  ))(ss)?;

  let (_, exponential) = opt(map(
    tuple((
      alt((char('e'), char('E'))),
      opt(alt((char('+'), char('-')))),
      take_while1(is_digit),
    )),
    |(_, sign, digit)| 1 + sign_to_int(sign) + len(digit),
  ))(ss)?;

  let numeric = match (minority, exponential) {
    (Some(n), Some(m)) => Some(n + m),
    (Some(n), _) => Some(n),
    (_, Some(n)) => Some(n),
    _ => None,
  };

  match numeric {
    Some(n) => {
      let (s, numeric) = take(sign + majority + n)(s)?;
      Ok((s, Numeric::Float(numeric.fragment().parse().unwrap())))
    }
    None => {
      let (s, numeric) = take(sign + majority)(s)?;
      Ok((s, Numeric::Int(numeric.fragment().parse().unwrap())))
    }
  }
}

#[derive(Debug, PartialEq)]
enum Identifier {
  BooleanLiteral(bool),
  Unreserved(String),
  Reserved(Keyword),
}

fn parse_identifier(s: LexSpan) -> LexResult<Identifier> {
  let (s, head) = is_a("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(s)?;
  let (s, rest) = opt(is_a(
    "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456789",
  ))(s)?;
  let string = format!(
    "{}{}",
    head.fragment().to_owned(),
    rest.map(|s| s.fragment().to_owned()).unwrap_or("")
  );

  let identifier = match string.as_ref() {
    "if" => Identifier::Reserved(Keyword::If),
    "else" => Identifier::Reserved(Keyword::Else),
    "for" => Identifier::Reserved(Keyword::For),
    "while" => Identifier::Reserved(Keyword::While),
    "loop" => Identifier::Reserved(Keyword::Loop),
    "in" => Identifier::Reserved(Keyword::In),
    "break" => Identifier::Reserved(Keyword::Break),
    "continue" => Identifier::Reserved(Keyword::Continue),
    "match" => Identifier::Reserved(Keyword::Match),
    "return" => Identifier::Reserved(Keyword::Return),
    "yield" => Identifier::Reserved(Keyword::Yield),
    "where" => Identifier::Reserved(Keyword::Where),
    "const" => Identifier::Reserved(Keyword::Const),
    "static" => Identifier::Reserved(Keyword::Static),
    "let" => Identifier::Reserved(Keyword::Let),
    "mut" => Identifier::Reserved(Keyword::Mut),
    "fn" => Identifier::Reserved(Keyword::Function),
    "trait" => Identifier::Reserved(Keyword::Trait),
    "struct" => Identifier::Reserved(Keyword::Struct),
    "type" => Identifier::Reserved(Keyword::Type),
    "enum" => Identifier::Reserved(Keyword::Enum),
    "impl" => Identifier::Reserved(Keyword::Impl),
    "mod" => Identifier::Reserved(Keyword::Module),
    "Self" => Identifier::Reserved(Keyword::TypeSelf),
    "self" => Identifier::Reserved(Keyword::VariableSelf),
    "pub" => Identifier::Reserved(Keyword::Public),
    "async" => Identifier::Reserved(Keyword::Async),
    "await" => Identifier::Reserved(Keyword::Await),
    "use" => Identifier::Reserved(Keyword::Use),
    "super" => Identifier::Reserved(Keyword::Super),
    "as" => Identifier::Reserved(Keyword::As),
    "_" => Identifier::Reserved(Keyword::Placeholder),

    "true" => Identifier::BooleanLiteral(true),
    "false" => Identifier::BooleanLiteral(false),

    _ => Identifier::Unreserved(string),
  };

  Ok((s, identifier))
}

fn parse_symbol(s: LexSpan) -> LexResult<Symbol> {
  alt((
    parse_symbol0,
    parse_symbol1,
    parse_symbol2,
    parse_symbol3,
    parse_symbol4,
    parse_symbol5,
  ))(s)
}

fn parse_symbol0(s: LexSpan) -> LexResult<Symbol> {
  alt((
    map(tag("("), |_| Symbol::LeftParens),
    map(tag(")"), |_| Symbol::RightParens),
    map(tag("["), |_| Symbol::LeftBracket),
    map(tag("]"), |_| Symbol::RightBracket),
    map(tag("{"), |_| Symbol::LeftBrace),
    map(tag("}"), |_| Symbol::RightBrace),
  ))(s)
}

fn parse_symbol1(s: LexSpan) -> LexResult<Symbol> {
  alt((
    map(tag("->"), |_| Symbol::ReturnArrow),
    map(tag("=>"), |_| Symbol::BranchArrow),
    map(tag("..="), |_| Symbol::RangeOpen),
    map(tag(".."), |_| Symbol::RangeClose),
    map(tag("."), |_| Symbol::Dot),
    map(tag(","), |_| Symbol::Comma),
    map(tag("::"), |_| Symbol::DoubleColon),
    map(tag(":"), |_| Symbol::Colon),
    map(tag(";"), |_| Symbol::Semicolon),
  ))(s)
}

fn parse_symbol2(s: LexSpan) -> LexResult<Symbol> {
  alt((
    map(tag("=="), |_| Symbol::Equal),
    map(tag("="), |_| Symbol::Assign),
    map(tag("!="), |_| Symbol::NotEqual),
    map(tag("+="), |_| Symbol::AddAssign),
    map(tag("-="), |_| Symbol::SubAssign),
    map(tag("**="), |_| Symbol::ExpAssign),
    map(tag("*="), |_| Symbol::MulAssign),
    map(tag("/="), |_| Symbol::DivAssign),
    map(tag("%="), |_| Symbol::ModAssign),
    map(tag("&&="), |_| Symbol::AndAssign),
    map(tag("||="), |_| Symbol::OrAssign),
  ))(s)
}

fn parse_symbol3(s: LexSpan) -> LexResult<Symbol> {
  alt((
    map(tag("+"), |_| Symbol::Add),
    map(tag("-"), |_| Symbol::Sub),
    map(tag("**"), |_| Symbol::Exp),
    map(tag("*"), |_| Symbol::Mul),
    map(tag("/"), |_| Symbol::Div),
    map(tag("%"), |_| Symbol::Mod),
    map(tag("&&"), |_| Symbol::And),
    map(tag("||"), |_| Symbol::Or),
    map(tag("!"), |_| Symbol::Not),
    map(tag("?"), |_| Symbol::Question),
  ))(s)
}

fn parse_symbol4(s: LexSpan) -> LexResult<Symbol> {
  alt((
    map(tag("&="), |_| Symbol::BitAndAssign),
    map(tag("|="), |_| Symbol::BitOrAssign),
    map(tag("^="), |_| Symbol::BitXorAssign),
    map(tag("<<="), |_| Symbol::BitLeftAssign),
    map(tag(">>="), |_| Symbol::BitRightAssign),
    map(tag("&"), |_| Symbol::BitAnd),
    map(tag("|"), |_| Symbol::BitOr),
    map(tag("~"), |_| Symbol::BitNot),
    map(tag("^"), |_| Symbol::BitXor),
    map(tag("<<"), |_| Symbol::BitLeft),
    map(tag(">>"), |_| Symbol::BitRight),
  ))(s)
}

fn parse_symbol5(s: LexSpan) -> LexResult<Symbol> {
  alt((
    map(tag(">="), |_| Symbol::GreaterThanOrEqual),
    map(tag("<="), |_| Symbol::LessThanOrEqual),
    map(tag(">"), |_| Symbol::GreaterThan),
    map(tag("<"), |_| Symbol::LessThan),
    map(tag("="), |_| Symbol::Assign),
    map(tag("+"), |_| Symbol::Add),
  ))(s)
}

fn parse_illegal(s: LexSpan) -> LexResult<()> {
  map(take(1usize), |_| ())(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn string() {
    let source = "\"foo\"";
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::StringLiteral("foo".to_owned())])
    );

    let source = "\"foo
bar\"";
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::StringLiteral("foo\nbar".to_owned())])
    );
  }

  #[test]
  fn int() {
    let source = r#"1"#;
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::IntLiteral(1)])
    );

    let source = r#"-1"#;
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::IntLiteral(-1)])
    );
  }

  #[test]
  fn float() {
    let source = r#"1.0"#;
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::FloatLiteral(1.0)])
    );

    let source = r#"-1.0"#;
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::FloatLiteral(-1.0)])
    );

    let source = r#"1e0"#;
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::FloatLiteral(1e0)])
    );

    let source = r#"-1e0"#;
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::FloatLiteral(-1e0)])
    );

    let source = r#"1.0e0"#;
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::FloatLiteral(1.0e0)])
    );

    let source = r#"-1.0e0"#;
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![Token::FloatLiteral(-1.0e0)])
    );
  }

  #[test]
  fn keyword() {
    let source = r#"if else for while loop in break continue match return yield where
const static let mut fn trait struct type enum impl mod Self self pub async await use super as _"#;
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![
        Token::Keyword(Keyword::If),
        Token::Keyword(Keyword::Else),
        Token::Keyword(Keyword::For),
        Token::Keyword(Keyword::While),
        Token::Keyword(Keyword::Loop),
        Token::Keyword(Keyword::In),
        Token::Keyword(Keyword::Break),
        Token::Keyword(Keyword::Continue),
        Token::Keyword(Keyword::Match),
        Token::Keyword(Keyword::Return),
        Token::Keyword(Keyword::Yield),
        Token::Keyword(Keyword::Where),
        Token::Keyword(Keyword::Const),
        Token::Keyword(Keyword::Static),
        Token::Keyword(Keyword::Let),
        Token::Keyword(Keyword::Mut),
        Token::Keyword(Keyword::Function),
        Token::Keyword(Keyword::Trait),
        Token::Keyword(Keyword::Struct),
        Token::Keyword(Keyword::Type),
        Token::Keyword(Keyword::Enum),
        Token::Keyword(Keyword::Impl),
        Token::Keyword(Keyword::Module),
        Token::Keyword(Keyword::TypeSelf),
        Token::Keyword(Keyword::VariableSelf),
        Token::Keyword(Keyword::Public),
        Token::Keyword(Keyword::Async),
        Token::Keyword(Keyword::Await),
        Token::Keyword(Keyword::Use),
        Token::Keyword(Keyword::Super),
        Token::Keyword(Keyword::As),
        Token::Keyword(Keyword::Placeholder),
      ])
    );
  }

  #[test]
  fn symbol() {
    let source = "( ) [ ] { }
-> => ..= .. . , :: : ;
== = != += -= **= *= /= %= &&= ||=
+ - ** * / % && || ! ?
&= |= ^= <<= >>= & | ~ ^ << >>
>= <= > <";
    assert_eq!(
      lex(source).map(|(_, token_list)| token_list),
      Ok(vec![
        Token::Symbol(Symbol::LeftParens),
        Token::Symbol(Symbol::RightParens),
        Token::Symbol(Symbol::LeftBracket),
        Token::Symbol(Symbol::RightBracket),
        Token::Symbol(Symbol::LeftBrace),
        Token::Symbol(Symbol::RightBrace),
        Token::Symbol(Symbol::ReturnArrow),
        Token::Symbol(Symbol::BranchArrow),
        Token::Symbol(Symbol::RangeOpen),
        Token::Symbol(Symbol::RangeClose),
        Token::Symbol(Symbol::Dot),
        Token::Symbol(Symbol::Comma),
        Token::Symbol(Symbol::DoubleColon),
        Token::Symbol(Symbol::Colon),
        Token::Symbol(Symbol::Semicolon),
        Token::Symbol(Symbol::Equal),
        Token::Symbol(Symbol::Assign),
        Token::Symbol(Symbol::NotEqual),
        Token::Symbol(Symbol::AddAssign),
        Token::Symbol(Symbol::SubAssign),
        Token::Symbol(Symbol::ExpAssign),
        Token::Symbol(Symbol::MulAssign),
        Token::Symbol(Symbol::DivAssign),
        Token::Symbol(Symbol::ModAssign),
        Token::Symbol(Symbol::AndAssign),
        Token::Symbol(Symbol::OrAssign),
        Token::Symbol(Symbol::Add),
        Token::Symbol(Symbol::Sub),
        Token::Symbol(Symbol::Exp),
        Token::Symbol(Symbol::Mul),
        Token::Symbol(Symbol::Div),
        Token::Symbol(Symbol::Mod),
        Token::Symbol(Symbol::And),
        Token::Symbol(Symbol::Or),
        Token::Symbol(Symbol::Not),
        Token::Symbol(Symbol::Question),
        Token::Symbol(Symbol::BitAndAssign),
        Token::Symbol(Symbol::BitOrAssign),
        Token::Symbol(Symbol::BitXorAssign),
        Token::Symbol(Symbol::BitLeftAssign),
        Token::Symbol(Symbol::BitRightAssign),
        Token::Symbol(Symbol::BitAnd),
        Token::Symbol(Symbol::BitOr),
        Token::Symbol(Symbol::BitNot),
        Token::Symbol(Symbol::BitXor),
        Token::Symbol(Symbol::BitLeft),
        Token::Symbol(Symbol::BitRight),
        Token::Symbol(Symbol::GreaterThanOrEqual),
        Token::Symbol(Symbol::LessThanOrEqual),
        Token::Symbol(Symbol::GreaterThan),
        Token::Symbol(Symbol::LessThan),
      ])
    );
  }
}
