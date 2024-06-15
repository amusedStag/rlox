use std::{error, fmt::{self, Display}};

#[derive(Debug)]
pub(crate) struct SyntaxError {
    found_at_char: char,
    line: u64,
}

impl SyntaxError {
    pub(crate)fn new(line: u64, found_at_char: char) -> Self {
	Self {
	    found_at_char,
	    line,
	}
    }
}

impl error::Error for SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Compilation error in line: {:?}", self.line)
    }
}

#[derive(Debug)]
pub(crate)enum Error {
    SyntaxErrors(Vec<SyntaxError>),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
	    Self::SyntaxErrors(errors) => {
		for e in errors {
		    write!(f, "[line: {}; at char: {}]\n", e.line, e.found_at_char)?
			
		}
		Ok(())
	    }
	}
    }
}
