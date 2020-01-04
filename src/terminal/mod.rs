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

mod arguments;
mod error;
mod input;
mod options;
mod output;
mod state;
mod terminal;

pub use self::error::TerminalError;
pub use self::input::TerminalInput;
pub use self::output::TerminalOutput;
pub use self::terminal::NetworkVirtualTerminal;

pub mod option {
    pub use super::arguments::msdp;
    pub use super::arguments::mssp;
    pub use super::arguments::naocrd;
    pub use super::arguments::naohts;
    pub use super::arguments::status;
    pub use super::arguments::TelnetArgument;
    pub use super::options::TerminalOption;
}
