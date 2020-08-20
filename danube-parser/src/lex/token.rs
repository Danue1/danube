#[derive(Debug, PartialEq, Clone)]
pub struct Tokens<'a> {
  pub list: &'a [Token],
  pub start: usize,
  pub end: usize,
}

impl<'a> Tokens<'a> {
  pub fn new(list: &'a [Token]) -> Self {
    Tokens {
      list,
      start: 0,
      end: list.len(),
    }
  }
}

impl<'a> nom::InputLength for Tokens<'a> {
  #[inline]
  fn input_len(&self) -> usize {
    self.list.len()
  }
}

impl<'a> nom::InputTake for Tokens<'a> {
  #[inline]
  fn take(&self, count: usize) -> Self {
    Tokens {
      list: &self.list[0..count],
      start: 0,
      end: count,
    }
  }

  #[inline]
  fn take_split(&self, count: usize) -> (Self, Self) {
    let (prefix, suffix) = self.list.split_at(count);
    let first = Tokens {
      list: prefix,
      start: 0,
      end: prefix.len(),
    };
    let second = Tokens {
      list: suffix,
      start: 0,
      end: suffix.len(),
    };
    (second, first)
  }
}

impl nom::InputLength for Token {
  #[inline]
  fn input_len(&self) -> usize {
    1
  }
}

impl<'a> nom::Slice<std::ops::Range<usize>> for Tokens<'a> {
  #[inline]
  fn slice(&self, range: std::ops::Range<usize>) -> Self {
    Tokens {
      list: self.list.slice(range.clone()),
      start: self.start + range.start,
      end: self.start + range.end,
    }
  }
}

impl<'a> nom::Slice<std::ops::RangeTo<usize>> for Tokens<'a> {
  #[inline]
  fn slice(&self, range: std::ops::RangeTo<usize>) -> Self {
    self.slice(0..range.end)
  }
}

impl<'a> nom::Slice<std::ops::RangeFrom<usize>> for Tokens<'a> {
  #[inline]
  fn slice(&self, range: std::ops::RangeFrom<usize>) -> Self {
    self.slice(range.start..self.end - self.start)
  }
}

impl<'a> nom::Slice<std::ops::RangeFull> for Tokens<'a> {
  #[inline]
  fn slice(&self, _: std::ops::RangeFull) -> Self {
    Tokens {
      list: self.list,
      start: self.start,
      end: self.end,
    }
  }
}

impl<'a> nom::InputIter for Tokens<'a> {
  type Item = &'a Token;
  type Iter = std::iter::Enumerate<::std::slice::Iter<'a, Token>>;
  type IterElem = std::slice::Iter<'a, Token>;

  #[inline]
  fn iter_indices(&self) -> Self::Iter {
    self.list.iter().enumerate()
  }

  #[inline]
  fn iter_elements(&self) -> Self::IterElem {
    self.list.iter()
  }

  #[inline]
  fn position<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Item) -> bool,
  {
    self.list.iter().position(predicate)
  }

  #[inline]
  fn slice_index(&self, count: usize) -> Option<usize> {
    if self.list.len() >= count {
      Some(count)
    } else {
      None
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Keyword(Keyword),
  Symbol(Symbol),
  BooleanLiteral(bool),
  IntLiteral(i64),
  FloatLiteral(f64),
  StringLiteral(String),
  Identifier(String),
  Illegal,
  EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
  If,           // if
  Else,         // else
  For,          // for
  While,        // while
  Loop,         // loop
  In,           // in
  Break,        // break
  Continue,     // continue
  Match,        // match
  Return,       // return
  Yield,        // yield
  Where,        // where
  Const,        // const
  Static,       // static
  Let,          // let
  Mut,          // mut
  Function,     // fn
  Trait,        // trait
  Struct,       // struct
  Type,         // type
  Enum,         // enum
  Impl,         // impl
  Module,       // mod
  TypeSelf,     // Self
  VariableSelf, // self
  Public,       // pub
  Async,        // async
  Await,        // await
  Use,          // use
  Super,        // super
  As,           // as
  Placeholder,  // _
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
  LeftParens,         // (
  RightParens,        // )
  LeftBracket,        // [
  RightBracket,       // ]
  LeftBrace,          // {
  RightBrace,         // }
  ChainArrow,         // |>
  ReturnArrow,        // ->
  BranchArrow,        // =>
  RangeOpen,          // ..=
  RangeClose,         // ..
  Dot,                // .
  Comma,              // ,
  DoubleColon,        // ::
  Colon,              // :
  Semicolon,          // ;
  Equal,              // ==
  Assign,             // =
  NotEqual,           // !=
  AddAssign,          // +=
  SubAssign,          // -=
  ExpAssign,          // **=
  MulAssign,          // *=
  DivAssign,          // /=
  ModAssign,          // %=
  AndAssign,          // &&=
  OrAssign,           // ||=
  Add,                // +
  Sub,                // -
  Exp,                // **
  Mul,                // *
  Div,                // /
  Mod,                // %
  And,                // &&
  Or,                 // ||
  Not,                // !
  Question,           // ?
  BitAndAssign,       // &=
  BitOrAssign,        // |=
  BitXorAssign,       // ^=
  BitLeftAssign,      // <<=
  BitRightAssign,     // >>=
  BitAnd,             // &
  BitOr,              // |
  BitNot,             // ~
  BitXor,             // ^
  BitLeft,            // <<
  BitRight,           // >>
  GreaterThanOrEqual, // >=
  LessThanOrEqual,    // <=
  GreaterThan,        // >
  LessThan,           // <
}
