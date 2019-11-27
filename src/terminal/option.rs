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
/// [Telnet Options](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml)
///
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TelnetOption {
    /// Telnet Binary Transmission [RFC856](https://tools.ietf.org/html/rfc856)
    /// Note: Enable 8 bit data path
    TransmitBinary,
    /// Telnet Echo Option [RFC857](https://tools.ietf.org/html/rfc857)
    Echo,
    /// Telnet Reconnection Option [DDN Protocol Handbook, "Telnet Reconnection Option", NIC 50005, December 1985.]()
    /// Note: Prepare to reconnect
    Reconnection,
    /// Supress Go ahead [RFC858](https://tools.ietf.org/html/rfc858)
    SuppressGoAhead,
    /// Approximate Message Size Negotiation
    ApproxMessageSizeNegotiation,
    /// Telnet Status Option [RFC859](http://www.iana.org/go/rfc859)
    Status,
    /// Telnet Timing Mark Option [RFC860](http://www.iana.org/go/rfc860)
    TimingMark,
    /// Remote Controlled Trans and Echo [RFC726](http://www.iana.org/go/rfc726)
    RCTE,
    /// Output Line Width [DDN Protocol Handbook, "Telnet Output Line Width Option", NIC 50005, December 1985.]()
    OutLineWidth,
    /// Output Page Size [DDN Protocol Handbook, "Telnet Output Page Size Option", NIC 50005, December 1985.]()
    OutPageSize,
    /// Output Carriage-Return Disposition [RFC652](http://www.iana.org/go/rfc652)
    NAOCRD,
    /// Output Horizontal Tab Stops [RFC653](http://www.iana.org/go/rfc653)
    NAOHTS,
    /// Output Horizontal Tab Disposition [RFC654](http://www.iana.org/go/rfc654)
    NAOHTD,
    /// Output Formfeed Disposition [RFC655](http://www.iana.org/go/rfc655)
    NAOFFD,
    /// Output Vertical Tabstops [RFC656](http://www.iana.org/go/rfc656)
    NAOVTS,
    /// Output Vertical Tab Disposition [RFC657](http://www.iana.org/go/rfc657)
    NAOVTD,
    /// Output Linefeed Disposition [RFC658](http://www.iana.org/go/rfc658)
    NAOLFD,
    /// Extended ASCII [RFC698](http://www.iana.org/go/rfc698)
    XASCII,
    /// Logout Option [RFC727](http://www.iana.org/go/rfc727)
    Logout,
    /// Byte Macro [RFC735](http://www.iana.org/go/rfc735)
    ByteMacro,
    /// Data Entry Terminal [RFC1043](http://www.iana.org/go/rfc1043) [RFC732](http://www.iana.org/go/rfc732)
    DET,
    /// SUPDUP [RFC736](http://www.iana.org/go/rfc736) [RFC734](http://www.iana.org/go/rfc734)
    SUPDUP,
    /// SUPDUP Output [RFC749](http://www.iana.org/go/rfc749)
    SUPDUPOutput,
    /// Send Location [RFC779](http://www.iana.org/go/rfc779)
    SNDLOC,
    /// Terminal Type [RFC1091](http://www.iana.org/go/rfc1091)
    TTYPE,
    /// End of Record [RFC885](http://www.iana.org/go/rfc885)
    EOR,
    /// TACACS User Identification [RFC927](http://www.iana.org/go/rfc927)
    TUID,
    /// Output Marking [RFC933](http://www.iana.org/go/rfc933)
    OUTMRK,
    /// Terminal Location Number [RFC946](http://www.iana.org/go/rfc946)
    TTYLOC,
    /// Telnet 3270 Regime [RFC1041](http://www.iana.org/go/rfc1041)
    OPT3270Regime,
    /// X.3 PAD [RFC1053](http://www.iana.org/go/rfc1053)
    X3PAD,
    /// Negotiate About Window Size [RFC1073](http://www.iana.org/go/rfc1073)
    NAWS,
    /// Terminal Speed [RFC1079](http://www.iana.org/go/rfc1079)
    TSPEED,
    /// Remote Flow Control [RFC1372](http://www.iana.org/go/rfc1372)
    LFLOW,
    /// Linemode [RFC1184](http://www.iana.org/go/rfc1184)
    Linemode,
    /// X Display Location [RFC1096](http://www.iana.org/go/rfc1096)
    XDISPLOC,
    /// Environment Option [RFC1408](http://www.iana.org/go/rfc1408)
    Environment,
    /// Authentication Option [RFC2941](http://www.iana.org/go/rfc2941)
    Authentication,
    /// Encryption Option [RFC2946](http://www.iana.org/go/rfc2946)
    Encryption,
    /// New Environment Option [RFC1572](http://www.iana.org/go/rfc1572)
    NewEnvironment,
    /// TN3270E [RFC2355](http://www.iana.org/go/rfc2355)
    TN3270E,
    /// XAUTH [Rob_Earhart](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Rob_Earhart)
    XAUTH,
    /// Charset [RFC2066](http://www.iana.org/go/rfc2066)
    Charset,
    /// Telnet Remote Serial Port (RSP)	[Robert_Barnes](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Robert_Barnes)
    TRSP,
    /// Com Port Control Option	[RFC2217](http://www.iana.org/go/rfc2217)
    CPCO,
    /// Telnet Suppress Local Echo	[Wirt_Atmar](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Wirt_Atmar)
    TSLE,
    /// Telnet Start TLS [Michael_Boe](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Michael_Boe)
    StartTLS,
    /// Kermit [RFC2840](http://www.iana.org/go/rfc2840)
    Kermit,
    /// SEND-URL [David_Croft](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#David_Croft)
    SendUrl,
    /// FORWARD_X [Jeffrey_Altman](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Jeffrey_Altman)
    ForwardX,
    /// Mud Server Data Protocol [MSDP](https://tintin.sourceforge.io/protocols/msdp/)
    MSDP,
    /// Mud Server Status Protocol [MSSP](https://tintin.sourceforge.io/protocols/mssp/)
    MSSP,
    /// Mud Client Compression Protocol version 1 [MCCPv1](http://www.gammon.com.au/mccp/protocol.html)
    Compress1,
    /// Mud Client Compression Protocol version 2 [MCCPv2](https://tintin.sourceforge.io/protocols/mccp/)
    Compress2,
    /// Zenith Mud Protocol [ZMP](http://discworld.starturtle.net/external/protocols/zmp.html)
    ZMP,
    /// Telnet Option Pragma Logon [Steve_McGregory](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Steve_McGregory)
    PragmaLogon,
    /// Telnet Option SSPI Logon [Steve_McGregory](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Steve_McGregory)
    SSPILogon,
    /// Telnet Option Pragma Heartbeat [Steve_McGregory](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Steve_McGregory)
    PragmaHeartbeat,
    /// Extended-Options-List [RFC861](http://www.iana.org/go/rfc861)
    EXOPL,
    /// Unknown
    Unknown(u8),
}

