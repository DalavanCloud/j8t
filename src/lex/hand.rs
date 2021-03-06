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

// Hand-written lexers; called by the main autogenerated lexer.

use lex::scanner::{Result, Scanner};
use std;

pub fn whitespace(it: &mut Scanner) -> bool {
    let mut saw_newline = false;
    loop {
        let c = it.peek();
        match c as char {
            '\r' => saw_newline = true,
            '\n' => saw_newline = true,
            ' ' => {}
            '\t' => {}
            _ => return saw_newline,
        }
        it.next();
    }
}

pub fn block_comment(s: &mut Scanner) -> Result<()> {
    loop {
        match s.read() as char {
            '*' => match s.peek() as char {
                '/' => {
                    s.read();
                    return Ok(());
                }
                '\0' => return Err(s.err("unexpected eof")),
                _ => {}
            },
            '\0' => return Err(s.err("unexpected eof")),
            _ => {}
        }
    }
}

pub fn line_comment(s: &mut Scanner) {
    loop {
        match s.read() as char {
            '\0' => return,
            '\n' => {
                // Note: let whitespace scanner read the newline, so it can
                // trigger ASI.
                s.back();
                return;
            }
            _ => {}
        }
    }
}

fn digit_hex(c: u8) -> Option<u8> {
    Some(match c as char {
        '0'...'9' => c - '0' as u8,
        'A'...'F' => 10 + c - 'A' as u8,
        'a'...'f' => 10 + c - 'a' as u8,
        _ => return None,
    })
}

pub fn number_hex(scanner: &mut Scanner) -> u64 {
    let mut val: u64 = 0;
    loop {
        let c = scanner.read();
        let digit = match digit_hex(c) {
            Some(d) => d as u64,
            None => {
                scanner.back();
                return val;
            }
        };
        val = val * 16 + digit;
    }
}

pub fn number(scanner: &mut Scanner) -> Result<f64> {
    let start = scanner.pos;
    match scanner.read() as char {
        '0' => match scanner.read() as char {
            'x' => return Ok(number_hex(scanner) as f64),
            _ => scanner.back(),
        },
        _ => scanner.back(),
    }
    loop {
        match scanner.read() as char {
            '0'...'9' | '.' => {}
            // ExponentPart
            'e' | 'E' => {
                match scanner.read() as char {
                    '-' | '+' => {}
                    _ => scanner.back(),
                }
                // TODO: the full state machine here.
            }
            _ => {
                scanner.back();
                break;
            }
        }
    }
    let end = scanner.pos;
    let str = std::str::from_utf8(&scanner.input[start..end]).unwrap();
    match str.parse() {
        Ok(n) => Ok(n),
        Err(_) => Err(scanner.err(format!("bad number: {:?}", str))),
    }
}

pub fn ident(s: &mut Scanner) {
    loop {
        match s.peek() as char {
            'a'...'z' | 'A'...'Z' | '_' | '$' | '0'...'9' => {}
            c if c as usize > 0x7f => {}
            _ => return,
        }
        s.next();
    }
}

fn unicode_escape(s: &mut Scanner) -> Result<char> {
    let mut codepoint: u32 = 0;
    match s.read() as char {
        '{' => loop {
            let c = s.read();
            if c as char == '}' {
                break;
            }
            let digit = match digit_hex(c) {
                Some(d) => d as u32,
                None => {
                    return Err(s.err("bad hex escape"));
                }
            };
            codepoint = codepoint * 16 + digit;
        },
        _ => {
            s.back();
            for _ in 0..4 {
                let c = s.read();
                let digit = match digit_hex(c) {
                    Some(d) => d as u32,
                    None => {
                        return Err(s.err("bad hex escape"));
                    }
                };
                codepoint = codepoint * 16 + digit;
            }
        }
    };
    match std::char::from_u32(codepoint) {
        Some(c) => Ok(c),
        None => Err(s.err(format!("bad codepoint {}", codepoint))),
    }
}

