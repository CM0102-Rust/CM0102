// ---------- cx02_9: the Irish cups' rounds and the Leinster test (nir_cup.cpp, ire_chal_cup.cpp, ire_pres_cup.cpp) ----------

/// The Northern Irish Cup rounds: no prize fund (0) and the round table (`set_up_prize_rounds`). Returns 1.
/// Original: `NirCup::set_up_rounds` (0x78afe0, nir_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_cup_set_up_rounds(this: usize) -> i32 {
    covered!("cx02_9 northern irish cup rounds");
    set_up_prize_rounds(Cup(this), 0)
}

/// The rounds of the cups that share this virtual (the Irish Challenge Cup, the Leinster and Munster Cups and the
/// Croatian Cup): a prize fund of 100,000 and the round table (`set_up_prize_rounds`). Returns 1.
/// Original: `IreChalCup::set_up_rounds` (0x634c10, ire_chal_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_chal_cup_set_up_rounds(this: usize) -> i32 {
    covered!("cx02_9 irish challenge cup rounds");
    set_up_prize_rounds(Cup(this), 100_000)
}

/// Whether the zero-terminated text at `at` equals `text`.
unsafe fn c_text_is(at: usize, text: &[u8]) -> bool {
    text.iter().enumerate().all(|(k, &c)| read_u8(at + k) == c) && read_u8(at + text.len()) == 0
}

/// Whether a club plays in Leinster, by its stadium's city name (+4 of the city record): a name starting with "Dubl"
/// (strncmp of 4) or equal to Bray, Longford, Dundalk, Drogheda, Athlone or Kilkenny. No club, stadium or city: 0.
/// Original: `ire_club_is_in_leinster` (0x6399c0, ire_pres_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn ire_pres_cup_club_is_in_leinster(club: usize) -> u32 {
    if club == 0 {
        return 0;
    }
    let stadium = Club(club).stadium();
    if stadium.is_null() || stadium.city() == 0 {
        return 0;
    }
    let name = stadium.city() + 4; // the city's name
    if (0..4).all(|k| read_u8(name + k) == b"Dubl"[k]) {
        covered!("cx02_9 dublin club");
        return 1;
    }
    let towns: [&[u8]; 6] = [b"Bray", b"Longford", b"Dundalk", b"Drogheda", b"Athlone", b"Kilkenny"];
    towns.iter().any(|t| c_text_is(name, t)) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_nir_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_set_up_prize_rounds("nir_cup_set_up_rounds", Cup(this), || o(this), || nir_cup_set_up_rounds(this))
}

pub(crate) unsafe extern "thiscall" fn compare_ire_chal_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_set_up_prize_rounds("ire_chal_cup_set_up_rounds", Cup(this), || o(this), || ire_chal_cup_set_up_rounds(this))
}

/// Comparer for `ire_pres_cup_club_is_in_leinster`: the test only reads; the answers must match.
pub(crate) unsafe extern "C" fn compare_ire_pres_cup_club_is_in_leinster(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(club);
    let c = ire_pres_cup_club_is_in_leinster(club);
    SCOREBOARD.record(r == c, || format!("ire_pres_cup_club_is_in_leinster({:#x}) original {} copy {}", club, r, c));
    r
}
