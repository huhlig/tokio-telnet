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

///
/// Option State
///
#[derive(Clone, Copy)]
pub struct OptionState {
    /// Local requested permission to perform
    pub local_will: bool,
    /// Remote approved permission for local to perform
    pub remote_do: bool,
    /// Remote requested permission to perform
    pub remote_will: bool,
    /// Local approved Permission for remote to perform
    pub local_do: bool,
}

impl OptionState {
    /// Should we be performing a given option
    pub fn local_perform(&self) -> bool {
        self.local_will & &self.remote_do
    }
    /// Are we expecting them to perform a given option
    pub fn remote_perform(&self) -> bool {
        self.remote_will & &self.local_do
    }
}

impl fmt::Debug for OptionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "LtR(local_will: {}, remote_do: {}) = {}, RtL(remote_will: {}, local_do: {}) = {}",
            self.local_will,
            self.remote_do,
            self.local_perform(),
            self.remote_will,
            self.local_do,
            self.remote_perform()
        )
    }
}

impl Default for OptionState {
    fn default() -> Self {
        OptionState {
            local_will: false,
            remote_do: false,
            remote_will: false,
            local_do: false,
        }
    }
}