impl TelnetOption {
    ///
    /// Does this Library Support this option.
    ///
    pub fn support_local(&self) -> bool {
        match self {
            TelnetOption::TransmitBinary => true,
            _ => false,
        }
    }
    ///
    /// Does this Library Support this option.
    ///
    pub fn support_remote(&self) -> bool {
        match self {
            TelnetOption::TransmitBinary => true,
            _ => false,
        }
    }
}

impl From<u8> for TelnetOption {
    fn from(byte: u8) -> Self {
        match byte {
            consts::option::BINARY => TelnetOption::TransmitBinary,
            consts::option::ECHO => TelnetOption::Echo,
            consts::option::RCP => TelnetOption::Reconnection,
            consts::option::SGA => TelnetOption::SuppressGoAhead,
            consts::option::NAMS => TelnetOption::ApproxMessageSizeNegotiation,
            consts::option::STATUS => TelnetOption::Status,
            consts::option::TM => TelnetOption::TimingMark,
            consts::option::RCTE => TelnetOption::RCTE,
            consts::option::NAOL => TelnetOption::OutLineWidth,
            consts::option::NAOP => TelnetOption::OutPageSize,
            consts::option::NAOCRD => TelnetOption::NAOCRD,
            consts::option::NAOHTS => TelnetOption::NAOHTS,
            consts::option::NAOHTD => TelnetOption::NAOHTD,
            consts::option::NAOFFD => TelnetOption::NAOFFD,
            consts::option::NAOVTS => TelnetOption::NAOVTS,
            consts::option::NAOVTD => TelnetOption::NAOVTD,
            consts::option::NAOLFD => TelnetOption::NAOLFD,
            consts::option::XASCII => TelnetOption::XASCII,
            consts::option::LOGOUT => TelnetOption::Logout,
            consts::option::BM => TelnetOption::ByteMacro,
            consts::option::DET => TelnetOption::DET,
            consts::option::SUPDUP => TelnetOption::SUPDUP,
            consts::option::SUPDUP_OUTPUT => TelnetOption::SUPDUPOutput,
            consts::option::SNDLOC => TelnetOption::SNDLOC,
            consts::option::TTYPE => TelnetOption::TTYPE,
            consts::option::EOR => TelnetOption::EOR,
            consts::option::TUID => TelnetOption::TUID,
            consts::option::OUTMRK => TelnetOption::OUTMRK,
            consts::option::TTYLOC => TelnetOption::TTYLOC,
            consts::option::OPT3270REGIME => TelnetOption::OPT3270Regime,
            consts::option::X3PAD => TelnetOption::X3PAD,
            consts::option::NAWS => TelnetOption::NAWS,
            consts::option::TSPEED => TelnetOption::TSPEED,
            consts::option::LFLOW => TelnetOption::LFLOW,
            consts::option::LINEMODE => TelnetOption::Linemode,
            consts::option::XDISPLOC => TelnetOption::XDISPLOC,
            consts::option::OLD_ENVIRONMENT => TelnetOption::Environment,
            consts::option::AUTHENTICATION => TelnetOption::Authentication,
            consts::option::ENCRYPTION => TelnetOption::Encryption,
            consts::option::NEW_ENVIRONMENT => TelnetOption::NewEnvironment,
            consts::option::TN3270E => TelnetOption::TN3270E,
            consts::option::XAUTH => TelnetOption::XAUTH,
            consts::option::CHARSET => TelnetOption::Charset,
            consts::option::TRSP => TelnetOption::TRSP,
            consts::option::CPCO => TelnetOption::CPCO,
            consts::option::TSLE => TelnetOption::TSLE,
            consts::option::START_TLS => TelnetOption::StartTLS,
            consts::option::KERMIT => TelnetOption::Kermit,
            consts::option::SENDURL => TelnetOption::SendUrl,
            consts::option::FORWARDX => TelnetOption::ForwardX,
            consts::option::MSDP => TelnetOption::MSDP,
            consts::option::MSSP => TelnetOption::MSSP,
            consts::option::COMPRESS1 => TelnetOption::Compress1,
            consts::option::COMPRESS2 => TelnetOption::Compress2,
            consts::option::ZMP => TelnetOption::ZMP,
            consts::option::EXOPL => TelnetOption::EXOPL,
            consts::option::PRAGMA_LOGIN => TelnetOption::PragmaLogon,
            consts::option::SSPI_LOGIN => TelnetOption::SSPILogon,
            consts::option::PRAGMA_HEARTBEAT => TelnetOption::PragmaHeartbeat,
            byte => TelnetOption::Unknown(byte),
        }
    }
}

