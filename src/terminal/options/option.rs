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
    TelnetOptionPragmaLogon,
    /// Telnet Option SSPI Logon [Steve_McGregory](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Steve_McGregory)
    TelnetOptionSSPILogon,
    /// Telnet Option Pragma Heartbeat [Steve_McGregory](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml#Steve_McGregory)
    TelnetOptionPragmaHeartbeat,
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
            consts::negotiation::BINARY => TelnetOption::TransmitBinary,
            consts::negotiation::ECHO => TelnetOption::Echo,
            consts::negotiation::RCP => TelnetOption::Reconnection,
            consts::negotiation::SGA => TelnetOption::SuppressGoAhead,
            consts::negotiation::NAMS => TelnetOption::ApproxMessageSizeNegotiation,
            consts::negotiation::STATUS => TelnetOption::Status,
            consts::negotiation::TM => TelnetOption::TimingMark,
            consts::negotiation::RCTE => TelnetOption::RCTE,
            consts::negotiation::NAOL => TelnetOption::OutLineWidth,
            consts::negotiation::NAOP => TelnetOption::OutPageSize,
            consts::negotiation::NAOCRD => TelnetOption::NAOCRD,
            consts::negotiation::NAOHTS => TelnetOption::NAOHTS,
            consts::negotiation::NAOHTD => TelnetOption::NAOHTD,
            consts::negotiation::NAOFFD => TelnetOption::NAOFFD,
            consts::negotiation::NAOVTS => TelnetOption::NAOVTS,
            consts::negotiation::NAOVTD => TelnetOption::NAOVTD,
            consts::negotiation::NAOLFD => TelnetOption::NAOLFD,
            consts::negotiation::XASCII => TelnetOption::XASCII,
            consts::negotiation::LOGOUT => TelnetOption::Logout,
            consts::negotiation::BM => TelnetOption::ByteMacro,
            consts::negotiation::DET => TelnetOption::DET,
            consts::negotiation::SUPDUP => TelnetOption::SUPDUP,
            consts::negotiation::SUPDUP_OUTPUT => TelnetOption::SUPDUPOutput,
            consts::negotiation::SNDLOC => TelnetOption::SNDLOC,
            consts::negotiation::TTYPE => TelnetOption::TTYPE,
            consts::negotiation::EOR => TelnetOption::EOR,
            consts::negotiation::TUID => TelnetOption::TUID,
            consts::negotiation::OUTMRK => TelnetOption::OUTMRK,
            consts::negotiation::TTYLOC => TelnetOption::TTYLOC,
            consts::negotiation::OPT3270REGIME => TelnetOption::OPT3270Regime,
            consts::negotiation::X3PAD => TelnetOption::X3PAD,
            consts::negotiation::NAWS => TelnetOption::NAWS,
            consts::negotiation::TSPEED => TelnetOption::TSPEED,
            consts::negotiation::LFLOW => TelnetOption::LFLOW,
            consts::negotiation::LINEMODE => TelnetOption::Linemode,
            consts::negotiation::XDISPLOC => TelnetOption::XDISPLOC,
            consts::negotiation::OLD_ENVIRONMENT => TelnetOption::Environment,
            consts::negotiation::AUTHENTICATION => TelnetOption::Authentication,
            consts::negotiation::ENCRYPTION => TelnetOption::Encryption,
            consts::negotiation::NEW_ENVIRONMENT => TelnetOption::NewEnvironment,
            consts::negotiation::TN3270E => TelnetOption::TN3270E,
            consts::negotiation::XAUTH => TelnetOption::XAUTH,
            consts::negotiation::CHARSET => TelnetOption::Charset,
            consts::negotiation::TRSP => TelnetOption::TRSP,
            consts::negotiation::CPCO => TelnetOption::CPCO,
            consts::negotiation::TSLE => TelnetOption::TSLE,
            consts::negotiation::START_TLS => TelnetOption::StartTLS,
            consts::negotiation::KERMIT => TelnetOption::Kermit,
            consts::negotiation::SENDURL => TelnetOption::SendUrl,
            consts::negotiation::FORWARDX => TelnetOption::ForwardX,
            consts::negotiation::MSDP => TelnetOption::MSDP,
            consts::negotiation::MSSP => TelnetOption::MSSP,
            consts::negotiation::COMPRESS1 => TelnetOption::Compress1,
            consts::negotiation::COMPRESS2 => TelnetOption::Compress2,
            consts::negotiation::ZMP => TelnetOption::ZMP,
            consts::negotiation::EXOPL => TelnetOption::EXOPL,
            consts::negotiation::PRAGMA_LOGIN => TelnetOption::TelnetOptionPragmaLogon,
            consts::negotiation::SSPI_LOGIN => TelnetOption::TelnetOptionSSPILogon,
            consts::negotiation::PRAGMA_HEARTBEAT => TelnetOption::TelnetOptionPragmaHeartbeat,
            byte => TelnetOption::Unknown(byte),
        }
    }
}

