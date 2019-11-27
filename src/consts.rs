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

/// Null
pub const NUL: u8 = 0;
/// Line Feed
pub const LF: u8 = 10;
/// Carriage Return
pub const CR: u8 = 13;
/// Negative Acknowledge
pub const NAK: u8 = 21;
/// End of File (EOF already used)
pub const EOF: u8 = 236;
/// Suspend Process
pub const SUSP: u8 = 237;
/// Abort Process
pub const ABORT: u8 = 238;
/// End of Record
pub const EOR: u8 = 239;
/// Subnegotiation End
pub const SE: u8 = 240;
/// No Operation
pub const NOP: u8 = 241;
/// Data Mark
pub const DM: u8 = 242;
/// Break
pub const BRK: u8 = 243;
/// Interrupt Process
pub const IP: u8 = 244;
/// Abort Output
pub const AO: u8 = 245;
/// Are you There
pub const AYT: u8 = 246;
/// Erase Character
pub const EC: u8 = 247;
/// Erase Line
pub const EL: u8 = 248;
/// Go Ahead
pub const GA: u8 = 249;
/// Subnegotation Begin
pub const SB: u8 = 250;
/// Will use this Option
pub const WILL: u8 = 251;
/// Won't use this Option
pub const WONT: u8 = 252;
/// Option Available
pub const DO: u8 = 253;
/// Option Not Available
pub const DONT: u8 = 254;
/// Interpret As Command
pub const IAC: u8 = 255;

