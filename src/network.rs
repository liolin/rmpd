use std::cmp::{Eq, PartialEq};
use std::io::prelude::*;

use log::info;

#[derive(Debug, Eq, PartialEq)]
pub struct Request {
    command: String,
    args: Option<Vec<String>>,
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request({}, {:?})", self.command, self.args)
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse<R: Read>(stream: &mut R) -> std::io::Result<Request> {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)?;

        info!("Request string from client: {}", buffer);

        let command = Parser::parse_command(&buffer);
        let args = Parser::parse_arguments(&buffer);

        let request = Request {
            command: command,
            args: args,
        };

        info!("Parsed request from string: {}", request);

        Ok(request)
    }

    fn parse_command(buffer: &String) -> String {
        // remove trailing \n and splits at every whitespace
        let v: Vec<&str> = buffer.trim_end().split(|c| c == ' ' || c == '\t').collect();
        v[0].to_string()
    }

    fn parse_arguments(buffer: &String) -> Option<Vec<String>> {
        let mut v = buffer
            .trim_end()
            .split(|c| c == ' ' || c == '\t')
            .collect::<Vec<&str>>();
        v.remove(0);

        let x: Vec<String> = v.iter().map(|c| c.clone().to_string()).collect();
        if x.len() == 0 {
            None
        } else {
            Some(x)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_without_argument() {
        let lhs = Request {
            command: "currentsong".to_string(),
            args: None,
        };

        let mut request_string = "currentsong\n".as_bytes();
        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }

    #[test]
    fn test_parse_with_single_argument() {
        simple_logger::init().unwrap();
        let lhs = Request {
            command: "setvol".to_string(),
            args: Some(vec!["50".to_string()]),
        };

        let mut request_string = "setvol 50\n".as_bytes();
        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }

    #[test]
    fn test_parse_with_multiple_arguments() {
        let lhs = Request {
            command: "moveid".to_string(),
            args: Some(vec!["2".to_string(), "5".to_string()]),
        };

        let mut request_string = "moveid 2 5\n".as_bytes();
        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }
}
