use std::cmp::{Eq, PartialEq};
use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;

use log::info;

#[derive(Debug, Eq, PartialEq)]
pub enum MPDCommand {
    ClearError,
    CurrentSong,
    Idle,
    Status,
    Stats,

    Consume,
    Crossfade,
    MixRampDb,
    MixRampDelay,
    Random,
    Repeate,
    Setvol,

    MoveId,
}

impl FromStr for MPDCommand {
    type Err = ParseMPDCommmandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "clearerror" => Ok(MPDCommand::ClearError),
            "currentsong" => Ok(MPDCommand::CurrentSong),
            "idle" => Ok(MPDCommand::Idle),
            "status" => Ok(MPDCommand::Status),
            "stats" => Ok(MPDCommand::Stats),

            "consume" => Ok(MPDCommand::Consume),
            "crossfade" => Ok(MPDCommand::Crossfade),
            "mixrampdb" => Ok(MPDCommand::MixRampDb),
            "mixrampdelay" => Ok(MPDCommand::MixRampDelay),
            "random" => Ok(MPDCommand::Random),
            "repeate" => Ok(MPDCommand::Repeate),
            "setvol" => Ok(MPDCommand::Setvol),

            "moveid" => Ok(MPDCommand::MoveId),
            _ => Err(ParseMPDCommmandError {}),
        }
    }
}

impl std::fmt::Display for MPDCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MPDCommand::ClearError => write!(f, "ClearError"),
            MPDCommand::CurrentSong => write!(f, "CurrentSong"),
            MPDCommand::Idle => write!(f, "Idle"),
            MPDCommand::Status => write!(f, "Status"),
            MPDCommand::Stats => write!(f, "Stats"),

            MPDCommand::Consume => write!(f, "Consume"),
            MPDCommand::Crossfade => write!(f, "Crossfade"),
            MPDCommand::MixRampDb => write!(f, "MixRampDb"),
            MPDCommand::MixRampDelay => write!(f, "MixRampDelay"),
            MPDCommand::Random => write!(f, "Random"),
            MPDCommand::Repeate => write!(f, "Repeate"),
            MPDCommand::Setvol => write!(f, "Setvol"),

	    MPDCommand::MoveId => write!(f, "MoveId"),
            _ => panic!("fmt not supportet for specifid value"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParseMPDCommmandError {}

impl Error for ParseMPDCommmandError {}

impl std::fmt::Display for ParseMPDCommmandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Request {
    command: MPDCommand,
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

    fn parse_command(buffer: &String) -> MPDCommand {
        // remove trailing \n and splits at every whitespace
        let v: Vec<&str> = buffer.trim_end().split(|c| c == ' ' || c == '\t').collect();
        MPDCommand::from_str(v[0]).unwrap()
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
            command: MPDCommand::CurrentSong,
            args: None,
        };

        let mut request_string = "currentsong\n".as_bytes();
        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }

    #[test]
    fn test_parse_with_single_argument() {
        simple_logger::init().unwrap();
        let lhs = Request {
            command: MPDCommand::Setvol,
            args: Some(vec!["50".to_string()]),
        };

        let mut request_string = "setvol 50\n".as_bytes();
        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }

    #[test]
    fn test_parse_with_multiple_arguments() {
        let lhs = Request {
            command: MPDCommand::MoveId,
            args: Some(vec!["2".to_string(), "5".to_string()]),
        };

        let mut request_string = "moveid 2 5\n".as_bytes();
        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }

    #[test]
    fn test_mpdcommand_from_str() {
        let values_as_string = vec![
            "ClearError",
            "CurrentSong",
            "Idle",
            "Status",
            "Stats",
            "Consume",
            "Crossfade",
            "MixRampDb",
            "MixRampDelay",
            "Random",
            "Repeate",
            "Setvol",
            "MoveId",
        ];

        for value in values_as_string.iter() {
            println!(
                "testing {} => {}",
                *value,
                MPDCommand::from_str(value).unwrap()
            );
            // println!("{}", MPDCommand::from_str(value).unwrap().to_string());
        }
    }
}
