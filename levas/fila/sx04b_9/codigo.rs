// ---------- sx04b_9: the month award's and a nation's awards' destructors and the team of the week order (month_award.cpp, nation_awards.cpp) ----------

/// `MONTH_AWARD::~MONTH_AWARD` (0x766f40, thiscall), as the deleting destructor calls it (through the hook).
const FN_MONTH_AWARD_DESTRUCT: usize = 0x766f40;
/// The destructor of a block owner (0x58e330, thiscall): frees the block at +0 when there is one (the month ratings'
/// entries).
const FN_FREE_OWNED_BLOCK: usize = 0x58e330;
/// An award's virtual deleting destructor (slot 0: thiscall(flags)).
const AWARD_DELETING_DESTRUCT_SLOT: usize = 0x0;

/// Destroys a month award: its vtable back to `MONTH_AWARD_VTABLE`, then the award base destructor.
/// Original: `MONTH_AWARD::~MONTH_AWARD` (0x766f40, month_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn month_award_destruct(this: usize) {
    write_u32(this, MONTH_AWARD_VTABLE);
    covered!("sx04b_9 month award destroyed");
    award_destruct_base(this);
}

/// Comparer for `month_award_destruct`: side by side over the vtable (the base destructor puts its own back).
pub(crate) unsafe extern "thiscall" fn compare_month_award_destruct(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(this, 4)], || o(this), || month_award_destruct(this));
    SCOREBOARD.record(equal, || format!("month_award_destruct({:#x}) {:?}", this, first_differences(&a, &b)));
}

/// The month award's deleting destructor: `MONTH_AWARD::~MONTH_AWARD`, then the object freed with bit 0 of the flags.
/// Returns this.
/// Original: `MONTH_AWARD::scalar deleting destructor` (0x766f20, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_deleting_destruct(this: usize, flags: u32) -> usize {
    award_deleting_destruct(this, flags, FN_MONTH_AWARD_DESTRUCT)
}

/// Comparer for `month_award_deleting_destruct`: it may free the object; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_month_award_deleting_destruct(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

/// Destroys a nation's awards: the base vtable (`NATION_AWARDS_BASE_VTABLE`) back in place; each award of the table
/// (the count re-read at each step) deleted through its virtual deleting destructor (flags 1); the table freed
/// (`exe::crt_free`); the month ratings, when there are any, their entries freed and the object deleted
/// (`exe::crt_operator_delete`). The table and ratings pointers are left as they were.
/// Original: `NationAwards::~NationAwards` (0x7682b0, nation_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn nation_awards_destruct(this: usize) {
    use nation_awards_fields as na;
    write_u32(this, NATION_AWARDS_BASE_VTABLE);
    let mut k = 0i16;
    while k < read_i16(this + na::COUNT) {
        let award = read_usize(read_usize(this + na::TABLE) + 4 * k as usize);
        if award != 0 {
            let delete: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(award) + AWARD_DELETING_DESTRUCT_SLOT));
            delete(award, 1); // sx04b_9: each award deleted
        }
        k += 1;
    }
    let table = read_usize(this + na::TABLE);
    if table != 0 {
        exe::crt_free(table);
    }
    let ratings = read_usize(this + na::MONTH_RATINGS);
    if ratings != 0 {
        covered!("sx04b_9 nation month ratings deleted");
        let free_entries: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FREE_OWNED_BLOCK);
        free_entries(ratings);
        exe::crt_operator_delete(ratings);
    }
}

/// Comparer for `nation_awards_destruct`: it frees the awards; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

/// The qsort order of the team of the week candidates (11-byte records: person, club, match rating byte at +8,
/// position mask): the higher match rating first (unsigned bytes, b − a).
/// Original: `nation_awards_compare_byte_8_descending` (0x76a460, nation_awards.cpp, cdecl).
pub(crate) unsafe extern "C" fn nation_awards_compare_byte_8_descending(a: *const usize, b: *const usize) -> i32 {
    covered!("sx04b_9 team of the week ordered");
    read_u8(b as usize + 8) as i32 - read_u8(a as usize + 8) as i32 // sx04b_9: the higher rating first
}

compare_order!(compare_nation_awards_compare_byte_8_descending, nation_awards_compare_byte_8_descending, %%I3%%);
