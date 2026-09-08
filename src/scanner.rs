use crate::token::Token;

use std::fs::File;
use std::io::{BufReader, Read, Bytes};
use std::iter::Peekable;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
enum ReadStatus {
    Continue,
    Stop,
    Error(String)
}

pub struct Scanner {
    // buffer reads for performance
    // allow peaking for the = vs. == case
    reader: Peekable<Bytes<BufReader<File>>>,
    curr_token: Token,
    separator_set: HashSet<char>,
    symbol_map: HashMap<char, Token>,
    keyword_map: HashMap<String, Token>,

    separator_continue_read: fn(char, &Scanner) -> ReadStatus,
    noop_process_output: fn(&str) -> Token,
    constant_continue_read: fn(char, &Scanner) -> ReadStatus,
    constant_process_output: fn(&str) -> Token,
    identifier_continue_read: fn(char, &Scanner) -> ReadStatus,
    identifier_process_output: fn(&str) -> Token,
    string_continue_read: fn(char, &Scanner) -> ReadStatus,
    string_process_output: fn(&str) -> Token
}

impl Scanner {
    // associated constants
    const BUF_DEFAULT_LENGTH: usize = 64;

    const STRING_DELIMITER: char = '\'';
    const CONST_MIN: u16 = 0;
    const CONST_MAX: u16 = 8191;

    fn is_separator(&self, c: char) -> bool {
        self.separator_set.contains(&c)
    }

    fn match_symbols(&mut self, c: char) -> Token {
        let mut output_token = 
        match self.symbol_map.get(&c) {
            Some(token) => token.clone(),
            None => return Token::ERROR(String::from("Symbol not found"))
        };

        // consume the symbol
        self.reader.next();

        // handle = vs ==
        if output_token == Token::ASSIGN {
            let mut next_char = ' ';
            if let Some(peek_res) = self.reader.peek() {
                if let Ok(byte_read) = peek_res {
                    next_char = *byte_read as char;
                }
            }

            if *self.symbol_map.get(&next_char).unwrap_or(&Token::EOS) == Token::ASSIGN {
                output_token = Token::EQUAL;
                self.reader.next(); // consume the second =
            }
        }

        return output_token;
    }

    fn read_buf_until(&mut self, continue_read: impl Fn(char, &Scanner) -> ReadStatus, process_output: impl Fn(&str) -> Token, eos_on_eos: bool) -> Token {
        let mut string = String::with_capacity(Scanner::BUF_DEFAULT_LENGTH);
        let mut curr_status = ReadStatus::Continue;

        while curr_status == ReadStatus::Continue {
            // handle EOS based on caller settings
            let Some(peek_res) = self.reader.peek() else {
                if eos_on_eos {
                    return Token::EOS;
                } else {
                    return process_output(&string);
                }
            };

            // handle read error
            let curr_char: char = 
            match peek_res {
                Ok(curr_u8) => *curr_u8 as char,
                Err(error) => return Token::ERROR(error.to_string())
            };
            
            // either continue reading and consume char, or error out
            curr_status = continue_read(curr_char, self);
            match curr_status {
                ReadStatus::Continue => { string.push(curr_char); self.reader.next(); },
                ReadStatus::Error(ref error_str) => string = error_str.clone(),
                _ => ()
            }
        }

        match curr_status {
            ReadStatus::Stop => process_output(&string),
            ReadStatus::Error(error_str) => Token::ERROR(error_str),
            _ => Token::EOS // arbitrary, should never happen
        }

    }

    pub fn next_token(&mut self) {
        // flush separators and handle EOS case
        let separator_token = self.read_buf_until(self.separator_continue_read, self.noop_process_output, true);
        if  separator_token == Token::EOS || matches!(separator_token, Token::ERROR { .. }) {
            self.curr_token = Token::EOS;
            return;
        }

        // peek the next character to match on 
        // we know it exists since !EOS guarantee by above
        let curr_char: char;
        match self.reader.peek().expect("This should never happen") {
            Ok(byte) => curr_char = *byte as char,
            Err(e) => {
                self.curr_token = Token::ERROR(e.to_string());
                return;
            }
        }

        // match for special characters
        let symbol_token = self.match_symbols(curr_char);
        if !matches!(symbol_token, Token::ERROR { .. }) {
            self.curr_token = symbol_token;
            return;
        }

        // extract string
        if curr_char == Scanner::STRING_DELIMITER {
            self.reader.next(); // consume starting '
            match self.read_buf_until(self.string_continue_read, self.string_process_output, true) {
                Token::EOS => 
                    self.curr_token = Token::ERROR(String::from("Unclosed string (EOS reached before closing ')")),
                other => self.curr_token = other
            }

            self.reader.next(); // consume ending '
            return;
        }

        // extract constant
        if curr_char.is_numeric() {
            self.curr_token = self.read_buf_until(self.constant_continue_read, self.constant_process_output, false);
            return;
        }

        // extract ID and check against keyword map
        if curr_char.is_alphabetic() {
            self.curr_token = self.read_buf_until(self.identifier_continue_read, self.identifier_process_output, false);

            if let Token::ID(id_str) = &self.curr_token {
                if let Some(keyword) = self.keyword_map.get(id_str) {
                    self.curr_token = keyword.clone();
                }
            }
            return;
        }

        // error for unknown symbol if unmatched at ths point
        self.curr_token = Token::ERROR(format!("Unrecognized symbol: \"{curr_char}\""));
    }

