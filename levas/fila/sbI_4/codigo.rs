// ---------- sbI_4: the squad manager object ----------

pub(crate) const FN_SQUAD_MANAGER_LOAD_SBI: usize = 0x859bc0; // SQUAD_MANAGER::load thiscall(this, save) ret 4 -> 1/0

/// Builds the squad manager: the CD check (`find_game_cd`, the no-CD patch read at 0x859ade and 0x859b21), the
/// registration year (+0, short) 0 and, with a saved game (`save`), the year read from it (`SQUAD_MANAGER::load`, its
/// result not looked at). A new game sets the year later (`FN_SQUAD_MANAGER_INIT_NEW_GAME`). Returns this.
/// Original: `SQUAD_MANAGER::SQUAD_MANAGER` (0x859aa0, squad_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_squad_manager(this: usize, save: usize) -> usize {
    find_game_cd(0x859ade, 0x859b21);
    write_u16(this, 0); // +0: the registration year
    if save != 0 {
        covered!("sbI_4 squad manager from the saved game");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_LOAD_SBI);
        load(this, save); // sbI_4: the year from the saved game
    }
    this
}

/// Comparer for `new_squad_manager`: only the original runs (it waits for the CD and reads the saved game); without a
/// saved game the year must be 0.
pub(crate) unsafe extern "thiscall" fn compare_new_squad_manager(this: usize, save: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save);
    SCOREBOARD.record(r == this && (save != 0 || read_u16(this) == 0),
        || format!("new_squad_manager(save {:#x}): returned {:#x}, year {}", save, r, read_u16(this)));
    r
}
