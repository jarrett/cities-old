use std;
use std::convert::From;
use freetype;

use futil::IoErrorLine;

#[derive(Debug)]
pub enum GameError {
    UnknownError,
    IoError(std::io::Error),
    IoErrorLine(IoErrorLine),
    FreeTypeError(freetype::error::Error),
    ImageBoundsError(String)
}

impl From<std::io::Error> for GameError {
    fn from(e: std::io::Error) -> GameError {
        GameError::IoError(e)
    }
}

impl From<freetype::error::Error> for GameError {
    fn from(e: freetype::error::Error) -> GameError {
        GameError::FreeTypeError(e)
    }
}

impl From<IoErrorLine> for GameError {
    fn from(e: IoErrorLine) -> GameError {
        GameError::IoErrorLine(e)
    }
}