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
use crate::TelnetAction;
use crate::TelnetOption;
use bytes::BufMut;
use std::collections::HashMap;

///
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct TelnetOptionStatus(HashMap<TelnetOption, (TelnetAction, TelnetAction)>);

impl TelnetOptionStatus {
    ///
    /// Get Encoded Length of `TelnetOptionStatus`
    ///
    pub(crate) fn len(&self) -> usize {
        1 + self.0.len() * 4 // Two bytes per item
    }
    ///
    /// Encode `TelnetOptionStatus` to `BufMut`
    ///
    pub(crate) fn encode<T: BufMut>(&self, dst: &mut T) {
        dst.put_u8(consts::negotiation::status::IS);
        for (option, action) in &self.0 {
            dst.put_u8(u8::from(action.0));
            dst.put_u8(u8::from(*option));
            dst.put_u8(u8::from(action.1));
            dst.put_u8(u8::from(*option));
        }
    }
}
