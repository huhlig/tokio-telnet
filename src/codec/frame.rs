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

///
/// `TelnetFrame` is a single immutable Frame of data.
///
#[derive(Clone, Debug, PartialEq)]
pub enum TelnetFrame {
    /// Telnet Data
    Data(u8),
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
    /// Request to start using specified arguments.
    Do(TelnetOption),
    /// Demand to stop using specified arguments.
    Dont(TelnetOption),
    /// Agreement to use the specified arguments.
    Will(TelnetOption),
    /// Reject the proposed arguments.
    Wont(TelnetOption),
    /// Subnegotiation Payload
    Subnegotiate(TelnetOption, Vec<u8>),
}
