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
use bytes::BufMut;
use std::collections::HashMap;

///
/// (Mud Server Status)[https://tintin.sourceforge.io/protocols/mssp/]
///
#[derive(Clone, Debug)]
pub struct MudServerStatus(HashMap<String, Vec<String>>);

impl MudServerStatus {
    ///
    /// Create a new MudServerStatus
    ///
    pub fn new() -> MudServerStatus {
        MudServerStatus(HashMap::new())
    }
    ///
    /// Get Encoded Length of `MudServerStatus`
    ///
    pub fn len(&self) -> usize {
        let mut length = 0;
        for (key, values) in &self.0 {
            length += 1;
            length += key.len();
            for value in values {
                length += 1;
                length += value.len();
            }
        }
        length
    }
    ///
    /// Encode `MudServerStatus` to `BufMut`
    ///
    pub fn encode<T: BufMut>(&self, dst: &mut T) {
        for (key, values) in &self.0 {
            dst.put_u8(consts::negotiation::mssp::VAR);
            dst.put(
                key.chars()
                    .filter(|ch| {
                        *ch != consts::NUL as char
                            && *ch != consts::IAC as char
                            && *ch != consts::negotiation::mssp::VAR as char
                            && *ch != consts::negotiation::mssp::VAL as char
                    })
                    .collect::<String>(),
            );
            for value in values {
                dst.put_u8(consts::negotiation::mssp::VAL);
                dst.put(
                    value
                        .chars()
                        .filter(|ch| {
                            *ch != consts::NUL as char
                                && *ch != consts::IAC as char
                                && *ch != consts::negotiation::mssp::VAR as char
                                && *ch != consts::negotiation::mssp::VAL as char
                        })
                        .collect::<String>(),
                );
            }
        }
    }
}
