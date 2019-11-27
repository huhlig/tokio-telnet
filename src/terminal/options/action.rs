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

use super::consts;

///
/// Telnet Negotiation Action
///
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TelnetAction {
    /// Request performance of option from Remote to Local
    Do,
    /// Terminate performance of option from Remote to Local
    Dont,
    /// Request performance of option from Local to Remote
    Will,
    /// Terminate performance of option from Local to Remote
    Wont,
    /// No idea what we received
    Unknown(u8),
}

impl From<u8> for TelnetAction {
    fn from(byte: u8) -> Self {
        match byte {
            consts::DO => TelnetAction::Do,
            consts::DONT => TelnetAction::Dont,
            consts::WILL => TelnetAction::Will,
            consts::WONT => TelnetAction::Wont,
            value => TelnetAction::Unknown(value),
        }
    }
}

impl From<TelnetAction> for u8 {
    fn from(action: TelnetAction) -> Self {
        match action {
            TelnetAction::Do => consts::DO,
            TelnetAction::Dont => consts::DONT,
            TelnetAction::Will => consts::WILL,
            TelnetAction::Wont => consts::WONT,
            TelnetAction::Unknown(value) => value,
        }
    }
}
