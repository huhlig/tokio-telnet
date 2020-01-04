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

use crate::consts::option::SUPPORT;
use crate::terminal::options::TerminalOption;

pub struct ConfigurationError;

pub struct NegotiationError;

///
/// Network Virtual Terminal Configuration
///
pub struct OptionState {
    options: [((ConfiguredState, NegotiationState), (ConfiguredState, NegotiationState)); 255],
}

impl TerminalConfiguration {
    pub fn option_local_supported(&self, option: TerminalOption) -> bool {
        crate::consts::option::SUPPORT[option].0
    }
    pub fn option_remote_supported(&self, option: TerminalOption) -> bool {
        crate::consts::option::SUPPORT[option].1
    }
    pub fn option_allowed_local(&self, option: TerminalOption) -> bool {
        self.option_local_supported(option) && (self.options[option].0).0
    }
    pub fn option_allowed_remote(&self, option: TerminalOption) -> bool {
        self.option_remote_supported(option) && (self.options[option].1).0
    }
    pub fn option_enabled_local(&mut self, option: TerminalOption) -> bool {
        self.options.
    }
    pub fn option_enabled_remote(&mut self, option: TerminalOption) -> bool {
        self.option_remote_supported(option) && (self.options[option].1).0
    }
}

impl TerminalConfiguration {
    pub fn enable_local(
        &mut self,
        option: TelnetOption,
    ) -> Result<Option<TelnetFrame>, NegotiationError> {
    }
    pub fn disable_local(
        &mut self,
        option: TelnetOption,
    ) -> Result<Option<TelnetFrame>, NegotiationError> {
    }
    pub fn enable_remote(
        &mut self,
        option: TelnetOption,
    ) -> Result<Option<TelnetFrame>, NegotiationError> {
    }
    pub fn disable_remote(
        &mut self,
        option: TelnetOption,
    ) -> Result<Option<TelnetFrame>, NegotiationError> {
    }
    pub fn receive_do(
        &mut self,
        option: TelnetOption,
    ) -> Result<Option<TelnetFrame>, NegotiationError> {
    }
    pub fn receive_dont(
        &mut self,
        option: TelnetOption,
    ) -> Result<Option<TelnetFrame>, NegotiationError> {
    }
    pub fn receive_dowill(
        &mut self,
        option: TelnetOption,
    ) -> Result<Option<TelnetFrame>, NegotiationError> {
    }
    pub fn receive_wont(
        &mut self,
        option: TelnetOption,
    ) -> Result<Option<TelnetFrame>, NegotiationError> {
    }
    /// Handle Incoming
    fn handle(
        &mut self,
        action: Action,
        option: TelnetOption,
    ) -> Result<Option<TelnetFrame>, NegotiationError> {
        use self::{Action::*, Source::*, State::*};
        // @formatter:off
        #[rustfmt::skip]
        match (self.options[option].0, self.options[option].1, source, action) {
        //  (Local State, Remote State, Source, Action) => {  }
            (         No,            _,   Recv,       Do) => { self.options[option].0 = WantYes;
                                                               Ok(Some(TelnetFrame::Will(option)))}
            (         No,            _,   Recv,     Dont) => { Ok(None) /* Ignore */ }
            (_,_,_) => {panic!("Incomplete")}
        }
        // @formatter:on
    }
}

impl Default for OptionManager {
    fn default() -> OptionManager {
        OptionManager {
            options: [(State::No, State::No); 255],
            enabled: [(false, false); 255],
        }
    }
}

enum Source {
    Send,
    Recv,
}

enum Action {
    Do,
    Dont,
    Will,
    Wont,
}

enum NegotiationState {
    No,
    WantNo,
    WantNoOpposite,
    Yes,
    WantYes,
    WantYesOpposite,
}

enum ConfiguredState {
    Unsupported,
    Supported,
    Allowed,
}