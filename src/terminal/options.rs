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

use crate::codec::TelnetOption;
use crate::consts;
use crate::terminal::TerminalEndpoint;

///
/// Which Side of the Terminal
///
pub enum TerminalEndpoint {
    /// Local Terminal Side
    Local,
    /// Remote Terminal Side
    Remote,
}

///
/// Terminal Actions
///
pub enum TerminalAction {
    /// Sending Data
    Sending,
    /// Receiving Data
    Receiving,
}

///
/// State a `TerminalOption` can exist in.
///
pub enum TerminalOptionState {
    /// Option is unsupported by this library
    Unsupported,
    /// Option is supported by the library but disallowed by the user.
    Supported,
    /// Option is allowed by the user but not yet negotiated
    Allowed,
    /// Option is negotiated
    Enabled,
}

///
/// Terminal Options
///
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TerminalOption {
    /// Send Binary
    SendBinary,
    /// Receive Binary
    ReceiveBinary,
    /// Unknown Option
    Unknown(TerminalEndpoint, TelnetOption),
}

impl TerminalOption {
    pub fn endpoint(&self) -> TerminalEndpoint {
        match *self {
            TerminalOption::SendBinary => TerminalEndpoint::Local,
            TerminalOption::ReceiveBinary => TerminalEndpoint::Remote,
            TerminalOption::Unknown(endpoint, _) => endpoint,
        }
    }
    pub fn option(&self) -> TelnetOption {
        match *self {
            TerminalOption::SendBinary => TelnetOption::TransmitBinary,
            TerminalOption::ReceiveBinary => TerminalEndpoint::Remote,
            TerminalOption::Unknown(_, option) => option,
        }
    }
}
