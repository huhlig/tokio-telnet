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
/// Line Ending
pub const CRLF: [u8; 2] = [CR, LF];

pub mod option {
    pub const BINARY: u8 = 0;
    pub const ECHO: u8 = 1;
    pub const RCP: u8 = 2;
    pub const SGA: u8 = 3;
    pub const NAMS: u8 = 4;
    pub const STATUS: u8 = 5;
    pub const TM: u8 = 6;
    pub const RCTE: u8 = 7;
    pub const NAOL: u8 = 8;
    pub const NAOP: u8 = 9;
    pub const NAOCRD: u8 = 10;
    pub const NAOHTS: u8 = 11;
    pub const NAOHTD: u8 = 12;
    pub const NAOFFD: u8 = 13;
    pub const NAOVTS: u8 = 14;
    pub const NAOVTD: u8 = 15;
    pub const NAOLFD: u8 = 16;
    pub const XASCII: u8 = 17;
    pub const LOGOUT: u8 = 18;
    pub const BM: u8 = 19;
    pub const DET: u8 = 20;
    pub const SUPDUP: u8 = 21;
    pub const SUPDUP_OUTPUT: u8 = 22;
    pub const SNDLOC: u8 = 23;
    pub const TTYPE: u8 = 24;
    pub const EOR: u8 = 25;
    pub const TUID: u8 = 26;
    pub const OUTMRK: u8 = 27;
    pub const TTYLOC: u8 = 28;
    pub const OPT3270REGIME: u8 = 29;
    pub const X3PAD: u8 = 30;
    pub const NAWS: u8 = 31;
    pub const TSPEED: u8 = 32;
    pub const LFLOW: u8 = 33;
    pub const LINEMODE: u8 = 34;
    pub const XDISPLOC: u8 = 35;
    pub const OLD_ENVIRONMENT: u8 = 36;
    pub const AUTHENTICATION: u8 = 37;
    pub const ENCRYPTION: u8 = 38;
    pub const NEW_ENVIRONMENT: u8 = 39;
    pub const TN3270E: u8 = 40;
    pub const XAUTH: u8 = 41;
    pub const CHARSET: u8 = 42;
    pub const TRSP: u8 = 43;
    pub const CPCO: u8 = 44;
    pub const TSLE: u8 = 45;
    pub const START_TLS: u8 = 46;
    pub const KERMIT: u8 = 47;
    pub const SENDURL: u8 = 48;
    pub const FORWARDX: u8 = 49;
    pub const MSDP: u8 = 69;
    pub const MSSP: u8 = 70;
    pub const COMPRESS1: u8 = 85;
    pub const COMPRESS2: u8 = 86;
    pub const ZMP: u8 = 93;
    pub const PRAGMA_LOGIN: u8 = 138;
    pub const SSPI_LOGIN: u8 = 139;
    pub const PRAGMA_HEARTBEAT: u8 = 140;
    pub const EXOPL: u8 = 255;

    /// Charset Subnegotiation Tokens
    pub mod charset {
        pub const REQUEST: u8 = 1;
        pub const ACCEPTED: u8 = 02;
        pub const REJECTED: u8 = 03;
        pub const TTABLE_IS: u8 = 04;
        pub const TTABLE_REJECTED: u8 = 05;
        pub const TTABLE_ACK: u8 = 06;
        pub const TTABLE_NAK: u8 = 07;
    }

    pub mod naocrd {
        pub const DR: u8 = 0;
        pub const DS: u8 = 1;
    }

    pub mod new_environ {
        pub const VAR: u8 = 0;
        pub const VALUE: u8 = 1;
        pub const ESC: u8 = 2;
        pub const USERVAR: u8 = 3;
    }

    pub mod msdp {
        pub const VAR: u8 = 1;
        pub const VAL: u8 = 2;
        pub const TABLE_OPEN: u8 = 3;
        pub const TABLE_CLOSE: u8 = 4;
        pub const ARRAY_OPEN: u8 = 5;
        pub const ARRAY_CLOSE: u8 = 6;
    }

    pub mod mssp {
        pub const VAR: u8 = 1;
        pub const VAL: u8 = 2;
    }

    pub mod status {
        /// Subnegotiation IS command.
        pub const IS: u8 = 0;
        /// Subnegotiation SEND command.
        pub const SEND: u8 = 1;
    }
}
