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

use super::option::TerminalOption;
use crate::terminal::{OptionStatus, TerminalEndpoint};

///
/// Output generated from the Telnet Network Virtual Terminal
///
#[derive(Clone, Debug)]
pub enum TerminalOutput {
    /// No Operation
    NoOperation,
    ///End of urgent Data Stream
    DataMark,
    /// Operator pressed the Break key or the Attention key.
    Break,
    /// Interrupt current process.
    InterruptProcess,
    /// Cancel output from current process.
    AbortOutput,
    /// Request acknowledgment.
    AreYouThere,
    /// Request that operator erase the previous character.
    EraseCharacter,
    /// Request that operator erase the previous line.
    EraseLine,
    /// End of input for half-duplex connections.
    GoAhead,
    /// Terminal Received Line of Ascii
    AsciiData(String),
    /// Terminal Received Binary Data
    BinaryData(Vec<u8>),
    /// Current Status of option. May have changed
    TerminalOptionStatus(TerminalEndpoint, TerminalOption, OptionStatus),
    /// Received Option Argument
    TerminalOptionArgument(TerminalEndpoint, TerminalOptionArgument),
}
