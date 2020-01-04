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

use bytes::BufMut;

///
/// Telnet Subnegotiation Argument
///
pub enum TelnetArgument {
    Unknown(Vec<u8>),
}

pub trait Argument {
    ///
    /// Get Encoded Length of `Argument`
    ///
    fn len(&self) -> usize;
    ///
    /// Encode `Argument` to `BufMut`
    ///
    fn encode<T: BufMut>(&self, dst: &mut T);
    ///
    /// Decode `Argument` to `BufMut`
    ///
    fn dencode<T: BufMut>(&self, src: &mut T) -> Self;
}
