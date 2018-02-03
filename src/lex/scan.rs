/*
 * Copyright 2017 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// GENERATED by genscan.rs
use lex::scanner::{Result, Scanner};
use lex::hand;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tok {
    EOF,
    LBrace,
    LParen,
    RParen,
    LSquare,
    RSquare,
    Dot,
    Ellipsis,
    Semi,
    Question,
    Colon,
    RBrace,
    Not,
    BNot,
    PlusPlus,
    MinusMinus,
    Eq,
    LT,
    GT,
    LTE,
    GTE,
    EqEq,
    NEq,
    EqEqEq,
    NEqEq,
    Plus,
    Minus,
    Star,
    Percent,
    StarStar,
    LTLT,
    GTGT,
    GTGTGT,
    BAnd,
    BOr,
    Xor,
    AndAnd,
    OrOr,
    PlusEq,
    MinusEq,
    StarEq,
    PercentEq,
    StarStarEq,
    LTLTEq,
    GTGTEq,
    GTGTGTEq,
    AndEq,
    OrEq,
    CaratEq,
    Div,
    DivEq,
    Comma,
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    InstanceOf,
    New,
    Return,
    Super,
    Switch,
    This,
    Throw,
    Try,
    TypeOf,
    Var,
    Void,
    While,
    With,
    Yield,
    Comment,
    String,
    Number,
    Ident,
}
impl Tok {
    pub fn is_kw(&self) -> bool {
        match self {
            &Tok::EOF => false,
            &Tok::LBrace => false,
            &Tok::LParen => false,
            &Tok::RParen => false,
            &Tok::LSquare => false,
            &Tok::RSquare => false,
            &Tok::Dot => false,
            &Tok::Ellipsis => false,
            &Tok::Semi => false,
            &Tok::Question => false,
            &Tok::Colon => false,
            &Tok::RBrace => false,
            &Tok::Not => false,
            &Tok::BNot => false,
            &Tok::PlusPlus => false,
            &Tok::MinusMinus => false,
            &Tok::Eq => false,
            &Tok::LT => false,
            &Tok::GT => false,
            &Tok::LTE => false,
            &Tok::GTE => false,
            &Tok::EqEq => false,
            &Tok::NEq => false,
            &Tok::EqEqEq => false,
            &Tok::NEqEq => false,
            &Tok::Plus => false,
            &Tok::Minus => false,
            &Tok::Star => false,
            &Tok::Percent => false,
            &Tok::StarStar => false,
            &Tok::LTLT => false,
            &Tok::GTGT => false,
            &Tok::GTGTGT => false,
            &Tok::BAnd => false,
            &Tok::BOr => false,
            &Tok::Xor => false,
            &Tok::AndAnd => false,
            &Tok::OrOr => false,
            &Tok::PlusEq => false,
            &Tok::MinusEq => false,
            &Tok::StarEq => false,
            &Tok::PercentEq => false,
            &Tok::StarStarEq => false,
            &Tok::LTLTEq => false,
            &Tok::GTGTEq => false,
            &Tok::GTGTGTEq => false,
            &Tok::AndEq => false,
            &Tok::OrEq => false,
            &Tok::CaratEq => false,
            &Tok::Div => false,
            &Tok::DivEq => false,
            &Tok::Comma => false,
            &Tok::Await => true,
            &Tok::Break => true,
            &Tok::Case => true,
            &Tok::Catch => true,
            &Tok::Class => true,
            &Tok::Const => true,
            &Tok::Continue => true,
            &Tok::Debugger => true,
            &Tok::Default => true,
            &Tok::Delete => true,
            &Tok::Do => true,
            &Tok::Else => true,
            &Tok::Export => true,
            &Tok::Extends => true,
            &Tok::Finally => true,
            &Tok::For => true,
            &Tok::Function => true,
            &Tok::If => true,
            &Tok::Import => true,
            &Tok::In => true,
            &Tok::InstanceOf => true,
            &Tok::New => true,
            &Tok::Return => true,
            &Tok::Super => true,
            &Tok::Switch => true,
            &Tok::This => true,
            &Tok::Throw => true,
            &Tok::Try => true,
            &Tok::TypeOf => true,
            &Tok::Var => true,
            &Tok::Void => true,
            &Tok::While => true,
            &Tok::With => true,
            &Tok::Yield => true,
            &Tok::Comment => false,
            &Tok::String => false,
            &Tok::Number => false,
            &Tok::Ident => false,
        }
    }
}
#[derive(Debug)]
pub enum TokData {
    None,
    String(String),
    Number(f64),
}
pub fn sc(s: &mut Scanner, data: &mut TokData) -> Result<Tok> {
    Ok(match s.read() as char {
        '\u{0}' => Tok::EOF,
        '!' => match s.read() as char {
            '=' => match s.read() as char {
                '=' => Tok::NEqEq,
                _ => {
                    s.back();
                    Tok::NEq
                }
            },
            _ => {
                s.back();
                Tok::Not
            }
        },
        '\"' => {
            *data = TokData::String(hand::quoted(s, '"')?);
            Tok::String
        }
        '%' => match s.read() as char {
            '=' => Tok::PercentEq,
            _ => {
                s.back();
                Tok::Percent
            }
        },
        '&' => match s.read() as char {
            '&' => Tok::AndAnd,
            '=' => Tok::AndEq,
            _ => {
                s.back();
                Tok::BAnd
            }
        },
        '\'' => {
            *data = TokData::String(hand::quoted(s, '\'')?);
            Tok::String
        }
        '(' => Tok::LParen,
        ')' => Tok::RParen,
        '*' => match s.read() as char {
            '*' => match s.read() as char {
                '=' => Tok::StarStarEq,
                _ => {
                    s.back();
                    Tok::StarStar
                }
            },
            '=' => Tok::StarEq,
            _ => {
                s.back();
                Tok::Star
            }
        },
        '+' => match s.read() as char {
            '+' => Tok::PlusPlus,
            '=' => Tok::PlusEq,
            _ => {
                s.back();
                Tok::Plus
            }
        },
        ',' => Tok::Comma,
        '-' => match s.read() as char {
            '-' => Tok::MinusMinus,
            '=' => Tok::MinusEq,
            _ => {
                s.back();
                Tok::Minus
            }
        },
        '.' => match s.read() as char {
            '.' => match s.read() as char {
                '.' => Tok::Ellipsis,
                c => panic!("xxx {:?}", c),
            },
            _ => {
                s.back();
                Tok::Dot
            }
        },
        '/' => match s.read() as char {
            '*' => {
                hand::block_comment(s)?;
                Tok::Comment
            }
            '/' => {
                hand::line_comment(s);
                Tok::Comment
            }
            '=' => Tok::DivEq,
            _ => {
                s.back();
                Tok::Div
            }
        },
        ':' => Tok::Colon,
        ';' => Tok::Semi,
        '<' => match s.read() as char {
            '<' => match s.read() as char {
                '=' => Tok::LTLTEq,
                _ => {
                    s.back();
                    Tok::LTLT
                }
            },
            '=' => Tok::LTE,
            _ => {
                s.back();
                Tok::LT
            }
        },
        '=' => match s.read() as char {
            '=' => match s.read() as char {
                '=' => Tok::EqEqEq,
                _ => {
                    s.back();
                    Tok::EqEq
                }
            },
            _ => {
                s.back();
                Tok::Eq
            }
        },
        '>' => match s.read() as char {
            '=' => Tok::GTE,
            '>' => match s.read() as char {
                '=' => Tok::GTGTEq,
                '>' => match s.read() as char {
                    '=' => Tok::GTGTGTEq,
                    _ => {
                        s.back();
                        Tok::GTGTGT
                    }
                },
                _ => {
                    s.back();
                    Tok::GTGT
                }
            },
            _ => {
                s.back();
                Tok::GT
            }
        },
        '?' => Tok::Question,
        '[' => Tok::LSquare,
        ']' => Tok::RSquare,
        '^' => match s.read() as char {
            '=' => Tok::CaratEq,
            _ => {
                s.back();
                Tok::Xor
            }
        },
        '{' => Tok::LBrace,
        '|' => match s.read() as char {
            '=' => Tok::OrEq,
            '|' => Tok::OrOr,
            _ => {
                s.back();
                Tok::BOr
            }
        },
        '}' => Tok::RBrace,
        '~' => Tok::BNot,
        '0'...'9' => {
            s.back();
            *data = TokData::Number(hand::number(s));
            Tok::Number
        }
        'a'...'z' | 'A'...'Z' | '_' | '$' => {
            hand::ident(s);
            Tok::Ident
        }
        c if c as usize > 127 => {
            hand::ident(s);
            Tok::Ident
        }
        c => panic!("xxx {:?}", c),
    })
}
pub fn kw(text: &[u8]) -> Tok {
    match text.len() {
        2 => {
            if text == "do".as_bytes() {
                return Tok::Do;
            }
            if text == "if".as_bytes() {
                return Tok::If;
            }
            if text == "in".as_bytes() {
                return Tok::In;
            }
        }
        3 => {
            if text == "for".as_bytes() {
                return Tok::For;
            }
            if text == "new".as_bytes() {
                return Tok::New;
            }
            if text == "try".as_bytes() {
                return Tok::Try;
            }
            if text == "var".as_bytes() {
                return Tok::Var;
            }
        }
        4 => {
            if text == "case".as_bytes() {
                return Tok::Case;
            }
            if text == "else".as_bytes() {
                return Tok::Else;
            }
            if text == "this".as_bytes() {
                return Tok::This;
            }
            if text == "void".as_bytes() {
                return Tok::Void;
            }
            if text == "with".as_bytes() {
                return Tok::With;
            }
        }
        5 => {
            if text == "await".as_bytes() {
                return Tok::Await;
            }
            if text == "break".as_bytes() {
                return Tok::Break;
            }
            if text == "catch".as_bytes() {
                return Tok::Catch;
            }
            if text == "class".as_bytes() {
                return Tok::Class;
            }
            if text == "const".as_bytes() {
                return Tok::Const;
            }
            if text == "super".as_bytes() {
                return Tok::Super;
            }
            if text == "throw".as_bytes() {
                return Tok::Throw;
            }
            if text == "while".as_bytes() {
                return Tok::While;
            }
            if text == "yield".as_bytes() {
                return Tok::Yield;
            }
        }
        6 => {
            if text == "delete".as_bytes() {
                return Tok::Delete;
            }
            if text == "export".as_bytes() {
                return Tok::Export;
            }
            if text == "import".as_bytes() {
                return Tok::Import;
            }
            if text == "return".as_bytes() {
                return Tok::Return;
            }
            if text == "switch".as_bytes() {
                return Tok::Switch;
            }
            if text == "typeof".as_bytes() {
                return Tok::TypeOf;
            }
        }
        7 => {
            if text == "default".as_bytes() {
                return Tok::Default;
            }
            if text == "extends".as_bytes() {
                return Tok::Extends;
            }
            if text == "finally".as_bytes() {
                return Tok::Finally;
            }
        }
        8 => {
            if text == "continue".as_bytes() {
                return Tok::Continue;
            }
            if text == "debugger".as_bytes() {
                return Tok::Debugger;
            }
            if text == "function".as_bytes() {
                return Tok::Function;
            }
        }
        10 => {
            if text == "instanceof".as_bytes() {
                return Tok::InstanceOf;
            }
        }
        _ => {}
    }
    return Tok::Ident;
}