/// Option Codes
pub mod option {
    /// Binary Option Code
    pub const BINARY: u8 = 0;
    /// Echo Option Code
    pub const ECHO: u8 = 1;
    /// Reconnection Option Code
    pub const RCP: u8 = 2;
    /// Supress Go Ahead Option Code
    pub const SGA: u8 = 3;
    /// Negotiate Approx Message Size Option Code
    pub const NAMS: u8 = 4;
    /// Terminal Option Status Option Code
    pub const STATUS: u8 = 5;
    /// Timing Mark Option Code
    pub const TM: u8 = 6;
    /// Remote Controlled Transmission and Echoing Option Code
    pub const RCTE: u8 = 7;
    /// Negotiate About Output Linewidth Option Code
    pub const NAOL: u8 = 8;
    /// Negotiate About Output Page Size Option Code
    pub const NAOP: u8 = 9;
    /// Negotiate About Output Carriage Return Disposition Option Code
    pub const NAOCRD: u8 = 10;
    /// Negotiate About Output Horizontal Tab Stops Option Code
    pub const NAOHTS: u8 = 11;
    /// Negotiate About utput Horizontal Tab Disposition Option Code
    pub const NAOHTD: u8 = 12;
    /// Negotiate About Output Form Feed Disposition Option Code
    pub const NAOFFD: u8 = 13;
    /// Negotiate About Output Vertical Tab Stops Option Code
    pub const NAOVTS: u8 = 14;
    /// Negotiate About Output Vertical Tab Disposition Option Code
    pub const NAOVTD: u8 = 15;
    /// Negotiate About Output Linefeed Disposition Option Code
    pub const NAOLFD: u8 = 16;
    /// Extended ASCII Option Code
    pub const XASCII: u8 = 17;
    /// Logout Option Code
    pub const LOGOUT: u8 = 18;
    /// Byte Macro Option Code
    pub const BM: u8 = 19;
    /// Data Entry Terminal Option Code
    pub const DET: u8 = 20;
    /// SUPDUP Option Code
    pub const SUPDUP: u8 = 21;
    /// SUPDUP Output Option Code
    pub const SUPDUP_OUTPUT: u8 = 22;
    /// Send Location Option Code
    pub const SNDLOC: u8 = 23;
    /// Terminal Type Option Code
    pub const TTYPE: u8 = 24;
    /// End of Record Option Code
    pub const EOR: u8 = 25;
    /// TACACS User Identification Option Code
    pub const TUID: u8 = 26;
    /// Output Marking Option Code
    pub const OUTMRK: u8 = 27;
    /// Terminal Location Number Option Code
    pub const TTYLOC: u8 = 28;
    /// Telnet 3270 Regime Option Code
    pub const OPT3270REGIME: u8 = 29;
    /// X.3 PAD Option Code
    pub const X3PAD: u8 = 30;
    /// Negotiate About Window Size Option Code
    pub const NAWS: u8 = 31;
    /// Terminal Speed Option Code
    pub const TSPEED: u8 = 32;
    /// Remote Line Flow Control Option Code
    pub const LFLOW: u8 = 33;
    /// Line Mode Option Code
    pub const LINEMODE: u8 = 34;
    /// X Display Location Option Code
    pub const XDISPLOC: u8 = 35;
    /// Environment Option Code
    pub const OLD_ENVIRONMENT: u8 = 36;
    /// Authentication Option Code
    pub const AUTHENTICATION: u8 = 37;
    /// Encryption Option Code
    pub const ENCRYPTION: u8 = 38;
    /// New Environment Option Code
    pub const NEW_ENVIRONMENT: u8 = 39;
    /// TN3270E Option Code
    pub const TN3270E: u8 = 40;
    /// XAUTH Option Code
    pub const XAUTH: u8 = 41;
    /// Character Set Option Code
    pub const CHARSET: u8 = 42;
    /// Telnet Remote Serial Port Option Code
    pub const TRSP: u8 = 43;
    /// COM Port Control Option Code
    pub const CPCO: u8 = 44;
    /// Telnet Supress Local Echo Option Code
    pub const TSLE: u8 = 45;
    /// Telnet Start TLS Option Code
    pub const START_TLS: u8 = 46;
    /// Kermit Option Code
    pub const KERMIT: u8 = 47;
    /// Send URL Option Code
    pub const SENDURL: u8 = 48;
    /// Forward X Option Code
    pub const FORWARDX: u8 = 49;
    /// Mud Server Data Protocol Option Code
    pub const MSDP: u8 = 69;
    /// Mud Server Status Protocol Option Code
    pub const MSSP: u8 = 70;
    /// Mud Client Compression Protocol v1 Option Code
    pub const COMPRESS1: u8 = 85;
    /// Mud Client Compression Protocol v2 Option Code
    pub const COMPRESS2: u8 = 86;
    /// Zenith Mud Protocol Option Code
    pub const ZMP: u8 = 93;
    /// Telnet Pragma Login Option Code
    pub const PRAGMA_LOGIN: u8 = 138;
    /// Telnet SSPI Login Option Code
    pub const SSPI_LOGIN: u8 = 139;
    /// Telnet Pragma Heartbeat Option Code
    pub const PRAGMA_HEARTBEAT: u8 = 140;
    /// Generic Mud Communication Protocol Option Code
    pub const GMCP: u8 = 201;
    /// Extended-Options-List Option Code
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

