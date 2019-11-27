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
use bytes::BufMut;

///
/// Negotiate About Output Carriage-Return Disposition Data Sender (NAOCRD)
///
#[derive(Clone, Debug)]
pub enum NAOCRD {
    Sender(u8),
    Receiver(u8),
    Unknown(u8, u8),
}

impl NAOCRD {
    pub fn len(&self) -> usize {
        2
    }
    pub fn encode<T: BufMut>(&self, dst: &mut T) {
        match *self {
            NAOCRD::Sender(value) => {
                dst.put_u8(consts::negotiation::naocrd::DS);
                dst.put_u8(value);
            }
            NAOCRD::Receiver(value) => {
                dst.put_u8(consts::negotiation::naocrd::DS);
                dst.put_u8(value);
            }
            NAOCRD::Unknown(side, value) => {
                dst.put_u8(side);
                dst.put_u8(value);
            }
        }
    }
}