impl From<TelnetOption> for u8 {
    fn from(option: TelnetOption) -> Self {
        match option {
            TelnetOption::TransmitBinary => consts::option::BINARY,
            TelnetOption::Echo => consts::option::ECHO,
            TelnetOption::Reconnection => consts::option::RCP,
            TelnetOption::SuppressGoAhead => consts::option::SGA,
            TelnetOption::ApproxMessageSizeNegotiation => consts::option::NAMS,
            TelnetOption::Status => consts::option::STATUS,
            TelnetOption::TimingMark => consts::option::TM,
            TelnetOption::RCTE => consts::option::RCTE,
            TelnetOption::OutLineWidth => consts::option::NAOL,
            TelnetOption::OutPageSize => consts::option::NAOP,
            TelnetOption::NAOCRD => consts::option::NAOCRD,
            TelnetOption::NAOHTS => consts::option::NAOHTS,
            TelnetOption::NAOHTD => consts::option::NAOHTD,
            TelnetOption::NAOFFD => consts::option::NAOFFD,
            TelnetOption::NAOVTS => consts::option::NAOVTS,
            TelnetOption::NAOVTD => consts::option::NAOVTD,
            TelnetOption::NAOLFD => consts::option::NAOLFD,
            TelnetOption::XASCII => consts::option::XASCII,
            TelnetOption::Logout => consts::option::LOGOUT,
            TelnetOption::ByteMacro => consts::option::BM,
            TelnetOption::DET => consts::option::DET,
            TelnetOption::SUPDUP => consts::option::SUPDUP,
            TelnetOption::SUPDUPOutput => consts::option::SUPDUP_OUTPUT,
            TelnetOption::SNDLOC => consts::option::SNDLOC,
            TelnetOption::TTYPE => consts::option::TTYPE,
            TelnetOption::EOR => consts::option::EOR,
            TelnetOption::TUID => consts::option::TUID,
            TelnetOption::OUTMRK => consts::option::OUTMRK,
            TelnetOption::TTYLOC => consts::option::TTYLOC,
            TelnetOption::OPT3270Regime => consts::option::OPT3270REGIME,
            TelnetOption::X3PAD => consts::option::X3PAD,
            TelnetOption::NAWS => consts::option::NAWS,
            TelnetOption::TSPEED => consts::option::TSPEED,
            TelnetOption::LFLOW => consts::option::LFLOW,
            TelnetOption::Linemode => consts::option::LINEMODE,
            TelnetOption::XDISPLOC => consts::option::XDISPLOC,
            TelnetOption::Environment => consts::option::OLD_ENVIRONMENT,
            TelnetOption::Authentication => consts::option::AUTHENTICATION,
            TelnetOption::Encryption => consts::option::ENCRYPTION,
            TelnetOption::NewEnvironment => consts::option::NEW_ENVIRONMENT,
            TelnetOption::TN3270E => consts::option::TN3270E,
            TelnetOption::XAUTH => consts::option::XAUTH,
            TelnetOption::Charset => consts::option::CHARSET,
            TelnetOption::TRSP => consts::option::TRSP,
            TelnetOption::CPCO => consts::option::CPCO,
            TelnetOption::TSLE => consts::option::TSLE,
            TelnetOption::StartTLS => consts::option::START_TLS,
            TelnetOption::Kermit => consts::option::KERMIT,
            TelnetOption::SendUrl => consts::option::SENDURL,
            TelnetOption::ForwardX => consts::option::FORWARDX,
            TelnetOption::MSDP => consts::option::MSDP,
            TelnetOption::MSSP => consts::option::MSSP,
            TelnetOption::Compress1 => consts::option::COMPRESS1,
            TelnetOption::Compress2 => consts::option::COMPRESS2,
            TelnetOption::ZMP => consts::option::ZMP,
            TelnetOption::PragmaLogon => consts::option::PRAGMA_LOGIN,
            TelnetOption::SSPILogon => consts::option::SSPI_LOGIN,
            TelnetOption::PragmaHeartbeat => consts::option::PRAGMA_HEARTBEAT,
            TelnetOption::EXOPL => consts::option::EXOPL,
            TelnetOption::Unknown(byte) => byte,
        }
    }
}
