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

use crate::consts;

///
/// [Telnet Terminal Options](https://www.iana.org/assignments/telnet-options/telnet-options.xhtml)
///
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TerminalOption {
    /// Telnet Binary Transmission [RFC856](https://tools.ietf.org/html/rfc856)
    TransmitBinary,
    /// Telnet Echo Option [RFC857](https://tools.ietf.org/html/rfc857)
    Echo,
    /// Telnet Reconnection Option [DDN Protocol Handbook, "Telnet Reconnection Option", NIC 50005, December 1985.]()
    /// Note: Prepare to reconnect
    Reconnection,
    /// Supress Go ahead [RFC858](https://tools.ietf.org/html/rfc858)
    SuppressGoAhead,
    /// Negotiate Approximate Message Size
    NegotiateApproxMessageSize,
    /// Telnet Status Option [RFC859](http://www.iana.org/go/rfc859)
    Status,
    /// Telnet Timing Mark Option [RFC860](http://www.iana.org/go/rfc860)
    TimingMark,
    /// Remote Controlled Transmission and Echo [RFC726](http://www.iana.org/go/rfc726)
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
    /// Output Form Feed Disposition [RFC655](http://www.iana.org/go/rfc655)
    NAOFFD,
    /// Output Vertical Tab Stops [RFC656](http://www.iana.org/go/rfc656)
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
    /// Generic Mud Communication Protocol [GMCP Protocol](https://www.gammon.com.au/gmcp)
    GMCP,
    /// Extended-Options-List [RFC861](http://www.iana.org/go/rfc861)
    EXOPL,
    /// Unknown
    Unknown(u8),
}

impl From<u8> for TerminalOption {
    fn from(byte: u8) -> Self {
        match byte {
            consts::option::BINARY => TerminalOption::TransmitBinary,
            consts::option::ECHO => TerminalOption::Echo,
            consts::option::RCP => TerminalOption::Reconnection,
            consts::option::SGA => TerminalOption::SuppressGoAhead,
            consts::option::NAMS => TerminalOption::NegotiateApproxMessageSize,
            consts::option::STATUS => TerminalOption::Status,
            consts::option::TM => TerminalOption::TimingMark,
            consts::option::RCTE => TerminalOption::RCTE,
            consts::option::NAOL => TerminalOption::OutLineWidth,
            consts::option::NAOP => TerminalOption::OutPageSize,
            consts::option::NAOCRD => TerminalOption::NAOCRD,
            consts::option::NAOHTS => TerminalOption::NAOHTS,
            consts::option::NAOHTD => TerminalOption::NAOHTD,
            consts::option::NAOFFD => TerminalOption::NAOFFD,
            consts::option::NAOVTS => TerminalOption::NAOVTS,
            consts::option::NAOVTD => TerminalOption::NAOVTD,
            consts::option::NAOLFD => TerminalOption::NAOLFD,
            consts::option::XASCII => TerminalOption::XASCII,
            consts::option::LOGOUT => TerminalOption::Logout,
            consts::option::BM => TerminalOption::ByteMacro,
            consts::option::DET => TerminalOption::DET,
            consts::option::SUPDUP => TerminalOption::SUPDUP,
            consts::option::SUPDUP_OUTPUT => TerminalOption::SUPDUPOutput,
            consts::option::SNDLOC => TerminalOption::SNDLOC,
            consts::option::TTYPE => TerminalOption::TTYPE,
            consts::option::EOR => TerminalOption::EOR,
            consts::option::TUID => TerminalOption::TUID,
            consts::option::OUTMRK => TerminalOption::OUTMRK,
            consts::option::TTYLOC => TerminalOption::TTYLOC,
            consts::option::OPT3270REGIME => TerminalOption::OPT3270Regime,
            consts::option::X3PAD => TerminalOption::X3PAD,
            consts::option::NAWS => TerminalOption::NAWS,
            consts::option::TSPEED => TerminalOption::TSPEED,
            consts::option::LFLOW => TerminalOption::LFLOW,
            consts::option::LINEMODE => TerminalOption::Linemode,
            consts::option::XDISPLOC => TerminalOption::XDISPLOC,
            consts::option::OLD_ENVIRONMENT => TerminalOption::Environment,
            consts::option::AUTHENTICATION => TerminalOption::Authentication,
            consts::option::ENCRYPTION => TerminalOption::Encryption,
            consts::option::NEW_ENVIRONMENT => TerminalOption::NewEnvironment,
            consts::option::TN3270E => TerminalOption::TN3270E,
            consts::option::XAUTH => TerminalOption::XAUTH,
            consts::option::CHARSET => TerminalOption::Charset,
            consts::option::TRSP => TerminalOption::TRSP,
            consts::option::CPCO => TerminalOption::CPCO,
            consts::option::TSLE => TerminalOption::TSLE,
            consts::option::START_TLS => TerminalOption::StartTLS,
            consts::option::KERMIT => TerminalOption::Kermit,
            consts::option::SENDURL => TerminalOption::SendUrl,
            consts::option::FORWARDX => TerminalOption::ForwardX,
            consts::option::MSDP => TerminalOption::MSDP,
            consts::option::MSSP => TerminalOption::MSSP,
            consts::option::COMPRESS1 => TerminalOption::Compress1,
            consts::option::COMPRESS2 => TerminalOption::Compress2,
            consts::option::ZMP => TerminalOption::ZMP,
            consts::option::PRAGMA_LOGIN => TerminalOption::PragmaLogon,
            consts::option::SSPI_LOGIN => TerminalOption::SSPILogon,
            consts::option::PRAGMA_HEARTBEAT => TerminalOption::PragmaHeartbeat,
            consts::option::GMCP => TerminalOption::GMCP,
            consts::option::EXOPL => TerminalOption::EXOPL,
            byte => TerminalOption::Unknown(byte),
        }
    }
}