impl From<TelnetOption> for u8 {
    fn from(option: TelnetOption) -> Self {
        match option {
            TelnetOption::TransmitBinary => consts::negotiation::BINARY,
            TelnetOption::Echo => consts::negotiation::ECHO,
            TelnetOption::Reconnection => consts::negotiation::RCP,
            TelnetOption::SuppressGoAhead => consts::negotiation::SGA,
            TelnetOption::ApproxMessageSizeNegotiation => consts::negotiation::NAMS,
            TelnetOption::Status => consts::negotiation::STATUS,
            TelnetOption::TimingMark => consts::negotiation::TM,
            TelnetOption::RCTE => consts::negotiation::RCTE,
            TelnetOption::OutLineWidth => consts::negotiation::NAOL,
            TelnetOption::OutPageSize => consts::negotiation::NAOP,
            TelnetOption::NAOCRD => consts::negotiation::NAOCRD,
            TelnetOption::NAOHTS => consts::negotiation::NAOHTS,
            TelnetOption::NAOHTD => consts::negotiation::NAOHTD,
            TelnetOption::NAOFFD => consts::negotiation::NAOFFD,
            TelnetOption::NAOVTS => consts::negotiation::NAOVTS,
            TelnetOption::NAOVTD => consts::negotiation::NAOVTD,
            TelnetOption::NAOLFD => consts::negotiation::NAOLFD,
            TelnetOption::XASCII => consts::negotiation::XASCII,
            TelnetOption::Logout => consts::negotiation::LOGOUT,
            TelnetOption::ByteMacro => consts::negotiation::BM,
            TelnetOption::DET => consts::negotiation::DET,
            TelnetOption::SUPDUP => consts::negotiation::SUPDUP,
            TelnetOption::SUPDUPOutput => consts::negotiation::SUPDUP_OUTPUT,
            TelnetOption::SNDLOC => consts::negotiation::SNDLOC,
            TelnetOption::TTYPE => consts::negotiation::TTYPE,
            TelnetOption::EOR => consts::negotiation::EOR,
            TelnetOption::TUID => consts::negotiation::TUID,
            TelnetOption::OUTMRK => consts::negotiation::OUTMRK,
            TelnetOption::TTYLOC => consts::negotiation::TTYLOC,
            TelnetOption::OPT3270Regime => consts::negotiation::OPT3270REGIME,
            TelnetOption::X3PAD => consts::negotiation::X3PAD,
            TelnetOption::NAWS => consts::negotiation::NAWS,
            TelnetOption::TSPEED => consts::negotiation::TSPEED,
            TelnetOption::LFLOW => consts::negotiation::LFLOW,
            TelnetOption::Linemode => consts::negotiation::LINEMODE,
            TelnetOption::XDISPLOC => consts::negotiation::XDISPLOC,
            TelnetOption::Environment => consts::negotiation::OLD_ENVIRONMENT,
            TelnetOption::Authentication => consts::negotiation::AUTHENTICATION,
            TelnetOption::Encryption => consts::negotiation::ENCRYPTION,
            TelnetOption::NewEnvironment => consts::negotiation::NEW_ENVIRONMENT,
            TelnetOption::TN3270E => consts::negotiation::TN3270E,
            TelnetOption::XAUTH => consts::negotiation::XAUTH,
            TelnetOption::Charset => consts::negotiation::CHARSET,
            TelnetOption::TRSP => consts::negotiation::TRSP,
            TelnetOption::CPCO => consts::negotiation::CPCO,
            TelnetOption::TSLE => consts::negotiation::TSLE,
            TelnetOption::StartTLS => consts::negotiation::START_TLS,
            TelnetOption::Kermit => consts::negotiation::KERMIT,
            TelnetOption::SendUrl => consts::negotiation::SENDURL,
            TelnetOption::ForwardX => consts::negotiation::FORWARDX,
            TelnetOption::MSDP => consts::negotiation::MSDP,
            TelnetOption::MSSP => consts::negotiation::MSSP,
            TelnetOption::Compress1 => consts::negotiation::COMPRESS1,
            TelnetOption::Compress2 => consts::negotiation::COMPRESS2,
            TelnetOption::ZMP => consts::negotiation::ZMP,
            TelnetOption::TelnetOptionPragmaLogon => consts::negotiation::PRAGMA_LOGIN,
            TelnetOption::TelnetOptionSSPILogon => consts::negotiation::SSPI_LOGIN,
            TelnetOption::TelnetOptionPragmaHeartbeat => consts::negotiation::PRAGMA_HEARTBEAT,
            TelnetOption::EXOPL => consts::negotiation::EXOPL,
            TelnetOption::Unknown(byte) => byte,
        }
    }
}
