// ---------- cx04_2: the Intertoto and UEFA cups' own data in the saved game (intertoto_cup.cpp, uefa_cup.cpp) ----------

/// Reads the Intertoto Cup's own data from the saved game: whether its winners were already passed into the UEFA Cup
/// (`intertoto_cup::WINNERS_SENT_TO_UEFA`, 4 bytes; `exe::crt_fread`). A short read shows the intertoto_cup.cpp box
/// (line 0x2d5) and returns 0; otherwise 1.
/// Original: `IntertotoCup::load_extra_data` (0x633eb0, intertoto_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + intertoto_cup::WINNERS_SENT_TO_UEFA, 4, 1, file) != 1 {
        show_error_at(0x9e9e14, 0x2d5); // intertoto_cup.cpp
        return 0;
    }
    covered!("cx04_2 intertoto data loaded");
    1
}

/// Writes the Intertoto Cup's own data to the saved game: the winners-passed flag (`intertoto_cup::WINNERS_SENT_TO_UEFA`,
/// 4 bytes; `exe::crt_fwrite`, unchecked). Returns the write's count.
/// Original: `IntertotoCup::save_extra_data` (0x633f50, intertoto_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn intertoto_cup_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx04_2 intertoto data saved");
    exe::crt_fwrite(this + intertoto_cup::WINNERS_SENT_TO_UEFA, 4, 1, file)
}

/// Reads the UEFA Cup's own data from the saved game: when the saved game's data version (`G_GAME_DATA_VERSION`,
/// absolute value) is above 0x3e003, the count of clubs in the preliminary round (`uefa_cup::PRELIMINARY_CLUB_COUNT`, 1
/// byte; `exe::crt_fread`, unchecked); in older saves that byte becomes 0. Returns 1.
/// Original: `UefaCup::load_extra_data` (0x903300, uefa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn uefa_cup_load_extra_data(this: usize, file: usize) -> u32 {
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() > 0x3e003 {
        covered!("cx04_2 uefa cup data loaded");
        exe::crt_fread(this + uefa_cup::PRELIMINARY_CLUB_COUNT, 1, 1, file);
    } else {
        write_u8(this + uefa_cup::PRELIMINARY_CLUB_COUNT, 0);
    }
    1
}

pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_cup_extra_data_load("intertoto_cup_load_extra_data", this, file, &[(intertoto_cup::WINNERS_SENT_TO_UEFA, 4)], || o(this, file), || intertoto_cup_load_extra_data(this, file))
}

/// Comparer for `intertoto_cup_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_intertoto_cup_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}

pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    compare_cup_extra_data_load("uefa_cup_load_extra_data", this, file, &[(uefa_cup::PRELIMINARY_CLUB_COUNT, 1)], || o(this, file), || uefa_cup_load_extra_data(this, file))
}
