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

#[derive(Debug)]
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("print on drop: {}", self.0);
    }
}

fn main() -> fmod::Result<()> {
    // # Safety: we are only calling this from the main fn and the main thread.
    // No other thread or api call will overlap this.
    let system = unsafe { fmod::studio::System::new()? };

    system.set_user_data(Some(PrintOnDrop("haiiiiii :3")))?;

    let print_on_drop = PrintOnDrop("bank userdata has been dropped :3");
    let master_bank = system.load_bank_file(
        c"fmod/api/studio/examples/media/Master.bank",
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    master_bank.set_user_data(Some(print_on_drop))?;
    master_bank.set_user_data(None::<()>)?;

    println!("updating system");

    system.update()?;

    println!("releasing system");

    unsafe {
        // # Safety we're done processingg and about to return from the main fn.
        // No other API calls can happen after this.
        system.release()?;
    }

    println!("system released");

    Ok(())
}