    pub fn current_token(&self) -> Token {
        self.curr_token.clone()
    }

    // if desired, everything in here could be exposed for the user to set
    // but i want to keep the main function clean so i'll leave it here
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        // special chars
        let mut symbol_map = HashMap::new();
        symbol_map.insert('+', Token::ADD);
        symbol_map.insert('-', Token::SUBTRACT);
        symbol_map.insert('*', Token::MULTIPLY);
        symbol_map.insert('/', Token::DIVIDE);
        symbol_map.insert('=', Token::ASSIGN);
        symbol_map.insert('<', Token::LESS);
        symbol_map.insert(':', Token::COLON);
        symbol_map.insert(';', Token::SEMICOLON);
        symbol_map.insert('.', Token::PERIOD);
        symbol_map.insert(',', Token::COMMA);
        symbol_map.insert('(', Token::LPAREN);
        symbol_map.insert(')', Token::RPAREN);
        symbol_map.insert('[', Token::LSQUARE);
        symbol_map.insert(']', Token::RSQUARE);
        symbol_map.insert('{', Token::LCURL);
        symbol_map.insert('}', Token::RCURL);

        // keywords
        let mut keyword_map = HashMap::new();
        keyword_map.insert(String::from("and"), Token::AND);
        keyword_map.insert(String::from("begin"), Token::BEGIN);
        keyword_map.insert(String::from("case"), Token::CASE);
        keyword_map.insert(String::from("do"), Token::DO);
        keyword_map.insert(String::from("else"), Token::ELSE);
        keyword_map.insert(String::from("end"), Token::END);
        keyword_map.insert(String::from("for"), Token::FOR);
        keyword_map.insert(String::from("if"), Token::IF);
        keyword_map.insert(String::from("in"), Token::IN);
        keyword_map.insert(String::from("integer"), Token::INTEGER);
        keyword_map.insert(String::from("is"), Token::IS);
        keyword_map.insert(String::from("new"), Token::NEW);
        keyword_map.insert(String::from("not"), Token::NOT);
        keyword_map.insert(String::from("object"), Token::OBJECT);
        keyword_map.insert(String::from("or"), Token::OR);
        keyword_map.insert(String::from("print"), Token::PRINT);
        keyword_map.insert(String::from("procedure"), Token::PROCEDURE);
        keyword_map.insert(String::from("read"), Token::READ);
        keyword_map.insert(String::from("return"), Token::RETURN);
        keyword_map.insert(String::from("then"), Token::THEN);

        // separators
        let mut separators = HashSet::new();
        separators.insert(' ');
        separators.insert('\n');
        separators.insert('\t');
        separators.insert('\r');

        // handle separators
        let separator_continue_read = |c: char, scanner: &Scanner| {
            if scanner.is_separator(c) {
                ReadStatus::Continue
            } else {
                ReadStatus::Stop
            }
        };
        let noop_process_output = |_string: &str| Token::ADD; // arbitrary

        // handle constants
        let constant_continue_read = |c: char, _scanner: &Scanner| {
            if c.is_digit(10) {
                ReadStatus::Continue
            } else {
                ReadStatus::Stop
            }
        };
        let constant_process_output = |string: &str| {
            let Ok(const_val) = string.parse::<u16>() else {
                return Token::ERROR(format!("Constant {} doesn't fit in [{}, {}]", string, Scanner::CONST_MIN, Scanner::CONST_MAX));
            };

            if const_val < Scanner::CONST_MIN || const_val > Scanner::CONST_MAX {
                return Token::ERROR(format!("Constant {} doesn't fit in [{}, {}]", const_val, Scanner::CONST_MIN, Scanner::CONST_MAX));
            }

            return Token::CONST(const_val);
        };

        // handle identifiers
        let identifier_continue_read = |c: char, _scanner: &Scanner| {
            if !c.is_alphanumeric() {
                ReadStatus::Stop
            } else {
                ReadStatus::Continue
            }
        };
        let identifier_process_output = |string: &str| Token::ID(string.to_string());

        // handle strings
        let string_continue_read = |c: char, _scanner: &Scanner| {
            if c == Scanner::STRING_DELIMITER {
                ReadStatus::Stop
            } else {
                ReadStatus::Continue
            }
        };
        let string_process_output = |string: &str| Token::STRING(string.to_string());
        
        match File::open(file_path) {
            Ok(file) => {
                let mut scanner = Scanner {
                    reader: BufReader::new(file).bytes().peekable(),
                    curr_token: Token::ADD, // abitrary
                    separator_set: separators,
                    symbol_map: symbol_map,
                    keyword_map: keyword_map,

                    separator_continue_read: separator_continue_read,
                    noop_process_output: noop_process_output,
                    constant_continue_read: constant_continue_read,
                    constant_process_output: constant_process_output,
                    identifier_continue_read: identifier_continue_read,
                    identifier_process_output: identifier_process_output,
                    string_continue_read: string_continue_read,
                    string_process_output: string_process_output
                };

                scanner.next_token();
                Ok(scanner)
            },
            Err(error) => Err(error)
        }
    }
}