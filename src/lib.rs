use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
  EOF,
  DEF,
  EXTERN,
  IDENTIFIER(String),
  NUMBER(f64),
  UNREGICNIZE(char),
}

// We can't both store value and reference here. Thus, it is client's responsibility to own the value.
// Reference is enough here.
pub struct Lexer<'a> {
  // data: String,
  curr: Chars<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(data: &'a str) -> Lexer<'a> {
    Lexer { curr: data.chars() }
  }

  fn getchar(&mut self) -> Option<char> {
    self.curr.next()
  }

  pub fn gettok(&mut self) -> Token {
    let mut last_char = ' ' ;
    while last_char.is_whitespace() {
      if let Some(ch) = self.getchar() {
        last_char = ch;
      } else {
        return Token::EOF;
      }
    }
    if last_char.is_alphabetic() {
      let mut s = last_char.to_string();
      while let Some(ch) = self.getchar() {
        if ch.is_alphanumeric() {
          s.push(ch);
        } else {
          last_char = ch;
          break;
        }
      }
      return match s.as_str() {
        "def" => Token::DEF,
        "extern" => Token::EXTERN,
        _ => Token::IDENTIFIER(s)
      }
    }
    if last_char.is_ascii_digit() {
      let mut s = last_char.to_string();
      while let Some(ch) = self.getchar() {
        if ch.is_ascii_digit() || ch == '.' {
          s.push(ch);
        } else {
          last_char = ch;
          break;
        }
      }
      return Token::NUMBER(s.parse::<f64>().unwrap_or(0.0));
    }
    if last_char == '#' {
      while let Some(ch) = self.getchar() {
        if ch == '\n' || ch == '\r' {
          last_char = ch;
          break;
        }
      }
      return self.gettok();
    }
    Token::UNREGICNIZE(last_char)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use Token::*;
  #[test]
  fn test_lex() {
    let document = "def this haha extern \
          !! 128 256  X".to_string();
    let mut lexer = Lexer::new(&document);
    assert_eq!(lexer.gettok(), DEF);
    assert_eq!(lexer.gettok(), IDENTIFIER("this".to_string()));
    assert_eq!(lexer.gettok(), IDENTIFIER("haha".to_string()));
    assert_eq!(lexer.gettok(), EXTERN);
    assert_eq!(lexer.gettok(), UNREGICNIZE('!'));
    assert_eq!(lexer.gettok(), UNREGICNIZE('!'));
    assert_eq!(lexer.gettok(), NUMBER(128.0));
    assert_eq!(lexer.gettok(), NUMBER(256.0));
    assert_eq!(lexer.gettok(), IDENTIFIER("X".to_string()));
    assert_eq!(lexer.gettok(), EOF);
    assert_eq!(lexer.gettok(), EOF);
    assert_eq!(lexer.gettok(), EOF);
    assert_eq!(lexer.gettok(), EOF);
    assert_eq!(lexer.gettok(), EOF);
  }
}