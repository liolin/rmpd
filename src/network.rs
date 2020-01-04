use std::cmp::{Eq, PartialEq};
use std::io::prelude::*;

use log::info;

#[derive(Debug, Eq, PartialEq)]
pub struct MPDCommand {
    command: String,
    args: Option<Vec<String>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Request {
    commands: Vec<MPDCommand>,
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Request({:?})", self.commands)
    }
}

pub struct Parser {}

impl Parser {
    pub fn parse<R: Read>(stream: &mut R) -> std::io::Result<Request> {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)?;

        info!("Request string from client: {}", buffer);

        let commands = Parser::parse_command_list(&buffer);
        let request = Request { commands };

        info!("Parsed request from string: {}", request);

        Ok(request)
    }

    fn parse_command_list(buffer: &str) -> Vec<MPDCommand> {
        let mut all_commands = Vec::new();
        let lines: Vec<&str> = buffer
            .trim_end()
            .split('\n')
            .collect();

        if lines[0] == "command_list_begin" {
            for line in lines {
                let command = Parser::parse_command(line);
                let args = Parser::parse_arguments(line);
                let command = MPDCommand { command, args };
                all_commands.push(command);
            }
        } else {
            let command = Parser::parse_command(buffer);
            let args = Parser::parse_arguments(buffer);
            let command = MPDCommand { command, args };
            all_commands.push(command);
        };

        all_commands
    }

    fn parse_command(buffer: &str) -> String {
        // remove trailing \n and splits at every whitespace
        let v: Vec<&str> = buffer.trim_end().split(|c| c == ' ' || c == '\t').collect();
        v[0].to_string()
    }

    fn parse_arguments(buffer: &str) -> Option<Vec<String>> {
        let mut v = buffer
            .trim_end()
            .split(|c| c == ' ' || c == '\t')
            .collect::<Vec<&str>>();
        v.remove(0);

        let x: Vec<String> = v.iter().map(|c| c.to_string()).collect();
        if x.is_empty() {
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
    fn test_parse_command_without_argument() {
        let mpd_command = MPDCommand {
            command: "currentsong".to_string(),
            args: None,
        };

        let lhs = Request {
            commands: vec![mpd_command],
        };

        let mut request_string = "currentsong\n".as_bytes();
        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }

    #[test]
    fn test_parse_with_single_argument() {
        let mpd_command = MPDCommand {
            command: "setvol".to_string(),
            args: Some(vec!["50".to_string()]),
        };

        let lhs = Request {
            commands: vec![mpd_command],
        };

        let mut request_string = "setvol 50\n".as_bytes();
        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }

    #[test]
    fn test_parse_with_multiple_arguments() {
        let mpd_command = MPDCommand {
            command: "moveid".to_string(),
            args: Some(vec!["2".to_string(), "5".to_string()]),
        };

        let lhs = Request {
            commands: vec![mpd_command],
        };

        let mut request_string = "moveid 2 5\n".as_bytes();
        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }

    #[test]
    fn test_parse_with_command_list() {
        let mut request_string = r#"command_list_begin
volume 86
moveid 2 5
command_list_end
"#
        .as_bytes();

        let mpd_command_0 = MPDCommand {
            command: "command_list_begin".to_string(),
            args: None,
        };

        let mpd_command_1 = MPDCommand {
            command: "volume".to_string(),
            args: Some(vec!["86".to_string()]),
        };

        let mpd_command_2 = MPDCommand {
            command: "moveid".to_string(),
            args: Some(vec!["2".to_string(), "5".to_string()]),
        };

        let mpd_command_3 = MPDCommand {
            command: "command_list_end".to_string(),
            args: None,
        };

        let lhs = Request {
            commands: vec![mpd_command_0, mpd_command_1, mpd_command_2, mpd_command_3],
        };

        assert_eq!(lhs, Parser::parse(&mut request_string).unwrap());
    }
}
