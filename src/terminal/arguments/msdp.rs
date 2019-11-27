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
use std::collections::HashMap;

///
/// `MudServerData` contains data about the Mud.
///
#[derive(Clone, Debug)]
pub struct MudServerData(HashMap<String, MudServerDataValue>);

impl MudServerData {
    ///
    /// Create a new `MudServerData` Table
    ///
    pub fn new(table: HashMap<String, MudServerDataValue>) -> MudServerData {
        MudServerData(table)
    }
    ///
    /// Get Encoded Length of `MudServerData`
    ///
    pub fn len(&self) -> usize {
        let mut length = 0;
        for (key, value) in &self.0 {
            length += 1; // VAL
            length += key.len();
            length += 1; // VAR
            length += value.len();
        }
        length
    }
    ///
    /// Encode `MudServerData` to `BufMut`
    ///
    pub fn encode<T: BufMut>(&self, dst: &mut T) {
        for (key, value) in &self.0 {
            dst.put_u8(consts::negotiation::msdp::VAR);
            dst.put(key);
            dst.put_u8(consts::negotiation::msdp::VAL);
            value.encode(dst);
        }
    }
}

///
/// `MudServerDataValue`
///
#[derive(Clone, Debug)]
pub enum MudServerDataValue {
    String(String),
    Array(Vec<MudServerDataValue>),
    Table(HashMap<String, MudServerDataValue>),
}

impl MudServerDataValue {
    ///
    /// Create a new String Value
    ///
    pub fn string(string: &str) -> MudServerDataValue {
        MudServerDataValue::String(string.to_string())
    }
    ///
    /// Create a new Array Value
    ///
    pub fn array(array: Vec<MudServerDataValue>) -> MudServerDataValue {
        MudServerDataValue::Array(array)
    }
    ///
    /// Create a new Table Value
    ///
    pub fn table(
        table: HashMap<String, MudServerDataValue>,
    ) -> MudServerDataValue {
        MudServerDataValue::Table(table)
    }
    ///
    /// Get Encoded Length of `MudServerDataValue`
    ///
    pub fn len(&self) -> usize {
        let mut length = 0;
        match self {
            MudServerDataValue::String(string) => {
                length += string.len();
            }
            MudServerDataValue::Array(array) => {
                length += 1; // ARRAY_OPEN
                for value in array {
                    length += 1; // VAL
                    length += value.len();
                }
                length += 1; // ARRAY_CLOSE
            }
            MudServerDataValue::Table(table) => {
                length += 1; // TABLE_OPEN
                for (key, value) in table {
                    length += 1; // VAR
                    length += key.len();
                    length += 1; // VAL
                    length += value.len();
                }
                length += 1; // TABLE_CLOSE
            }
        }
        length
    }
    ///
    /// Encode `MudServerDataValue` to `BufMut`
    ///
    pub fn encode<T: BufMut>(&self, dst: &mut T) {
        match self {
            MudServerDataValue::String(string) => {
                dst.put(string);
            }
            MudServerDataValue::Array(array) => {
                dst.put_u8(consts::negotiation::msdp::ARRAY_OPEN);
                for value in array {
                    dst.put_u8(consts::negotiation::msdp::VAL);
                    value.encode(dst);
                }
                dst.put_u8(consts::negotiation::msdp::ARRAY_CLOSE);
            }
            MudServerDataValue::Table(table) => {
                dst.put_u8(consts::negotiation::msdp::TABLE_OPEN);
                for (key, value) in table {
                    dst.put_u8(consts::negotiation::msdp::VAR);
                    dst.put(key);
                    dst.put_u8(consts::negotiation::msdp::VAL);
                    value.encode(dst);
                }
                dst.put_u8(consts::negotiation::msdp::TABLE_CLOSE);
            }
        }
    }
}
