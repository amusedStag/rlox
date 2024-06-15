use crate::{error::SyntaxError, token::Token};

pub(crate) fn scan_tokens(source: String)
			  -> Result<Vec<Token>, Vec<SyntaxError>> {

    let mut char_indices = source.char_indices().peekable();
    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    let mut line = 1;

    while let Some((pos, ch)) = char_indices.next() {
	let token = match ch {
	    '(' => Token::LEFT_PAREN,
	    ')' => Token::RIGHT_PAREN,
	    '{' => Token::LEFT_BRACE,
	    '}' => Token::RIGHT_BRACE,
	    ',' => Token::COMMA,
	    '.' => Token::DOT,
	    '-' => Token::MINUS,
	    '+' => Token::PLUS,
	    ';' => Token::SEMICOLON,
	    '*' => Token::STAR,
	    '!' => {
		match char_indices.next_if_eq(&(pos+1, '=')) {
		    
		    Some(_equals) => Token::BANG_EQUAL,
		    None => {
			errors.push(SyntaxError::new(line, ch));
			continue
		    },
		}
	    },
	    '=' => {
		match char_indices.next_if_eq(&(pos+1, '=')) {
		    Some(_equals) => Token::EQUAL_EQUAL,
		    None => Token::EQUAL,
		}
	    },
	    '<' => {
		match char_indices.next_if_eq(&(pos+1, '=')) {
		    Some(_equals) => Token::LESS_EQUAL,
		    None => Token::LESS,
		}
	    },
	    '>' => {
		match char_indices.next_if_eq(&(pos+1, '=')) {
		    Some(_equals) => Token::GREATER_EQUAL,
		    None => Token::GREATER,
		}
	    },
	    '"' => {
		let s: String = char_indices
		    .by_ref()
		    .take_while(|(_pos, c)| { *c != '"' })
		    .map(|(_pos, c)| { c })
		    .collect();

		Token::STRING(s)
	    }
	    '\n' => { line+=1; continue}
	    _ if ch.is_whitespace() => continue,
	    _ => {
		errors.push(SyntaxError::new(line, ch));
		continue
	    }
	};
	tokens.push(token);
    }

    if errors.is_empty() {
	return Ok(tokens);
    }
    Err(errors)
}
