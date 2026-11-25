// ---------- rx11_11: the African qualifying progress text and the reserve, Under 21 and B competitions' scalar deleting destructors (wc_african_cup.cpp, record_utils.cpp) ----------

/// rule: a nation's African qualifying progress reads "Qualified For World Cup" or "Final" in the final group,
/// "Qualified For World Cup", "Qualified For Final" or "Second Round" in the groups, and "First Round" in the first
/// round
///
/// Into `buf` (`qualifying_progress_sprintf`): a buffer under 30 bytes shows the wc_african_cup.cpp box (line 0x307) and
/// writes nothing. By the stage label `stage` (a signed word): 0x96 (the final group) "Qualified For World Cup" (with
/// the competition's gender byte) with status 1, else "Final"; 0x407 to 0x40b and 0x472 (the groups) "Qualified For
/// World Cup" with status 7, "Qualified For Final" with 6, else "Second Round"; otherwise, with the round code `round`
/// 10, "First Round", else nothing. Returns the writer's result (0 when nothing is written; the original leaves garbage
/// in eax). `this` is not read.
/// Original: `WcAfricanCup::write_progress_text` (0x910260, wc_african_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_write_progress_text(_this: usize, buf: usize, size: i32, stage: u32, round: u32, status: u32, comp: usize) -> i32 {
    if size < 0x1e {
        show_error_at(wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP, 0x307);
        return 0;
    }
    covered!("rx11_11 african progress text");
    let (stage, status) = (stage as u16 as i16, status as u8);
    if stage == 0x96 {
        // "Qualified For World Cup" / "Final"
        return if status == 1 { qualifying_progress_sprintf(buf, 0xad4658, Some(comp)) } else { qualifying_progress_sprintf(buf, 0x9a757c, None) };
    }
    if (0x407..=0x40b).contains(&stage) || stage == 0x472 {
        return match status {
            7 => qualifying_progress_sprintf(buf, 0xad4658, Some(comp)), // "Qualified For World Cup"
            6 => qualifying_progress_sprintf(buf, 0xad4634, Some(comp)), // rx11_11: "Qualified For Final"
            _ => qualifying_progress_sprintf(buf, 0x9a7b94, None),       // "Second Round"
        };
    }
    if round as u16 == 0xa {
        return qualifying_progress_sprintf(buf, 0x9a7c04, None); // "First Round"
    }
    0
}

/// rule: deleting the reserve competition runs its destructor and frees the object when asked
///
/// `delete_through` with `ReserveComp::~ReserveComp` (0x7e6150, through its hook).
/// Original: `ReserveComp::scalar_deleting_destructor` (0x7e6130, record_utils.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_reserve_comp(this: usize, flags: u32) -> usize {
    covered!("rx11_11 reserve competition deleted");
    delete_through(this, flags, 0x7e6150)
}

/// rule: deleting the Under 21 international competition runs its destructor and frees the object when asked
///
/// `delete_through` with `Under21IntComp::~Under21IntComp` (0x7e6340, through its hook).
/// Original: `Under21IntComp::scalar_deleting_destructor` (0x7e6320, record_utils.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_under21_int_comp(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7e6340)
}

/// rule: deleting the B international competition runs its destructor and frees the object when asked
///
/// `delete_through` with `BIntComp::~BIntComp` (0x7e6370, through its hook).
/// Original: `BIntComp::scalar_deleting_destructor` (0x7e6350, record_utils.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_b_int_comp(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x7e6370)
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_write_progress_text(this: usize, buf: usize, size: i32, stage: u32, round: u32, status: u32, comp: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, usize) -> i32>(%%I0%%);
    compare_qualifying_progress_text("wc_african_cup_write_progress_text", buf, size, stage, round, status, || o(this, buf, size, stage, round, status, comp),
        |b, n| wc_african_cup_write_progress_text(this, b, n, stage, round, status, comp))
}

/// Comparers of the three deleting destructors: they free the object; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_delete_reserve_comp(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_under21_int_comp(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_b_int_comp(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}
