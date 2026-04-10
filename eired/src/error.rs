use std::io;
use std::process;

use crossbeam::channel::SendError;

use crate::runtime::RuntimeTask;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Io(io::Error),
    Send(SendError<RuntimeTask>),
}

pub(crate) fn handle_err(err: Error) {
    match err {
        Error::Io(_) => process::exit(5),
        Error::Send(_) => process::exit(1),
    }
}