fn hex_escape(s: &mut Scanner) -> Result<char> {
    let c1 = match digit_hex(s.read()) {
        Some(d) => d as u32,
        None => return Err(s.err("bad hex escape")),
    };
    let c2 = match digit_hex(s.read()) {
        Some(d) => d as u32,
        None => return Err(s.err("bad hex escape")),
    };

    let codepoint = (c1 << 4) | c2;
    match std::char::from_u32(codepoint) {
        Some(c) => Ok(c),
        None => Err(s.err(format!("bad codepoint {}", codepoint))),
    }
}

pub fn quoted(s: &mut Scanner, quote: char) -> Result<String> {
    let mut str: Vec<u8> = Vec::new();
    loop {
        match s.read() as char {
            c if c == quote => break,
            '\0' => panic!("EOF while reading quote"),
            '\\' => {
                match s.read() as char {
                    'b' => str.push(0x8),
                    'f' => str.push(0xC),
                    'n' => str.push('\n' as u8),
                    'r' => str.push('\r' as u8),
                    't' => str.push('\t' as u8),
                    'u' => {
                        let c = unicode_escape(s)?;
                        if (c as u64) < 128 {
                            str.push(c as u8);
                        } else {
                            // TODO: unicode.
                            str.push('?' as u8);
                        }
                    }
                    'v' => str.push(0xB),
                    'x' => {
                        let c = hex_escape(s)?;
                        if (c as u64) < 128 {
                            str.push(c as u8);
                        } else {
                            // TODO: unicode.
                            str.push('?' as u8);
                        }
                    }
                    '"' => str.push('"' as u8),
                    '\'' => str.push('\'' as u8),
                    '\\' => str.push('\\' as u8),
                    '0' => str.push('\0' as u8),
                    // Note: this is invalid but accepted by JSCompiler(?).
                    '/' => str.push('/' as u8),
                    c => return Err(s.err(format!("unknown escape \\{:?}", c))),
                }
            }
            c => str.push(c as u8),
        }
    }
    match String::from_utf8(str) {
        Ok(s) => Ok(s),
        Err(err) => Err(s.err(format!("bad UTF-8: {}", err))),
    }
}

pub fn template(s: &mut Scanner) -> Result<String> {
    let mut str: Vec<u8> = Vec::new();
    loop {
        match s.read() as char {
            '`' => break,
            c => str.push(c as u8),
        }
    }
    match String::from_utf8(str) {
        Ok(s) => Ok(s),
        Err(err) => Err(s.err(format!("bad UTF-8: {}", err))),
    }
}

fn regex_body(s: &mut Scanner) {
    loop {
        match s.read() as char {
            '/' => break,
            '\0' => panic!("EOF while reading regex"),
            '\n' => panic!("newline while reading regex"),
            '\\' => {
                s.read();
            }
            '[' => loop {
                match s.read() as char {
                    '\0' => panic!("EOF while reading regex"),
                    '\n' => panic!("newline while reading regex"),
                    '\\' => {
                        s.read();
                    }
                    ']' => break,
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

pub fn regex(s: &mut Scanner) {
    regex_body(s);
    loop {
        match s.read() as char {
            'g' | 'i' | 'm' => {}
            _ => {
                s.back();
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod regex {
        use super::*;
        fn parse(input: &str) {
            let mut s = Scanner::new(input.as_bytes());
            regex_body(&mut s);
            assert_eq!(s.read(), 0);
        }

        #[test]
        fn body() {
            parse("a[b/]/");
        }
    }

    #[test]
    fn block_comment_stars() {
        let mut s = Scanner::new("/*//***/".as_bytes());
        block_comment(&mut s).unwrap();
        block_comment(&mut s).unwrap();
        assert_eq!(s.read(), 0);
    }

    fn parse_number(s: &str) -> f64 {
        let mut scan = Scanner::new(s.as_bytes());
        let n = number(&mut scan);
        if scan.read() != 0 {
            panic!("leftover text after parse");
        }
        n.unwrap()
    }

    #[test]
    fn number_variants() {
        assert_eq!(parse_number("1"), 1.0);
        assert_eq!(parse_number("1.1"), 1.1);
        assert_eq!(parse_number("0xb"), 11.0);
        assert_eq!(parse_number("1e3"), 1000.0);
        assert_eq!(parse_number("1e-3"), 0.001);
        // TODO: some failing number variants, e.g.
        //  "1e3e3", "1.2.3"
    }
}
