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
    Single,
    ReplayGainMode,
    ReplayGainStatus,
    Volume,

    Next,
    Pause,
    Play,
    PlayId,
    Previous,
    Seek,
    SeekId,
    SeekCur,
    Stop,

    Add,
    AddId,
    Clear,
    Delete,
    DeleteId,
    Move,
    MoveId,
    // Playlist // deprecated
    PlaylistFind,
    PlaylistId,
    PlaylistInfo,
    PlaylistSearch,
    PlChanges,
    PlChangesPosId,
    Prio,
    PrioId,
    RangeId,
    Shuffle,
    Swap,
    SwapId,
    AddTagId,
    ClearTagId,
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
	    "single" => Ok(MPDCommand::Single),
	    "replaygainmode" => Ok(MPDCommand::ReplayGainMode),
	    "replaygainstatus" => Ok(MPDCommand::ReplayGainStatus),
	    "volume" => Ok(MPDCommand::Volume),

	    "next" => Ok(MPDCommand::Next),
	    "pause" => Ok(MPDCommand::Pause),
	    "play" => Ok(MPDCommand::Play),
	    "playid" => Ok(MPDCommand::PlayId),
	    "previous" => Ok(MPDCommand::Previous),
	    "seek" => Ok(MPDCommand::Seek),
	    "seekid" => Ok(MPDCommand::SeekId),
	    "seekcur" => Ok(MPDCommand::SeekCur),
	    "stop" => Ok(MPDCommand::Stop),

	    "add" => Ok(MPDCommand::Add),
	    "addid" => Ok(MPDCommand::AddId),
	    "clear" => Ok(MPDCommand::Clear),
	    "delete" => Ok(MPDCommand::Delete),
	    "deleteid" => Ok(MPDCommand::DeleteId),
	    "move" => Ok(MPDCommand::Move),
	    "moveid" => Ok(MPDCommand::MoveId),
	    // Playlist // deprecated
	    "playlistfind" => Ok(MPDCommand::PlaylistFind),
	    "playlistid" => Ok(MPDCommand::PlaylistId),
	    "playlistinfo" => Ok(MPDCommand::PlaylistInfo),
	    "playlistsearch" => Ok(MPDCommand::PlaylistSearch),
	    "plchanges" => Ok(MPDCommand::PlChanges),
	    "plchangesposid" => Ok(MPDCommand::PlChangesPosId),
	    "prio" => Ok(MPDCommand::Prio),
	    "prioid" => Ok(MPDCommand::PrioId),
	    "rangeid" => Ok(MPDCommand::RangeId),
	    "shuffle" => Ok(MPDCommand::Shuffle),
	    "swap" => Ok(MPDCommand::Swap),
	    "swapid" => Ok(MPDCommand::SwapId),
	    "addtagid" => Ok(MPDCommand::AddTagId),
	    "cleartagid" => Ok(MPDCommand::ClearTagId),
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
	    MPDCommand::Single => write!(f, "Single"),
	    MPDCommand::ReplayGainMode => write!(f, "ReplayGainMode"),
	    MPDCommand::ReplayGainStatus => write!(f, "ReplayGainStatus"),
	    MPDCommand::Volume => write!(f, "Volume"),

	    MPDCommand::Next => write!(f, "Next"),
	    MPDCommand::Pause => write!(f, "Pause"),
	    MPDCommand::Play => write!(f, "Play"),
	    MPDCommand::PlayId => write!(f, "PlayId"),
	    MPDCommand::Previous => write!(f, "Previous"),
	    MPDCommand::Seek => write!(f, "Seek"),
	    MPDCommand::SeekId => write!(f, "SeekId"),
	    MPDCommand::SeekCur => write!(f, "SeekCur"),
	    MPDCommand::Stop => write!(f, "Stop"),

	    MPDCommand::Add => write!(f, "Add"),
	    MPDCommand::AddId => write!(f, "AddId"),
	    MPDCommand::Clear => write!(f, "Clear"),
	    MPDCommand::Delete => write!(f, "Delete"),
	    MPDCommand::DeleteId => write!(f, "DeleteId"),
	    MPDCommand::Move => write!(f, "Move"),
	    MPDCommand::MoveId => write!(f, "MoveId"),
	    // MPDCommand::Playlist // deprecate => write!(f, "Playlist")d
	    MPDCommand::PlaylistFind => write!(f, "PlaylistFind"),
	    MPDCommand::PlaylistId => write!(f, "PlaylistId"),
	    MPDCommand::PlaylistInfo => write!(f, "PlaylistInfo"),
	    MPDCommand::PlaylistSearch => write!(f, "PlaylistSearch"),
	    MPDCommand::PlChanges => write!(f, "PlChanges"),
	    MPDCommand::PlChangesPosId => write!(f, "PlChangesPosId"),
	    MPDCommand::Prio => write!(f, "Prio"),
	    MPDCommand::PrioId => write!(f, "PrioId"),
	    MPDCommand::RangeId => write!(f, "RangeId"),
	    MPDCommand::Shuffle => write!(f, "Shuffle"),
	    MPDCommand::Swap => write!(f, "Swap"),
	    MPDCommand::SwapId => write!(f, "SwapId"),
	    MPDCommand::AddTagId => write!(f, "AddTagId"),
	    MPDCommand::ClearTagId => write!(f, "ClearTagId"),
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
	    "Single",
	    "ReplayGainMode",
	    "ReplayGainStatus",
	    "Volume",
	    "Next",
	    "Pause",
	    "Play",
	    "PlayId",
	    "Previous",
	    "Seek",
	    "SeekId",
	    "SeekCur",
	    "Stop",
	    "Add",
	    "AddId",
	    "Clear",
	    "Delete",
	    "DeleteId",
	    "Move",
	    "MoveId",
	    // "Playlist" // deprecated
	    "PlaylistFind",
	    "PlaylistId",
	    "PlaylistInfo",
	    "PlaylistSearch",
	    "PlChanges",
	    "PlChangesPosId",
	    "Prio",
	    "PrioId",
	    "RangeId",
	    "Shuffle",
	    "Swap",
	    "SwapId",
	    "AddTagId",
	    "ClearTagId",
        ];

        for value in values_as_string.iter() {
            assert_eq!(
                *value,
                MPDCommand::from_str(value).unwrap().to_string()
            );
        }
    }

    #[test]
    fn test_parse_with_command_list() {
	// command_list_begin / command_list_ok_begin
	//     volume 86
	//     play 10240
	//     status
	// command_list_end
    }
}
