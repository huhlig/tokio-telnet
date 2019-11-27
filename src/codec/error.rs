//
// Copyright 2019 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::{error, fmt, io};

///
/// Decoding Errors
///
#[derive(Debug)]
pub enum DecodeError {
    IOError(io::Error),
    /// An unknown or invalid command was used
    UnknownCommand(u8),
}

impl error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DecodeError::IOError(inner) => Some(inner),
            DecodeError::UnknownCommand(_) => None,
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::IOError(inner) => {
                write!(f, "DecodeError::IOError({})", inner)
            }
            DecodeError::UnknownCommand(cmd) => {
                write!(f, "DecodeError::UnknownCommand({:#X})", cmd)
            }
        }
    }
}

impl From<io::Error> for DecodeError {
    fn from(error: io::Error) -> DecodeError {
        DecodeError::IOError(error)
    }
}

///
/// Telnet Encoding Errors
///
#[derive(Debug)]
pub enum EncodeError {
    IOError(io::Error),
}

impl error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            EncodeError::IOError(inner) => Some(inner),
        }
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodeError::IOError(inner) => {
                write!(f, "EncodeError::IOError({})", inner)
            }
        }
    }
}

impl From<io::Error> for EncodeError {
    fn from(error: io::Error) -> EncodeError {
        EncodeError::IOError(error)
    }
}
