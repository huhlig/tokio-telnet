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

mod action;
mod msdp;
mod mssp;
mod naocrd;
mod naohts;
mod option;
mod status;

pub use self::action::TelnetAction;
pub use self::msdp::MudServerData;
pub use self::msdp::MudServerDataValue;
pub use self::mssp::MudServerStatus;
pub use self::naocrd::NAOCRD;
pub use self::naohts::NAOHTS;
pub use self::option::TelnetOption;
pub use self::status::TelnetOptionStatus;

use super::consts;
use crate::TelnetCodecError;
use bytes::BufMut;

///
/// `TelnetNegotiation`
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TelnetNegotiation(TelnetAction, TelnetOption);

impl TelnetNegotiation {
    ///
    /// Create new `TelnetNegotiation`
    ///
    pub fn new(action: TelnetAction, option: TelnetOption) -> TelnetNegotiation {
        TelnetNegotiation(action, option)
    }
    ///
    /// Get the `TelnetAction` of this `TelnetNegotiation`
    ///
    pub fn action(&self) -> TelnetAction {
        self.0
    }
    ///
    /// Get the `TelnetOption` of this `TelnetNegotiation`
    ///
    pub fn option(&self) -> TelnetOption {
        self.1
    }
    ///
    /// Get Encoded Length of this TelnetNegotiation (Hint: Always 3)
    ///
    pub fn len(&self) -> usize {
        3
    }
    ///
    /// Encode TelnetNegotiation to BufMut
    ///
    pub fn encode<T: BufMut>(&self, dst: &mut T) -> Result<(), TelnetCodecError> {
        dst.put_u8(consts::IAC);
        dst.put_u8(self.action().into());
        dst.put_u8(self.option().into());
    }
}

///
/// `TelnetSubnegotiation`
///
#[derive(Clone, Debug, PartialEq)]
pub enum TelnetSubnegotiation {
    /// Status SEND Argument
    StatusSend,
    /// Status IS Argument
    StatusIs(TelnetOptionStatus),
    /// Negotiate About Output Carriage-Return Disposition Data Sender
    NAOCRDDataSender(NAOCRD),
    /// Negotiate About Output Carriage-Return Disposition Data Receiver
    NAOCRDDataReceiver(NAOCRD),
    /// Mud Server Data
    MudServerData(MudServerData),
    /// Mud Server Status
    MudServerStatus(MudServerStatus),
    /// Unknown Subnegotiation Argument
    Unknown(u8, Vec<u8>),
}

impl TelnetSubnegotiation {
    ///
    /// Get Telnet Option
    ///
    pub fn option(&self) -> TelnetOption {
        match *self {
            TelnetSubnegotiation::MudServerData(_) => TelnetOption::MSDP,
            TelnetSubnegotiation::MudServerStatus(_) => TelnetOption::MSSP,
            TelnetSubnegotiation::NAOCRDDataSender(_) => TelnetOption::NAOCRD,
            TelnetSubnegotiation::NAOCRDDataReceiver(_) => TelnetOption::NAOCRD,
            TelnetSubnegotiation::StatusSend => TelnetOption::Status,
            TelnetSubnegotiation::StatusIs(_) => TelnetOption::Status,
            TelnetSubnegotiation::Unknown(option, _) => TelnetOption::Unknown(option),
        }
    }
    ///
    /// Get Encoded Length of Subnegotiation
    ///
    pub fn len(&self) -> usize {
        // Base size of a subnegotiation is 4
        let mut length = 4;
        length += match self {
            TelnetSubnegotiation::MudServerData(msd) => msd.len(),
            TelnetSubnegotiation::MudServerStatus(mss) => mss.len(),
            TelnetSubnegotiation::NAOCRDDataSender(_) => 1,
            TelnetSubnegotiation::NAOCRDDataReceiver(_) => 1,
            TelnetSubnegotiation::StatusSend => 1,
            TelnetSubnegotiation::StatusIs(status) => status.len(),
            TelnetSubnegotiation::Unknown(_option, payload) => 1 + payload.len(),
        };
        length
    }
    ///
    /// Encode Subnegotiation to buffer
    ///
    pub fn encode<T: BufMut>(&self, dst: &mut T) -> Result<(), TelnetCodecError> {
        dst.put_u8(consts::IAC);
        dst.put_u8(consts::SB);
        dst.put_u8(self.option().into());
        match self {
            TelnetSubnegotiation::MudServerData(msd) => msd.encode(dst),
            TelnetSubnegotiation::MudServerStatus(mss) => mss.encode(dst),
            TelnetSubnegotiation::NAOCRDDataSender(ds) => ds.encode(dst),
            TelnetSubnegotiation::NAOCRDDataReceiver(dr) => dr.encode(dst),
            TelnetSubnegotiation::StatusSend => {
                dst.put_u8(consts::negotiation::status::SEND);
            }
            TelnetSubnegotiation::StatusIs(_options) => {}
            TelnetSubnegotiation::Unknown(_option, _payload) => {}
        }
        dst.put_u8(consts::IAC);
        dst.put_u8(consts::SE);
    }
}