impl From<TerminalOption> for u8 {
    fn from(option: TerminalOption) -> Self {
        match option {
            TerminalOption::TransmitBinary => consts::option::BINARY,
            TerminalOption::Echo => consts::option::ECHO,
            TerminalOption::Reconnection => consts::option::RCP,
            TerminalOption::SuppressGoAhead => consts::option::SGA,
            TerminalOption::NegotiateApproxMessageSize => consts::option::NAMS,
            TerminalOption::Status => consts::option::STATUS,
            TerminalOption::TimingMark => consts::option::TM,
            TerminalOption::RCTE => consts::option::RCTE,
            TerminalOption::OutLineWidth => consts::option::NAOL,
            TerminalOption::OutPageSize => consts::option::NAOP,
            TerminalOption::NAOCRD => consts::option::NAOCRD,
            TerminalOption::NAOHTS => consts::option::NAOHTS,
            TerminalOption::NAOHTD => consts::option::NAOHTD,
            TerminalOption::NAOFFD => consts::option::NAOFFD,
            TerminalOption::NAOVTS => consts::option::NAOVTS,
            TerminalOption::NAOVTD => consts::option::NAOVTD,
            TerminalOption::NAOLFD => consts::option::NAOLFD,
            TerminalOption::XASCII => consts::option::XASCII,
            TerminalOption::Logout => consts::option::LOGOUT,
            TerminalOption::ByteMacro => consts::option::BM,
            TerminalOption::DET => consts::option::DET,
            TerminalOption::SUPDUP => consts::option::SUPDUP,
            TerminalOption::SUPDUPOutput => consts::option::SUPDUP_OUTPUT,
            TerminalOption::SNDLOC => consts::option::SNDLOC,
            TerminalOption::TTYPE => consts::option::TTYPE,
            TerminalOption::EOR => consts::option::EOR,
            TerminalOption::TUID => consts::option::TUID,
            TerminalOption::OUTMRK => consts::option::OUTMRK,
            TerminalOption::TTYLOC => consts::option::TTYLOC,
            TerminalOption::OPT3270Regime => consts::option::OPT3270REGIME,
            TerminalOption::X3PAD => consts::option::X3PAD,
            TerminalOption::NAWS => consts::option::NAWS,
            TerminalOption::TSPEED => consts::option::TSPEED,
            TerminalOption::LFLOW => consts::option::LFLOW,
            TerminalOption::Linemode => consts::option::LINEMODE,
            TerminalOption::XDISPLOC => consts::option::XDISPLOC,
            TerminalOption::Environment => consts::option::OLD_ENVIRONMENT,
            TerminalOption::Authentication => consts::option::AUTHENTICATION,
            TerminalOption::Encryption => consts::option::ENCRYPTION,
            TerminalOption::NewEnvironment => consts::option::NEW_ENVIRONMENT,
            TerminalOption::TN3270E => consts::option::TN3270E,
            TerminalOption::XAUTH => consts::option::XAUTH,
            TerminalOption::Charset => consts::option::CHARSET,
            TerminalOption::TRSP => consts::option::TRSP,
            TerminalOption::CPCO => consts::option::CPCO,
            TerminalOption::TSLE => consts::option::TSLE,
            TerminalOption::StartTLS => consts::option::START_TLS,
            TerminalOption::Kermit => consts::option::KERMIT,
            TerminalOption::SendUrl => consts::option::SENDURL,
            TerminalOption::ForwardX => consts::option::FORWARDX,
            TerminalOption::MSDP => consts::option::MSDP,
            TerminalOption::MSSP => consts::option::MSSP,
            TerminalOption::Compress1 => consts::option::COMPRESS1,
            TerminalOption::Compress2 => consts::option::COMPRESS2,
            TerminalOption::ZMP => consts::option::ZMP,
            TerminalOption::PragmaLogon => consts::option::PRAGMA_LOGIN,
            TerminalOption::SSPILogon => consts::option::SSPI_LOGIN,
            TerminalOption::PragmaHeartbeat => consts::option::PRAGMA_HEARTBEAT,
            TerminalOption::GMCP => consts::option::GMCP,
            TerminalOption::EXOPL => consts::option::EXOPL,
            TerminalOption::Unknown(byte) => byte,
        }
    }
}