    ///
    /// Options this library supports.
    /// Set to True when implementation exists.
    /// (Local Support, Remote Support)
    ///
    pub static SUPPORT: [(bool, bool); 256] = [
        (false, false), //   0 - Binary
        (false, false), //   1 - Echo
        (false, false), //   2 - RCP
        (false, false), //   3 - SGA
        (false, false), //   4 - NAMS
        (false, false), //   5 - STATUS
        (false, false), //   6 - TM
        (false, false), //   7 - RCTE
        (false, false), //   8 - NAOL
        (false, false), //   9 - NAOP
        (false, false), //  10 - NAOCRD
        (false, false), //  11 - NAOHTS
        (false, false), //  12 - NAOHTD
        (false, false), //  13 - NAOFFD
        (false, false), //  14 - NAOVTS
        (false, false), //  15 - NAOVTD
        (false, false), //  16 - NAOLFD
        (false, false), //  17 - XASCII
        (false, false), //  18 - LOGOUT
        (false, false), //  19 - BM
        (false, false), //  20 - DET
        (false, false), //  21 - SUPDUP
        (false, false), //  22 - SUPDUP_OUTPUT
        (false, false), //  23 - SNDLOC
        (false, false), //  24 - TTYPE
        (false, false), //  25 - EOR
        (false, false), //  26 - TUID
        (false, false), //  27 - OUTMRK
        (false, false), //  28 - TTYLOC
        (false, false), //  29 - OPT3270REGIME
        (false, false), //  30 - X3PAD
        (false, false), //  31 - NAWS
        (false, false), //  32 - TSPEED
        (false, false), //  33 - LFLOW
        (false, false), //  34 - LINEMODE
        (false, false), //  35 - XDISPLOC
        (false, false), //  36 - OLD_ENVIRONMENT
        (false, false), //  37 - AUTHENTICATION
        (false, false), //  38 - ENCRYPTION
        (false, false), //  39 - NEW_ENVIRONMENT
        (false, false), //  40 - TN3270E
        (false, false), //  41 - XAUTH
        (false, false), //  42 - CHARSET
        (false, false), //  43 - TRSP
        (false, false), //  44 - CPCO
        (false, false), //  45 - TSLE
        (false, false), //  46 - START_TLS
        (false, false), //  47 - KERMIT
        (false, false), //  48 - SENDURL
        (false, false), //  49 - FORWARDX
        (false, false), //  50 -
        (false, false), //  51 -
        (false, false), //  52 -
        (false, false), //  53 -
        (false, false), //  54 -
        (false, false), //  55 -
        (false, false), //  56 -
        (false, false), //  57 -
        (false, false), //  58 -
        (false, false), //  59 -
        (false, false), //  60 -
        (false, false), //  61 -
        (false, false), //  62 -
        (false, false), //  63 -
        (false, false), //  64 -
        (false, false), //  65 -
        (false, false), //  66 -
        (false, false), //  67 -
        (false, false), //  68 -
        (false, false), //  69 - MSDP
        (false, false), //  70 - MSSP
        (false, false), //  71 -
        (false, false), //  72 -
        (false, false), //  73 -
        (false, false), //  74 -
        (false, false), //  75 -
        (false, false), //  76 -
        (false, false), //  77 -
        (false, false), //  78 -
        (false, false), //  79 -
        (false, false), //  80 -
        (false, false), //  81 -
        (false, false), //  82 -
        (false, false), //  83 -
        (false, false), //  84 -
        (false, false), //  85 - Compress1
        (false, false), //  86 - Compress2
        (false, false), //  87 -
        (false, false), //  88 -
        (false, false), //  89 -
        (false, false), //  90 -
        (false, false), //  91 -
        (false, false), //  92 -
        (false, false), //  93 - ZMP
        (false, false), //  94 -
        (false, false), //  95 -
        (false, false), //  96 -
        (false, false), //  97 -
        (false, false), //  98 -
        (false, false), //  99 -
        (false, false), // 100 -
        (false, false), // 101 -
        (false, false), // 102 -
        (false, false), // 103 -
        (false, false), // 104 -
        (false, false), // 105 -
        (false, false), // 106 -
        (false, false), // 107 -
        (false, false), // 108 -
        (false, false), // 109 -
        (false, false), // 110 -
        (false, false), // 111 -
        (false, false), // 112 -
        (false, false), // 113 -
        (false, false), // 114 -
        (false, false), // 115 -
        (false, false), // 116 -
        (false, false), // 117 -
        (false, false), // 118 -
        (false, false), // 119 -
        (false, false), // 120 -
        (false, false), // 121 -
        (false, false), // 122 -
        (false, false), // 123 -
        (false, false), // 124 -
        (false, false), // 125 -
        (false, false), // 126 -
        (false, false), // 127 -
        (false, false), // 128 -
        (false, false), // 129 -
        (false, false), // 130 -
        (false, false), // 131 -
        (false, false), // 132 -
        (false, false), // 133 -
        (false, false), // 134 -
        (false, false), // 135 -
        (false, false), // 136 -
        (false, false), // 137 -
        (false, false), // 138 - PRAGMA_LOGIN
        (false, false), // 139 - SSPI_LOGIN
        (false, false), // 140 - PRAGMA_HEARTBEAT
        (false, false), // 141 -
        (false, false), // 142 -
        (false, false), // 143 -
        (false, false), // 144 -
        (false, false), // 145 -
        (false, false), // 146 -
        (false, false), // 147 -
        (false, false), // 148 -
        (false, false), // 149 -
        (false, false), // 150 -
        (false, false), // 151 -
        (false, false), // 152 -
        (false, false), // 153 -
        (false, false), // 154 -
        (false, false), // 155 -
        (false, false), // 156 -
        (false, false), // 157 -
        (false, false), // 158 -
        (false, false), // 159 -
        (false, false), // 160 -
        (false, false), // 161 -
        (false, false), // 162 -
        (false, false), // 163 -
        (false, false), // 164 -
        (false, false), // 165 -
        (false, false), // 166 -
        (false, false), // 167 -
        (false, false), // 168 -
        (false, false), // 169 -
        (false, false), // 170 -
        (false, false), // 171 -
        (false, false), // 172 -
        (false, false), // 173 -
        (false, false), // 174 -
        (false, false), // 175 -
        (false, false), // 176 -
        (false, false), // 177 -
        (false, false), // 178 -
        (false, false), // 189 -
        (false, false), // 180 -
        (false, false), // 181 -
        (false, false), // 182 -
        (false, false), // 183 -
        (false, false), // 184 -
        (false, false), // 185 -
        (false, false), // 186 -
        (false, false), // 187 -
        (false, false), // 188 -
        (false, false), // 189 -
        (false, false), // 190 -
        (false, false), // 191 -
        (false, false), // 192 -
        (false, false), // 193 -
        (false, false), // 194 -
        (false, false), // 195 -
        (false, false), // 196 -
        (false, false), // 197 -
        (false, false), // 198 -
        (false, false), // 199 -
        (false, false), // 200 -
        (false, false), // 201 - GMCP
        (false, false), // 202 -
        (false, false), // 203 -
        (false, false), // 204 -
        (false, false), // 205 -
        (false, false), // 206 -
        (false, false), // 207 -
        (false, false), // 208 -
        (false, false), // 209 -
        (false, false), // 210 -
        (false, false), // 211 -
        (false, false), // 212 -
        (false, false), // 213 -
        (false, false), // 214 -
        (false, false), // 215 -
        (false, false), // 216 -
        (false, false), // 217 -
        (false, false), // 218 -
        (false, false), // 219 -
        (false, false), // 220 -
        (false, false), // 221 -
        (false, false), // 222 -
        (false, false), // 223 -
        (false, false), // 224 -
        (false, false), // 225 -
        (false, false), // 226 -
        (false, false), // 227 -
        (false, false), // 228 -
        (false, false), // 229 -
        (false, false), // 230 -
        (false, false), // 231 -
        (false, false), // 232 -
        (false, false), // 233 -
        (false, false), // 234 -
        (false, false), // 235 -
        (false, false), // 236 -
        (false, false), // 237 -
        (false, false), // 238 -
        (false, false), // 239 -
        (false, false), // 240 -
        (false, false), // 241 -
        (false, false), // 242 -
        (false, false), // 243 -
        (false, false), // 244 -
        (false, false), // 245 -
        (false, false), // 246 -
        (false, false), // 247 -
        (false, false), // 248 -
        (false, false), // 249 -
        (false, false), // 250 -
        (false, false), // 251 -
        (false, false), // 252 -
        (false, false), // 253 -
        (false, false), // 254 -
        (false, false), // 255 - EXOPL
    ];
}
