use std::fmt;

use crate::{scanner::Scanner, token::Token};

// same list for formal parameters and call arguments
pub enum Parameters {
    Id(String),
    Seq(String, Box<Parameters>)
}

impl Parameters {
    pub fn new(s: &mut Scanner) -> Result<Box<Self>, String> {
        let Token::ID(id) = s.current_token() else {
            return Err(format!("Expected ID in parameter list, got {:?}", s.current_token()));
        };
        s.next_token();

        Ok(Box::new(
            // a comma means there's at least one more ID
            if s.current_token() == Token::COMMA {
                // consume the comma
                s.next_token();
                Parameters::Seq(id, Parameters::new(s)?)
            }
            else {
                Parameters::Id(id)
            }
        ))
    }

    // flatten the list so callers can pair formals with arguments
    pub fn ids(&self) -> Vec<&str> {
        match self {
            Parameters::Id(id) => vec![id.as_str()],
            Parameters::Seq(id, params) => {
                let mut ids = vec![id.as_str()];
                ids.extend(params.ids());
                ids
            }
        }
    }
}

// print the IDs separated by commas
impl fmt::Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Parameters::Id(id) => write!(f, "{id}"),
            Parameters::Seq(id, params) => write!(f, "{id}, {params}")
        }
    }
}
