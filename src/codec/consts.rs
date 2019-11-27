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
// distributed under the License is distributed on an "AS IS" BASIS;
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

/// Interpret As Command
pub const IAC: u8 = 255;
/// Option Not Available
pub const DONT: u8 = 254;
/// Option Available
pub const DO: u8 = 253;
/// Won't use this Option
pub const WONT: u8 = 252;
/// Will use this Option
pub const WILL: u8 = 251;
/// Subnegotation Begin
pub const SB: u8 = 250;
/// Go Ahead
pub const GA: u8 = 249;
/// Erase Line
pub const EL: u8 = 248;
/// Erase Character
pub const EC: u8 = 247;
/// Are you There
pub const AYT: u8 = 246;
/// Abort Output
pub const AO: u8 = 245;
/// Interrupt Process
pub const IP: u8 = 244;
/// Break
pub const BRK: u8 = 243;
/// Data Mark
pub const DM: u8 = 242;
/// No Operation
pub const NOP: u8 = 241;
/// Subnegotiation End
pub const SE: u8 = 240;
/// End of Record
pub const EOR: u8 = 239;
/// Abort Process
pub const ABORT: u8 = 238;
/// Suspend Process
pub const SUSP: u8 = 237;
/// End of File (EOF already used)
pub const EOF: u8 = 236;
/// Negative Acknowledge
pub const NAK: u8 = 21;
/// Carriage Return
pub const CR: u8 = 13;
/// Line Feed
pub const LF: u8 = 10;
/// Null
pub const NUL: u8 = 0;
