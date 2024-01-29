// Copyright (C) 2024 Lily Lyons
//
// This file is part of fmod-rs.
//
// fmod-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fmod-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with fmod-rs.  If not, see <http://www.gnu.org/licenses/>.

use fmod::studio::LoadBankFlags;
use once_cell::sync::Lazy;

struct State {
    system: fmod::studio::System,
    master_bank: fmod::studio::Bank,
    string_bank: fmod::studio::Bank,
}

static STATE: Lazy<State> = Lazy::new(|| {
    // # Safety: Lazy ensures that this is called from only one thread
    let system = unsafe { fmod::studio::System::new().unwrap() };

    let master_bank = system
        .load_bank_file(
            c"../fmod/api/studio/examples/media/Master.bank",
            LoadBankFlags::NORMAL,
        )
        .unwrap();
    let string_bank = system
        .load_bank_file(
            c"../fmod/api/studio/examples/media/Master.strings.bank",
            LoadBankFlags::NORMAL,
        )
        .unwrap();

    State {
        system,
        master_bank,
        string_bank,
    }
});
